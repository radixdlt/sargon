use crate::prelude::*;
use sargon::UnstakeData as InternalUnstakeData;

/// The data associated with the various validator claim NFTs
#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct UnstakeData {
    pub name: String,

    /// An epoch number at (or after) which the pending unstaked XRD may be claimed.
    pub claim_epoch: Epoch,

    /// An XRD amount to be claimed.
    pub claim_amount: Decimal192,
}

impl From<InternalUnstakeData> for UnstakeData {
    fn from(value: InternalUnstakeData) -> Self {
        Self {
            name: value.name,
            claim_epoch: value.claim_epoch.into(),
            claim_amount: value.claim_amount.into(),
        }
    }
}

impl Into<InternalUnstakeData> for UnstakeData {
    fn into(self) -> InternalUnstakeData {
        InternalUnstakeData {
            name: self.name,
            claim_epoch: self.claim_epoch.into(),
            claim_amount: self.claim_amount.into(),
        }
    }
}
