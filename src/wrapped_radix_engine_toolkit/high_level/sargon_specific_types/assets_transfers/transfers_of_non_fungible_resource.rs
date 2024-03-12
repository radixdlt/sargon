use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct TransfersOfNonFungibleResource {
    pub resource: ResourceAddress,
    pub transfers: Vec<NonFungibleTransfer>,
}

impl TransfersOfNonFungibleResource {
    pub fn new(
        resource: impl Into<ResourceAddress>,
        transfers: impl IntoIterator<Item = NonFungibleTransfer>,
    ) -> Self {
        Self {
            resource: resource.into(),
            transfers: transfers.into_iter().collect_vec(),
        }
    }
}

impl TransfersOfNonFungibleResource {
    pub fn all_ids(&self) -> Vec<ScryptoNonFungibleLocalId> {
        self.transfers
            .clone()
            .into_iter()
            .flat_map(|x| x.non_fungible_local_ids)
            .map(ScryptoNonFungibleLocalId::from)
            .collect_vec()
    }
}

impl TransfersOfNonFungibleResource {
    pub(crate) fn sample_mainnet() -> Self {
        Self::new(
            ResourceAddress::sample_mainnet_xrd(),
            [
                NonFungibleTransfer::sample_mainnet(),
                NonFungibleTransfer::sample_mainnet_other(),
            ],
        )
    }

    pub(crate) fn sample_mainnet_other() -> Self {
        Self::new(
            ResourceAddress::sample_mainnet_candy(),
            [NonFungibleTransfer::sample_mainnet_other()],
        )
    }

    pub(crate) fn sample_stokenet() -> Self {
        Self::new(
            ResourceAddress::sample_stokenet_candy(),
            [
                NonFungibleTransfer::sample_stokenet(),
                NonFungibleTransfer::sample_stokenet_other(),
            ],
        )
    }

    pub(crate) fn sample_stokenet_other() -> Self {
        Self::new(
            ResourceAddress::sample_stokenet_gum(),
            [NonFungibleTransfer::sample_stokenet_other()],
        )
    }
}

impl HasSampleValues for TransfersOfNonFungibleResource {
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    fn sample_other() -> Self {
        Self::sample_stokenet_other()
    }
}
