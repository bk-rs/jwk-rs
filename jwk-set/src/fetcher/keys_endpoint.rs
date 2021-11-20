use http_api_client_endpoint::{
    http::{header::ACCEPT, Error as HttpError},
    Body, Endpoint, Request, Response, MIME_APPLICATION_JSON,
};
use jsonwebkey::JsonWebKey;
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;

//
#[derive(Debug, Clone, Default)]
pub struct KeysEndpoint<'a> {
    url: &'a str,
}
impl<'a> KeysEndpoint<'a> {
    pub fn new(url: &'a str) -> Self {
        Self { url }
    }
}

impl<'a> Endpoint for KeysEndpoint<'a> {
    type RenderRequestError = KeysEndpointError;

    type ParseResponseOutput = KeysEndpointResponseBody;
    type ParseResponseError = KeysEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let request = Request::builder()
            .uri(self.url)
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .body(vec![])
            .map_err(KeysEndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let body = serde_json::from_slice::<KeysEndpointResponseBody>(response.body())
            .map_err(KeysEndpointError::DeResponseBodyFailed)?;

        Ok(body)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeysEndpointResponseBody {
    pub keys: Vec<JsonWebKey>,
}

#[derive(thiserror::Error, Debug)]
pub enum KeysEndpointError {
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    #[error("DeResponseBodyFailed {0}")]
    DeResponseBodyFailed(SerdeJsonError),
}
