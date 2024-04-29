use crate::prelude::*;

#[derive(PartialEq, Debug)]
pub enum SessionOrigin {
    WebDapp(Url),
}

impl HasSampleValues for SessionOrigin {
    fn sample() -> Self {
        Self::WebDapp(Url::from_str("https://example.com").unwrap())
    }

    fn sample_other() -> Self {
        Self::WebDapp(Url::from_str("https://example.org").unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SessionOrigin;

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
