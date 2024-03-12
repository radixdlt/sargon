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
