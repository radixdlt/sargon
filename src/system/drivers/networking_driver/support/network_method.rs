use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, uniffi::Enum)]
pub enum NetworkMethod {
    Post,
    Get,
}

impl HasSampleValues for NetworkMethod {
    fn sample() -> Self {
        NetworkMethod::Post
    }
    fn sample_other() -> Self {
        NetworkMethod::Get
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NetworkMethod;

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
