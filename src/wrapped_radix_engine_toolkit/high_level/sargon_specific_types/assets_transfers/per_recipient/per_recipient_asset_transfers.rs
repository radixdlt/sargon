use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
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
        Self::sample_mainnet()
    }

    fn sample_other() -> Self {
        Self::sample_stokenet_other()
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
                        y.resource_address.clone(),
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
                        y.resource_address.clone(),
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
            self.address_of_sender.clone(),
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

    // #[test]
    // fn transpose() {
    //     pretty_assertions::assert_eq!(SUT::sample().transpose(), PerAssetTransfers::sample())
    // }
}
