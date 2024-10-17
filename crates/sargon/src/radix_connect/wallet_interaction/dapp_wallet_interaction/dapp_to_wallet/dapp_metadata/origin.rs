use crate::prelude::*;

/// The origin of a dapp.
#[derive(
    Clone,
    PartialEq,
    Eq,
    Debug,
    derive_more::FromStr,
    derive_more::Display,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct DappOrigin(pub String);

impl DappOrigin {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl From<&str> for DappOrigin {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl TryInto<Url> for DappOrigin {
    type Error = CommonError;

    fn try_into(self) -> std::result::Result<Url, Self::Error> {
        Url::parse(self.0.as_str()).map_err(|_| {
            CommonError::InvalidURL { bad_value: self.0 }
        })
    }
}

impl HasSampleValues for DappOrigin {
    fn sample() -> Self {
        Self::new("https://example.com")
    }

    fn sample_other() -> Self {
        Self::new("https://example.com/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappOrigin;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
