use std::{error, fmt};

use jsonwebkey::Algorithm;
use jsonwebtoken::{self as jwt, errors::Error as JwtError, Header as JwtHeader};
use serde::de::DeserializeOwned;

use crate::JsonWebKeySet;

pub trait DecryptExt {
    fn decrypt<JC>(
        &self,
        token: impl AsRef<str>,
        skip_validate_exp: impl Into<Option<bool>>,
        algorithms_supported: impl Into<Option<Vec<Algorithm>>>,
    ) -> Result<(JwtHeader, JC), DecryptError>
    where
        JC: DeserializeOwned;
}

#[derive(Debug)]
pub enum DecryptError {
    DecodeHeaderFailed(JwtError),
    KidMissing,
    KidNotFound,
    DecodeFailed(JwtError),
}
impl fmt::Display for DecryptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl error::Error for DecryptError {}

//
//
//
impl DecryptExt for JsonWebKeySet {
    fn decrypt<JC>(
        &self,
        token: impl AsRef<str>,
        skip_validate_exp: impl Into<Option<bool>>,
        algorithms_supported: impl Into<Option<Vec<Algorithm>>>,
    ) -> Result<(JwtHeader, JC), DecryptError>
    where
        JC: DeserializeOwned,
    {
        let token = token.as_ref();

        let jwt_header = jwt::decode_header(token).map_err(DecryptError::DecodeHeaderFailed)?;

        let kid = jwt_header.kid.ok_or_else(|| DecryptError::KidMissing)?;
        let jwt_alg = jwt_header.alg;

        let jwk = self
            .keys()
            .iter()
            .find(|jwk| {
                jwk.key_id == Some(kid.to_owned()) && jwk.algorithm.map(Into::into) == Some(jwt_alg)
            })
            .or(self
                .keys()
                .iter()
                .find(|jwk| jwk.key_id == Some(kid.to_owned())))
            .ok_or_else(|| DecryptError::KidNotFound)?;

        let jwt_key = jwk.key.to_decoding_key();

        let mut jwt_validation = jwt::Validation::default();

        if let Some(skip_validate_exp) = skip_validate_exp.into() {
            if skip_validate_exp {
                jwt_validation.validate_exp = false;
            }
        }

        if let Some(jwk_alg) = jwk.algorithm {
            jwt_validation.algorithms = vec![jwk_alg.into()];
        }
        if let Some(algorithms_supported) = algorithms_supported.into() {
            jwt_validation.algorithms = algorithms_supported.into_iter().map(Into::into).collect()
        }

        let jwt::TokenData { header, claims } =
            jwt::decode(token, &jwt_key, &jwt_validation).map_err(DecryptError::DecodeFailed)?;

        Ok((header, claims))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use jsonwebkey::JsonWebKey;
    use serde::Deserialize;
    use serde_json::{Map, Value};

    #[derive(Deserialize, Debug, Clone)]
    struct OidcKeysJson {
        keys: Vec<JsonWebKey>,
    }

    #[test]
    fn test_decrypt_with_apple() {
        let set = JsonWebKeySet::new(
            serde_json::from_str::<OidcKeysJson>(include_str!(
                "../tests/oidc_keys_json_files/apple.json"
            ))
            .unwrap()
            .keys,
        );

        let id_token = include_str!("../tests/oidc_id_token_files/apple.txt");

        let (header, claims): (_, Map<String, Value>) = set.decrypt(id_token, true, None).unwrap();

        assert_eq!(header.kid, Some("eXaunmL".to_owned()));
        assert_eq!(header.alg, jwt::Algorithm::RS256);

        assert_eq!(
            claims.get("iss").unwrap().as_str().unwrap(),
            "https://appleid.apple.com"
        );
    }

    #[test]
    fn test_decrypt_with_microsoft() {
        let set = JsonWebKeySet::new(
            serde_json::from_str::<OidcKeysJson>(include_str!(
                "../tests/oidc_keys_json_files/microsoft.json"
            ))
            .unwrap()
            .keys,
        );

        let id_token = include_str!("../tests/oidc_id_token_files/microsoft.txt");

        let (header, claims): (_, Map<String, Value>) =
            set.decrypt(id_token, true, vec![Algorithm::RS256]).unwrap();

        assert_eq!(header.kid, Some("bW8ZcMjBCnJZS-ibX5UQDNStvx4".to_owned()));
        assert_eq!(header.alg, jwt::Algorithm::RS256);

        assert_eq!(
            claims.get("iss").unwrap().as_str().unwrap(),
            "https://login.microsoftonline.com/9188040d-6c67-4c5b-b112-36a304b66dad/v2.0"
        );
    }
}
