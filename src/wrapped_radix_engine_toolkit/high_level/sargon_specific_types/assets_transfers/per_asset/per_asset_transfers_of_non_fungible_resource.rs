use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct PerAssetTransfersOfNonFungibleResource {
    pub resource: ResourceAddress,
    pub transfers: Vec<PerAssetNonFungibleTransfer>,
}

impl PerAssetTransfersOfNonFungibleResource {
    pub fn new(
        resource: impl Into<ResourceAddress>,
        transfers: impl IntoIterator<Item = PerAssetNonFungibleTransfer>,
    ) -> Self {
        Self {
            resource: resource.into(),
            transfers: transfers.into_iter().collect_vec(),
        }
    }
}

impl PerAssetTransfersOfNonFungibleResource {
    pub(crate) fn expanded(
        &mut self,
        transfer: impl Into<PerAssetNonFungibleTransfer>,
    ) {
        self.transfers.push(transfer.into());
    }
}

impl From<(&AssetsTransfersRecipient, PerRecipientNonFungiblesTransfer)>
    for PerAssetTransfersOfNonFungibleResource
{
    fn from(
        value: (&AssetsTransfersRecipient, PerRecipientNonFungiblesTransfer),
    ) -> Self {
        let (recipient, non_fungibles) = value;

        PerAssetTransfersOfNonFungibleResource::new(
            non_fungibles.resource_address.clone(),
            [PerAssetNonFungibleTransfer::new(
                recipient.clone(),
                non_fungibles.use_try_deposit_or_abort,
                non_fungibles.local_ids,
            )],
        )
    }
}

impl PerAssetTransfersOfNonFungibleResource {
    pub fn all_ids(&self) -> Vec<ScryptoNonFungibleLocalId> {
        self.transfers
            .clone()
            .into_iter()
            .flat_map(|x| x.non_fungible_local_ids)
            .map(ScryptoNonFungibleLocalId::from)
            .collect_vec()
    }
}

impl PerAssetTransfersOfNonFungibleResource {
    pub(crate) fn sample_mainnet() -> Self {
        Self::new(
            ResourceAddress::sample_mainnet_xrd(),
            [
                PerAssetNonFungibleTransfer::sample_mainnet(),
                PerAssetNonFungibleTransfer::sample_mainnet_other(),
            ],
        )
    }

    pub(crate) fn sample_mainnet_other() -> Self {
        Self::new(
            ResourceAddress::sample_mainnet_candy(),
            [PerAssetNonFungibleTransfer::sample_mainnet_other()],
        )
    }

    pub(crate) fn sample_stokenet() -> Self {
        Self::new(
            ResourceAddress::sample_stokenet_candy(),
            [
                PerAssetNonFungibleTransfer::sample_stokenet(),
                PerAssetNonFungibleTransfer::sample_stokenet_other(),
            ],
        )
    }

    pub(crate) fn sample_stokenet_other() -> Self {
        Self::new(
            ResourceAddress::sample_stokenet_gum(),
            [PerAssetNonFungibleTransfer::sample_stokenet_other()],
        )
    }
}

impl HasSampleValues for PerAssetTransfersOfNonFungibleResource {
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    fn sample_other() -> Self {
        Self::sample_stokenet_other()
    }
}
