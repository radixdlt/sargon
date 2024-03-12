use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct PerAssetTransfers {
    pub from_account: AccountAddress,
    pub fungible_resources: Vec<PerAssetTransfersOfFungibleResource>,
    pub non_fungible_resources: Vec<PerAssetTransfersOfNonFungibleResource>,
}

impl PerAssetTransfers {
    pub fn new(
        from: AccountAddress,
        fungibles: impl IntoIterator<Item = PerAssetTransfersOfFungibleResource>,
        non_fungibles: impl IntoIterator<
            Item = PerAssetTransfersOfNonFungibleResource,
        >,
    ) -> Self {
        Self {
            from_account: from,
            fungible_resources: fungibles.into_iter().collect_vec(),
            non_fungible_resources: non_fungibles.into_iter().collect_vec(),
        }
    }
}

impl HasSampleValues for PerAssetTransfers {
    fn sample() -> Self {
        Self::new(
            AccountAddress::sample_mainnet(),
            [
                PerAssetTransfersOfFungibleResource::sample_mainnet(),
                PerAssetTransfersOfFungibleResource::sample_mainnet_other(),
            ],
            [
                PerAssetTransfersOfNonFungibleResource::sample_mainnet(),
                PerAssetTransfersOfNonFungibleResource::sample_mainnet_other(),
            ],
        )
    }

    fn sample_other() -> Self {
        Self::new(
            AccountAddress::sample_stokenet(),
            [
                PerAssetTransfersOfFungibleResource::sample_stokenet(),
                PerAssetTransfersOfFungibleResource::sample_stokenet_other(),
            ],
            [
                PerAssetTransfersOfNonFungibleResource::sample_stokenet(),
                PerAssetTransfersOfNonFungibleResource::sample_stokenet_other(),
            ],
        )
    }
}
