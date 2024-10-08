use crate::prelude::*;
use sargon::PerRecipientAssetTransfers as InternalPerRecipientAssetTransfers;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct PerRecipientAssetTransfers {
    pub address_of_sender: AccountAddress,
    pub transfers: Vec<PerRecipientAssetTransfer>,
}

impl From<InternalPerRecipientAssetTransfers> for PerRecipientAssetTransfers {
    fn from(per_recipient_asset_transfers: InternalPerRecipientAssetTransfers) -> Self {
        Self {
            address_of_sender: per_recipient_asset_transfers.address_of_sender.into(),
            transfers: per_recipient_asset_transfers
                .transfers
                .into_vec()
        }
    }
}

impl Into<InternalPerRecipientAssetTransfers> for PerRecipientAssetTransfers {
    fn into(self) -> InternalPerRecipientAssetTransfers {
        InternalPerRecipientAssetTransfers {
            address_of_sender: self.address_of_sender.into(),
            transfers: self.transfers.into_internal_vec(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PerRecipientAssetTransfers;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn hash() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                SUT::sample_mainnet(),
                SUT::sample_mainnet_other(),
                SUT::sample_stokenet(),
                SUT::sample_stokenet_other(),
                // duplicates should be removed
                SUT::sample_mainnet(),
                SUT::sample_mainnet_other(),
                SUT::sample_stokenet(),
                SUT::sample_stokenet_other(),
            ])
            .len(),
            4
        )
    }

    #[test]
    fn transpose_simple() {
        pretty_assertions::assert_eq!(
            SUT::sample().transpose(),
            PerAssetTransfers::sample()
        )
    }

    #[test]
    fn transpose_complex() {
        let sender: AccountAddress = "account_tdx_2_128rkfzdztjpgajucstydar2gz2vp9jj779k33jy3gect2rh5r28rgn".into();
        let recip0: AccountAddress = "account_tdx_2_129e9h6zp5z08qkc0q5tdqz9zc67gg2k7tergrj9erznmke6qeevmsv".into();
        let recip1: AccountAddress = "account_tdx_2_128a45a7hetjfpfqdlsp07eyrmhq7edldefgd7263jd58puzuq09qks".into();

        let nft_c0: NonFungibleResourceAddress = "resource_tdx_2_1n2sjxxtk6vm6pvk8dxr798e8zpxqz50q5wlmldlat0qhh04u2mwmy8".into();
        let nft_c1: NonFungibleResourceAddress = "resource_tdx_2_1ntuaekqexa73m9en04jj3vdt3fk9u9kdk8q9su4efldun2y7nd3cga".into();

        let fung_0: ResourceAddress = ResourceAddress::sample_stokenet_xrd();
        let fung_1: ResourceAddress =
            ResourceAddress::sample_stokenet_gc_tokens();

        let per_asset_transfers = PerAssetTransfers::new(
            sender,
            [
                PerAssetTransfersOfFungibleResource::new(
                    PerAssetFungibleResource::new(fung_0, 18),
                    [
                        PerAssetFungibleTransfer::new(recip0, true, 30),
                        PerAssetFungibleTransfer::new(recip1, true, 50),
                    ],
                ),
                PerAssetTransfersOfFungibleResource::new(
                    PerAssetFungibleResource::new(fung_1, 18),
                    [
                        PerAssetFungibleTransfer::new(recip0, true, 3),
                        PerAssetFungibleTransfer::new(recip1, true, 5),
                    ],
                ),
            ],
            [
                PerAssetTransfersOfNonFungibleResource::new(
                    nft_c0,
                    [
                        PerAssetNonFungibleTransfer::new(
                            recip0,
                            true,
                            [
                                NonFungibleLocalId::integer(40),
                                NonFungibleLocalId::integer(48),
                            ],
                        ),
                        PerAssetNonFungibleTransfer::new(
                            recip1,
                            true,
                            [
                                NonFungibleLocalId::integer(34),
                                NonFungibleLocalId::integer(22),
                            ],
                        ),
                    ],
                ),
                PerAssetTransfersOfNonFungibleResource::new(
                    nft_c1,
                    [
                        PerAssetNonFungibleTransfer::new(
                            recip0,
                            true,
                            [
                                NonFungibleLocalId::integer(21),
                                NonFungibleLocalId::integer(3),
                            ],
                        ),
                        PerAssetNonFungibleTransfer::new(
                            recip1,
                            true,
                            [
                                NonFungibleLocalId::integer(15),
                                NonFungibleLocalId::integer(9),
                                NonFungibleLocalId::integer(13),
                            ],
                        ),
                    ],
                ),
            ],
        );

        assert_eq!(SUT::sample_other().transpose(), per_asset_transfers);
    }
}
