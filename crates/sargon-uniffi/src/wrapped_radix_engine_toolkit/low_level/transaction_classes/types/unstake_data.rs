use crate::prelude::*;
use sargon::UnstakeData as InternalUnstakeData;

/// The data associated with the various validator claim NFTs
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct UnstakeData {
    pub name: String,

    /// An epoch number at (or after) which the pending unstaked XRD may be claimed.
    pub claim_epoch: Epoch,

    /// An XRD amount to be claimed.
    pub claim_amount: Decimal192,
}
