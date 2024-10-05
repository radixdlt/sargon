use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct TXVersion(u64);

impl From<u64> for TXVersion {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl HasSampleValues for TXVersion {
    fn sample() -> Self {
        Self(1)
    }

    fn sample_other() -> Self {
        Self(2)
    }
}
