use crate::prelude::*;

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, derive_more::Display,
)]
pub struct SessionID(pub String);

impl SessionID {
    pub fn new(session_id: impl AsRef<str>) -> Self {
        Self(session_id.as_ref().to_owned())
    }
}

impl HasSampleValues for SessionID {
    fn sample() -> Self {
        Self::new("sample")
    }

    fn sample_other() -> Self {
        Self::new("sample_other")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SessionID;

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
