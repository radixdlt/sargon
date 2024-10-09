use crate::prelude::*;
use sargon::PerAssetTransfers as InternalPerAssetTransfers;

#[derive(Clone, Debug, PartialEq, Eq, Hash,  uniffi::Record)]
pub struct PerAssetTransfers {
    pub from_account: AccountAddress,
    pub fungible_resources: Vec<PerAssetTransfersOfFungibleResource>,
    pub non_fungible_resources: Vec<PerAssetTransfersOfNonFungibleResource>,
}

impl From<InternalPerAssetTransfers> for PerAssetTransfers {
    fn from(value: InternalPerAssetTransfers) -> Self {
        Self {
            from_account: value.from_account.into(),
            fungible_resources: value.fungible_resources.into_vec(),
            non_fungible_resources: value.non_fungible_resources.into_vec(),
        }
    }
}

impl Into<InternalPerAssetTransfers> for PerAssetTransfers {
    fn into(self) -> InternalPerAssetTransfers {
        InternalPerAssetTransfers {
            from_account: self.from_account.into(),
            fungible_resources: self.fungible_resources.into_internal_vec(),
            non_fungible_resources: self.non_fungible_resources.into_internal_vec(),
        }
    }
}

#[uniffi::export]
pub fn new_per_asset_transfers_sample() -> PerAssetTransfers {
    InternalPerAssetTransfers::sample().into()
}

#[uniffi::export]
pub fn new_per_asset_transfers_sample_other() -> PerAssetTransfers {
    InternalPerAssetTransfers::sample_other().into()
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

