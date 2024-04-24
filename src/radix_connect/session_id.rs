use crate::prelude::*;

uniffi::custom_newtype!(SessionID, String);

#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Ord, PartialOrd, Hash,
)]
pub struct SessionID(pub String);

impl SessionID {
    pub fn new(id: impl AsRef<str>) -> Self {
        Self(id.as_ref().to_owned())
    }
}

impl HasSampleValues for SessionID {
    fn sample() -> Self {
        Self::new("session_id1")
    }

    fn sample_other() -> Self {
        Self::new("session_id2")
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

    #[test]
    fn test_new() {
        assert_eq!(SUT::new("session_id1"), SUT::sample());
    }
}
