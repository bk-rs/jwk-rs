//! https://datatracker.ietf.org/doc/html/rfc7517#section-5

pub use jsonwebkey;

use jsonwebkey::JsonWebKey;
use serde::{Deserialize, Serialize};

#[cfg(feature = "with-decrypt")]
pub mod decrypt;

//
//
//
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct JsonWebKeySet {
    pub keys: Vec<JsonWebKey>,
}

#[cfg(feature = "serde_json")]
impl std::str::FromStr for JsonWebKeySet {
    type Err = serde_json::Error;
    fn from_str(json: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_with_apple_oidc() {
        let jwk_set = serde_json::from_str::<JsonWebKeySet>(include_str!(
            "../tests/oidc_keys_json_files/apple.json"
        ))
        .unwrap();
        assert_eq!(jwk_set.keys.len(), 3);
    }

    #[test]
    fn test_de_with_gitlab_oidc() {
        let jwk_set = serde_json::from_str::<JsonWebKeySet>(include_str!(
            "../tests/oidc_keys_json_files/gitlab.json"
        ))
        .unwrap();
        assert_eq!(jwk_set.keys.len(), 2);
    }

    #[test]
    fn test_de_with_google_oidc() {
        let jwk_set = serde_json::from_str::<JsonWebKeySet>(include_str!(
            "../tests/oidc_keys_json_files/google.json"
        ))
        .unwrap();
        assert_eq!(jwk_set.keys.len(), 3);
    }

    #[test]
    fn test_de_with_microsoft_oidc() {
        let jwk_set = serde_json::from_str::<JsonWebKeySet>(include_str!(
            "../tests/oidc_keys_json_files/microsoft.json"
        ))
        .unwrap();
        assert_eq!(jwk_set.keys.len(), 7);
    }

    #[test]
    fn test_de_with_yahoo_oidc() {
        let jwk_set = serde_json::from_str::<JsonWebKeySet>(include_str!(
            "../tests/oidc_keys_json_files/yahoo.json"
        ))
        .unwrap();
        assert_eq!(jwk_set.keys.len(), 3);
    }
}
