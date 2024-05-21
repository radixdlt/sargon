use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct NetworkRequest {
    pub url: Url,
    pub method: NetworkMethod,
    pub headers: HashMap<String, String>,

    pub body: BagOfBytes,
}

impl NetworkRequest {
    pub fn with_gateway_api_headers(self) -> Self {
        let headers = HashMap::<String, String>::from_iter([
            ("content-Type".to_owned(), "application/json".to_owned()),
            ("accept".to_owned(), "application/json".to_owned()),
            ("user-agent".to_owned(), "Sargon".to_owned()), // https://stackoverflow.com/a/77866494/1311272
            ("RDX-Client-Name".to_owned(), "Sargon".to_owned()),
            ("RDX-Client-Version".to_owned(), "1.5.1".to_owned()),
        ]);

        self.with_headers(headers)
    }
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
