use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct NetworkRequest {
    pub url: Url,
    pub method: NetworkMethod,
    pub headers: HashMap<String, String>,

    pub body: BagOfBytes,
}

impl NetworkRequest {
    pub fn new_post(url: Url) -> Self {
        Self {
            url,
            method: NetworkMethod::Post,
            headers: HashMap::new(),
            body: BagOfBytes::new(),
        }
    }

    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers.extend(headers);
        self
    }

    pub fn with_body(mut self, body: impl Into<BagOfBytes>) -> Self {
        self.body = body.into();
        self
    }

    pub fn with_serializing_body<T: Serialize>(self, body: T) -> Result<Self> {
        let serialized = serde_json::to_vec(&body)
            .map_err(|_| CommonError::FailedToSerializeToJSON)?;

        Ok(self.with_body(serialized))
    }
}
