//! https://datatracker.ietf.org/doc/html/rfc7517#section-5

pub use jsonwebkey;

use jsonwebkey::JsonWebKey;
use serde::{Deserialize, Serialize};

#[cfg(feature = "with-decrypt")]
pub mod decrypt;
#[cfg(feature = "with-fetcher")]
pub mod fetcher;

//
//
//
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct JsonWebKeySet {
    pub keys: Vec<JsonWebKey>,
}
impl JsonWebKeySet {
    pub fn new(keys: Vec<JsonWebKey>) -> Self {
        Self { keys }
    }

    pub fn keys(&self) -> &[JsonWebKey] {
        &self.keys
    }
}
