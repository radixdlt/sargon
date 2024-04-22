use crate::prelude::*;

uniffi::custom_newtype!(WalletInteractionId, String);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WalletInteractionId(pub String);

impl WalletInteractionId {
    pub fn new(id: impl AsRef<str>) -> Self {
        Self(id.as_ref().to_owned())
    }
}

impl HasSampleValues for WalletInteractionId {
    fn sample() -> Self {
        Self::new("sample")
    }

    fn sample_other() -> Self {
        Self::new("sample_other")
    }
}
