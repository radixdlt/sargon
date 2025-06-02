use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PerRecipientAssetTransfers {
    pub address_of_sender: AccountAddress,
    pub transfers: Vec<PerRecipientAssetTransfer>,
}

impl PerRecipientAssetTransfers {
    pub fn new(
        address_of_sender: AccountAddress,
        transfers: impl IntoIterator<Item = PerRecipientAssetTransfer>,
    ) -> Self {
        Self {
            address_of_sender,
            transfers: transfers.into_iter().collect_vec(),
        }
    }
}

#[allow(unused)]
impl PerRecipientAssetTransfers {
    pub(crate) fn sample_mainnet() -> Self {
        Self::new(
            AccountAddress::sample_mainnet(),
            [
                PerRecipientAssetTransfer::sample_mainnet(),
                PerRecipientAssetTransfer::sample_mainnet_other(),
            ],
        )
    }

    pub(crate) fn sample_mainnet_other() -> Self {
        Self::new(
            AccountAddress::sample_mainnet_other(),
            [PerRecipientAssetTransfer::sample_mainnet_other()],
        )
    }

    pub(crate) fn sample_stokenet() -> Self {
        Self::new(
            AccountAddress::sample_stokenet(),
            [
                PerRecipientAssetTransfer::sample_stokenet(),
                PerRecipientAssetTransfer::sample_stokenet_other(),
            ],
        )
    }

    pub(crate) fn sample_stokenet_other() -> Self {
        Self::new(
            AccountAddress::sample_stokenet_other(),
            [PerRecipientAssetTransfer::sample_stokenet_other()],
        )
    }
}

impl HasSampleValues for PerRecipientAssetTransfers {
    fn sample() -> Self {
        Self::new(
            AccountAddress::sample_mainnet(),
            [PerRecipientAssetTransfer::new(
                TransferRecipient::ProfileAccount { value: AccountForDisplay::new(AccountAddress::from_str("account_rdx129akrrsd9ctuphe99lesa8cf6auc5vqwdd2lu0ej6csncnuw9eedgv").unwrap(), DisplayName::sample(), AppearanceID::sample()) },
                [PerRecipientFungibleTransfer::new(
                    ResourceAddress::sample_mainnet_xrd(),
                    Decimal192::from_str("237.13372718281828").unwrap(),
                    true,
                    None,
                )],
                [
                    PerRecipientNonFungibleTransfer::new(NonFungibleResourceAddress::sample_mainnet(), true, [
                        NonFungibleLocalId::sample(),
                        NonFungibleLocalId::sample_other(),
                    ])
                ],
            ),
            PerRecipientAssetTransfer::new(
                TransferRecipient::AddressOfExternalAccount { value: AccountAddress::from_str("account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7").unwrap() }
                ,
                [PerRecipientFungibleTransfer::new(
                    ResourceAddress::sample_mainnet_xrd(),
                    Decimal192::from_str("987654.1234").unwrap(),
                    true,
                    None,
                ),
                PerRecipientFungibleTransfer::new(
                    ResourceAddress::sample_mainnet_candy(),
                    Decimal192::from_str("987654.1234").unwrap(),
                    true,
                    4
                )
                ],
                [
                    PerRecipientNonFungibleTransfer::new(NonFungibleResourceAddress::sample_mainnet(), true, [NonFungibleLocalId::sample_other()]),
                    PerRecipientNonFungibleTransfer::new(NonFungibleResourceAddress::sample_mainnet_other(), true, [NonFungibleLocalId::sample_other()])
                ],
            )
            ],
        )
    }

    fn sample_other() -> Self {
        let sender: AccountAddress = "account_tdx_2_128rkfzdztjpgajucstydar2gz2vp9jj779k33jy3gect2rh5r28rgn".parse().unwrap();
        let recip0: AccountAddress = "account_tdx_2_129e9h6zp5z08qkc0q5tdqz9zc67gg2k7tergrj9erznmke6qeevmsv".parse().unwrap();
        let recip1: AccountAddress = "account_tdx_2_128a45a7hetjfpfqdlsp07eyrmhq7edldefgd7263jd58puzuq09qks".parse().unwrap();

        let nft_c0: NonFungibleResourceAddress = "resource_tdx_2_1n2sjxxtk6vm6pvk8dxr798e8zpxqz50q5wlmldlat0qhh04u2mwmy8".parse().unwrap();
        let nft_c1: NonFungibleResourceAddress = "resource_tdx_2_1ntuaekqexa73m9en04jj3vdt3fk9u9kdk8q9su4efldun2y7nd3cga".parse().unwrap();

        let fung_0: ResourceAddress = ResourceAddress::sample_stokenet_xrd();
        let fung_1: ResourceAddress =
            ResourceAddress::sample_stokenet_gc_tokens();

        Self::new(
            sender,
            [
                PerRecipientAssetTransfer::new(
                    TransferRecipient::AddressOfExternalAccount {
                        value: recip0,
                    },
                    [
                        PerRecipientFungibleTransfer::new(fung_0, 30, true, 18),
                        PerRecipientFungibleTransfer::new(fung_1, 3, true, 18),
                    ],
                    [
                        PerRecipientNonFungibleTransfer::new(
                            nft_c0,
                            true,
                            [
                                NonFungibleLocalId::integer(40),
                                NonFungibleLocalId::integer(48),
                            ],
                        ),
                        PerRecipientNonFungibleTransfer::new(
                            nft_c1,
                            true,
                            [
                                NonFungibleLocalId::integer(21),
                                NonFungibleLocalId::integer(3),
                            ],
                        ),
                    ],
                ),
                PerRecipientAssetTransfer::new(
                    TransferRecipient::AddressOfExternalAccount {
                        value: recip1,
                    },
                    [
                        PerRecipientFungibleTransfer::new(fung_0, 50, true, 18),
                        PerRecipientFungibleTransfer::new(fung_1, 5, true, 18),
                    ],
                    [
                        PerRecipientNonFungibleTransfer::new(
                            nft_c0,
                            true,
                            [
                                NonFungibleLocalId::integer(34),
                                NonFungibleLocalId::integer(22),
                            ],
                        ),
                        PerRecipientNonFungibleTransfer::new(
                            nft_c1,
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
        )
    }
}

impl PerRecipientAssetTransfers {
    /// Transpose: `PerRecipient` -> `PerAsset`
    pub fn transpose(&self) -> PerAssetTransfers {
        let mut per_asset_fungibles = IndexMap::<
            ResourceAddress,
            PerAssetTransfersOfFungibleResource,
        >::new();

        let mut per_asset_non_fungibles = IndexMap::<
            ResourceAddress,
            PerAssetTransfersOfNonFungibleResource,
        >::new();

        self.transfers.clone().into_iter().for_each(|t| {
            let x = t.clone();
            let recipient = &x.recipient;

            x.fungibles.clone().into_iter().for_each(|y| {
                if let Some(existing_transfers) =
                    per_asset_fungibles.get_mut(&y.resource_address)
                {
                    existing_transfers.expanded((recipient, y.clone()));
                } else {
                    per_asset_fungibles.insert(
                        y.resource_address,
                        PerAssetTransfersOfFungibleResource::new(
                            PerAssetFungibleResource::new(
                                y.clone().resource_address,
                                y.clone().divisibility,
                            ),
                            [PerAssetFungibleTransfer::from((
                                recipient,
                                y.clone(),
                            ))],
                        ),
                    );
                }
            });

            x.non_fungibles.clone().into_iter().for_each(|y| {
                if let Some(existing_transfers) =
                    per_asset_non_fungibles.get_mut(&y.resource_address)
                {
                    existing_transfers.expanded((recipient, y.clone()));
                } else {
                    per_asset_non_fungibles.insert(
                        y.resource_address,
                        PerAssetTransfersOfNonFungibleResource::new(
                            y.clone().resource_address,
                            [PerAssetNonFungibleTransfer::from((
                                recipient,
                                y.clone(),
                            ))],
                        ),
                    );
                }
            });
        });

        PerAssetTransfers::new(
            self.address_of_sender,
            per_asset_fungibles.values().cloned(),
            per_asset_non_fungibles.values().cloned(),
        )
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
        let sender: AccountAddress = "account_tdx_2_128rkfzdztjpgajucstydar2gz2vp9jj779k33jy3gect2rh5r28rgn".parse::<AccountAddress>().unwrap();
        let recip0: AccountAddress = "account_tdx_2_129e9h6zp5z08qkc0q5tdqz9zc67gg2k7tergrj9erznmke6qeevmsv".parse::<AccountAddress>().unwrap();
        let recip1: AccountAddress = "account_tdx_2_128a45a7hetjfpfqdlsp07eyrmhq7edldefgd7263jd58puzuq09qks".parse::<AccountAddress>().unwrap();

        let nft_c0 = "resource_tdx_2_1n2sjxxtk6vm6pvk8dxr798e8zpxqz50q5wlmldlat0qhh04u2mwmy8".parse::<NonFungibleResourceAddress>().unwrap();
        let nft_c1 = "resource_tdx_2_1ntuaekqexa73m9en04jj3vdt3fk9u9kdk8q9su4efldun2y7nd3cga".parse::<NonFungibleResourceAddress>().unwrap();

        let fung_0: ResourceAddress = ResourceAddress::sample_stokenet_xrd();
        let fung_1: ResourceAddress =
            ResourceAddress::sample_stokenet_gc_tokens();

        let per_asset_transfers = PerAssetTransfers::new(
            sender,
            [
                PerAssetTransfersOfFungibleResource::new(
                    PerAssetFungibleResource::new(fung_0, 18),
                    [
                        PerAssetFungibleTransfer::new(
                            TransferRecipient::AddressOfExternalAccount {
                                value: recip0,
                            },
                            true,
                            30,
                        ),
                        PerAssetFungibleTransfer::new(
                            TransferRecipient::AddressOfExternalAccount {
                                value: recip1,
                            },
                            true,
                            50,
                        ),
                    ],
                ),
                PerAssetTransfersOfFungibleResource::new(
                    PerAssetFungibleResource::new(fung_1, 18),
                    [
                        PerAssetFungibleTransfer::new(
                            TransferRecipient::AddressOfExternalAccount {
                                value: recip0,
                            },
                            true,
                            3,
                        ),
                        PerAssetFungibleTransfer::new(
                            TransferRecipient::AddressOfExternalAccount {
                                value: recip1,
                            },
                            true,
                            5,
                        ),
                    ],
                ),
            ],
            [
                PerAssetTransfersOfNonFungibleResource::new(
                    nft_c0,
                    [
                        PerAssetNonFungibleTransfer::new(
                            TransferRecipient::AddressOfExternalAccount {
                                value: recip0,
                            },
                            true,
                            [
                                NonFungibleLocalId::integer(40),
                                NonFungibleLocalId::integer(48),
                            ],
                        ),
                        PerAssetNonFungibleTransfer::new(
                            TransferRecipient::AddressOfExternalAccount {
                                value: recip1,
                            },
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
                            TransferRecipient::AddressOfExternalAccount {
                                value: recip0,
                            },
                            true,
                            [
                                NonFungibleLocalId::integer(21),
                                NonFungibleLocalId::integer(3),
                            ],
                        ),
                        PerAssetNonFungibleTransfer::new(
                            TransferRecipient::AddressOfExternalAccount {
                                value: recip1,
                            },
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
