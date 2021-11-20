use std::error;

use http_api_client::{Client, ClientRespondEndpointError};
use http_api_client_endpoint::{Body, Request};

use crate::JsonWebKeySet;

pub mod keys_endpoint;

pub use keys_endpoint::{KeysEndpoint, KeysEndpointError, KeysEndpointResponseBody};

//
//
//
#[derive(Debug, Clone)]
pub struct Fetcher<C> {
    client: C,
}
impl<C> Fetcher<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }
}
impl<C> Fetcher<C>
where
    C: Client + Send + Sync,
{
    pub async fn fetch<PreRCB>(
        &self,
        url: impl AsRef<str>,
        pre_request_callback: PreRCB,
    ) -> Result<FetcherFetchOutput, FetcherFetchError>
    where
        PreRCB: FnMut(Request<Body>) -> Request<Body> + Send,
    {
        let url = url.as_ref();
        let endpoint = KeysEndpoint::new(url);

        let body = self
            .client
            .respond_endpoint_with_callback(&endpoint, pre_request_callback, |_| {})
            .await
            .map_err(|err| match err {
                ClientRespondEndpointError::RespondFailed(err) => {
                    FetcherFetchError::KeysEndpointRespondFailed(Box::new(err))
                }
                ClientRespondEndpointError::EndpointRenderRequestFailed(err) => {
                    FetcherFetchError::KeysEndpointError(err)
                }
                ClientRespondEndpointError::EndpointParseResponseFailed(err) => {
                    FetcherFetchError::KeysEndpointError(err)
                }
            })?;

        Ok(FetcherFetchOutput { set: body.into() })
    }
}

#[derive(Debug, Clone)]
pub struct FetcherFetchOutput {
    pub set: JsonWebKeySet,
}

#[derive(thiserror::Error, Debug)]
pub enum FetcherFetchError {
    #[error("KeysEndpointError {0}")]
    KeysEndpointError(KeysEndpointError),
    #[error("KeysEndpointRespondFailed {0}")]
    KeysEndpointRespondFailed(Box<dyn error::Error + Send + Sync>),
}

//
//
//
impl From<KeysEndpointResponseBody> for JsonWebKeySet {
    fn from(body: KeysEndpointResponseBody) -> Self {
        Self::new(body.keys)
    }
}
