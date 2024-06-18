use crate::prelude::*;

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum SessionOrigin {
    WebDapp(DappOrigin),
}

impl HasSampleValues for SessionOrigin {
    fn sample() -> Self {
        Self::WebDapp(DappOrigin::sample())
    }

    fn sample_other() -> Self {
        Self::WebDapp(DappOrigin::sample_other())
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
