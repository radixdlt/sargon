use crate::prelude::*;
use sargon::PerAssetTransfers as InternalPerAssetTransfers;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct PerAssetTransfers {
    pub from_account: AccountAddress,
    pub fungible_resources: Vec<PerAssetTransfersOfFungibleResource>,
    pub non_fungible_resources: Vec<PerAssetTransfersOfNonFungibleResource>,
}

#[uniffi::export]
pub fn new_per_asset_transfers_sample() -> PerAssetTransfers {
    InternalPerAssetTransfers::sample().into()
}

#[uniffi::export]
pub fn new_per_asset_transfers_sample_other() -> PerAssetTransfers {
    InternalPerAssetTransfers::sample_other().into()
}
