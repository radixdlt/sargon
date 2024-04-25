use crate::prelude::*;

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, derive_more::Display,
)]
pub struct SessionID(String);

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
