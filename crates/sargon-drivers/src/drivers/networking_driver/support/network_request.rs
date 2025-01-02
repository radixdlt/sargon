use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkRequest {
    pub url: Url,
    pub method: NetworkMethod,
    pub headers: HashMap<String, String>,

    pub body: BagOfBytes,
}

impl NetworkRequest {
    pub fn new(
        url: impl Into<Url>,
        method: impl Into<NetworkMethod>,
        headers: impl Into<HashMap<String, String>>,
        body: impl Into<BagOfBytes>,
    ) -> Self {
        Self {
            url: url.into(),
            method: method.into(),
            headers: headers.into(),
            body: body.into(),
        }
    }

    pub fn new_post(url: Url) -> Self {
        Self::new(url, NetworkMethod::Post, [], vec![])
    }

    pub fn new_get(url: Url) -> Self {
        Self::new(url, NetworkMethod::Get, [], vec![])
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

impl HasSampleValues for NetworkRequest {
    fn sample() -> Self {
        Self::new_post(Url::parse("https://example.com").unwrap())
    }

    fn sample_other() -> Self {
        Self::new_post(Url::parse("https://example.org").unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NetworkRequest;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn with_headers() {
        let mut headers = HashMap::new();
        headers.insert("key".to_owned(), "value".to_owned());

        let sut = SUT::sample().with_headers(headers.clone());

        assert_eq!(sut.headers, headers);
    }

    #[test]
    fn with_body() {
        let body = BagOfBytes::new();
        let sut = SUT::sample().with_body(body.clone());

        assert_eq!(sut.body, body);
    }

    #[test]
    fn with_serializing_body() {
        #[derive(Serialize)]
        struct Body {
            key: String,
        }

        let body = Body {
            key: "value".to_owned(),
        };

        let serialized = serde_json::to_vec(&body).unwrap();

        let sut = SUT::sample().with_serializing_body(body).unwrap();

        assert_eq!(sut.body, serialized.into());
    }
}
