use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct PerAssetTransfers {
    pub from_account: AccountAddress,
    pub fungible_resources: Vec<PerAssetTransfersOfFungibleResource>,
    pub non_fungible_resources: Vec<PerAssetTransfersOfNonFungibleResource>,
}

#[uniffi::export]
pub fn new_per_asset_transfers_sample() -> PerAssetTransfers {
    PerAssetTransfers::sample()
}

#[uniffi::export]
pub fn new_per_asset_transfers_sample_other() -> PerAssetTransfers {
    PerAssetTransfers::sample_other()
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PerAssetTransfers;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_per_asset_transfers_sample(),
                new_per_asset_transfers_sample_other(),
                // duplicates should get removed
                new_per_asset_transfers_sample(),
                new_per_asset_transfers_sample_other(),
            ])
            .len(),
            2
        );
    }
}

