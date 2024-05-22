use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct NetworkResponse {
    pub status_code: u16,

    /// Can be empty.
    pub body: BagOfBytes,
}

impl NetworkResponse {
    pub fn new(status_code: u16, body: impl Into<BagOfBytes>) -> Self {
        Self {
            status_code,
            body: body.into(),
        }
    }
}

impl HasSampleValues for NetworkResponse {
    fn sample() -> Self {
        Self::new(200, BagOfBytes::sample())
    }

    fn sample_other() -> Self {
        Self::new(404, BagOfBytes::sample())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NetworkResponse;

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
