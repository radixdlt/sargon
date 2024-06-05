use super::session_id::SessionID;
use super::session_origin::SessionOrigin;
use crate::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Session {
    pub id: SessionID,
    pub origin: SessionOrigin,
    pub encryption_key: Exactly32Bytes,
    pub callback_path: String, // TBA: possibly will contain also the callback path
}

impl Session {
    pub fn new(
        id: impl Into<SessionID>,
        origin: SessionOrigin,
        encryption_key: impl Into<Exactly32Bytes>,
        callback_path: String,
    ) -> Self {
        Self {
            id: id.into(),
            origin,
            encryption_key: encryption_key.into(),
            callback_path,
        }
    }
}

impl HasSampleValues for Session {
    fn sample() -> Self {
        Self::new(
            SessionID::sample(),
            SessionOrigin::sample(),
            Exactly32Bytes::sample(),
            String::from("callback_path"),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            SessionID::sample_other(),
            SessionOrigin::sample_other(),
            Exactly32Bytes::sample_other(),
            String::from("callback_path_other"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Session;

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
