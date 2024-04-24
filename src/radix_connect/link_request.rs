use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, uniffi::Record)]
pub struct LinkRequest {
    pub origin: Url,
    pub session_id: SessionID,
}

impl HasSampleValues for LinkRequest {
    fn sample() -> Self {
        Self {
            origin: Url::parse("radix://app").unwrap(),
            session_id: SessionID("123".to_owned()),
        }
    }

    fn sample_other() -> Self {
        Self {
            origin: Url::parse("radix://app").unwrap(),
            session_id: SessionID("456".to_owned()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = LinkRequest;

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
