use crate::prelude::*;

/// The data associated with the various validator claim NFTs
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnstakeData {
    pub name: String,

    /// An epoch number at (or after) which the pending unstaked XRD may be claimed.
    pub claim_epoch: Epoch,

    /// An XRD amount to be claimed.
    pub claim_amount: Decimal192,
}

impl UnstakeData {
    pub fn new(
        name: impl AsRef<str>,
        claim_epoch: impl Into<Epoch>,
        claim_amount: impl Into<Decimal192>,
    ) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            claim_epoch: claim_epoch.into(),
            claim_amount: claim_amount.into(),
        }
    }
}

impl From<ScryptoUnstakeData> for UnstakeData {
    fn from(value: ScryptoUnstakeData) -> Self {
        Self {
            name: value.name,
            claim_epoch: value.claim_epoch.into(),
            claim_amount: value.claim_amount.into(),
        }
    }
}

impl HasSampleValues for UnstakeData {
    fn sample() -> Self {
        Self::new(String::sample(), Epoch::sample(), Decimal192::sample())
    }

    fn sample_other() -> Self {
        Self::new(
            String::sample_other(),
            Epoch::sample_other(),
            Decimal192::sample_other(),
        )
    }
}
