use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenPriceService {
    pub base_url: Url,
}

impl TokenPriceService {
    pub const SCOPED_TOKENS_PATH: &str = "/price/tokens";
    pub const PRODUCTION_BASE_URL: &str =
        "https://token-price-service.radixdlt.com";

    pub fn new(base_url: Url) -> Self {
        Self { base_url }
    }

    pub fn production() -> Self {
        Self::new(Url::parse(Self::PRODUCTION_BASE_URL).expect("valid URL"))
    }

    pub fn scoped_tokens_url(&self) -> Result<Url> {
        self.base_url.join(Self::SCOPED_TOKENS_PATH).map_err(|_| {
            CommonError::InvalidURL {
                bad_value: self.base_url.to_string(),
            }
        })
    }
}

impl Identifiable for TokenPriceService {
    type ID = Url;

    fn id(&self) -> Self::ID {
        self.base_url.clone()
    }
}

impl HasSampleValues for TokenPriceService {
    fn sample() -> Self {
        Self::production()
    }

    fn sample_other() -> Self {
        Self::new(
            Url::parse("https://token-price-service-alt.example").unwrap(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TokenPriceService;

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
    fn identifiable() {
        let sut = SUT::sample();
        assert_eq!(sut.id(), sut.base_url);
    }

    #[test]
    fn scoped_tokens_url_is_derived_from_base_url() {
        let sut = SUT::sample();
        assert_eq!(
            sut.scoped_tokens_url().unwrap().as_str(),
            "https://token-price-service.radixdlt.com/price/tokens"
        );
    }

    #[test]
    fn json_roundtrip() {
        let sut = SUT::sample();
        assert_json_roundtrip(&sut);
    }
}
