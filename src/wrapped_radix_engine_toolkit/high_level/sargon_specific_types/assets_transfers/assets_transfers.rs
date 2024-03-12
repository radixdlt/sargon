use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct AssetsTransfers {
    pub from_account: AccountAddress,
    pub fungible_resources: Vec<TransfersOfFungibleResource>,
    pub non_fungible_resources: Vec<TransfersOfNonFungibleResource>,
}

impl AssetsTransfers {
    pub fn new(
        from: AccountAddress,
        fungibles: impl IntoIterator<Item = TransfersOfFungibleResource>,
        non_fungibles: impl IntoIterator<Item = TransfersOfNonFungibleResource>,
    ) -> Self {
        Self {
            from_account: from,
            fungible_resources: fungibles.into_iter().collect_vec(),
            non_fungible_resources: non_fungibles.into_iter().collect_vec(),
        }
    }
}

impl HasSampleValues for AssetsTransfers {
    fn sample() -> Self {
        Self::new(
            AccountAddress::sample_mainnet(),
            [
                TransfersOfFungibleResource::sample_mainnet(),
                TransfersOfFungibleResource::sample_mainnet_other(),
            ],
            [
                TransfersOfNonFungibleResource::sample_mainnet(),
                TransfersOfNonFungibleResource::sample_mainnet_other(),
            ],
        )
    }

    fn sample_other() -> Self {
        Self::new(
            AccountAddress::sample_stokenet(),
            [
                TransfersOfFungibleResource::sample_stokenet(),
                TransfersOfFungibleResource::sample_stokenet_other(),
            ],
            [
                TransfersOfNonFungibleResource::sample_stokenet(),
                TransfersOfNonFungibleResource::sample_stokenet_other(),
            ],
        )
    }
}
