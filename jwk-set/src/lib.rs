//! https://datatracker.ietf.org/doc/html/rfc7517#section-5

use jsonwebkey::JsonWebKey;

#[cfg(feature = "with-decrypt")]
pub mod decrypt;
#[cfg(feature = "with-fetcher")]
pub mod fetcher;

//
//
//
#[derive(Debug, Clone)]
pub struct JsonWebKeySet {
    inner: Vec<JsonWebKey>,
}
impl JsonWebKeySet {
    pub fn new(keys: Vec<JsonWebKey>) -> Self {
        Self { inner: keys }
    }

    pub fn keys(&self) -> &[JsonWebKey] {
        &self.inner
    }
}
