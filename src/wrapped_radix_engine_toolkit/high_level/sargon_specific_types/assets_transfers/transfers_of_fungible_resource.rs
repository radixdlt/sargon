use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct TransfersOfFungibleResource {
    pub resource: FungibleResource,
    pub transfers: Vec<FungibleTransfer>,
}

impl TransfersOfFungibleResource {
    pub fn new(
        resource: FungibleResource,
        transfers: impl IntoIterator<Item = FungibleTransfer>,
    ) -> Self {
        Self {
            resource,
            transfers: transfers.into_iter().collect_vec(),
        }
    }

    /// sum of all `amount`s in `self.transfers.map(|x| x.amount)`
    pub fn total_transfer_amount(&self) -> ScryptoDecimal192 {
        let amount = self
            .transfers
            .clone()
            .into_iter()
            .map(|x| x.amount)
            .fold(Decimal::zero(), |acc, x| acc + x);

        let rounded = amount.round(self.resource.divisibility);

        rounded.into()
    }
}

impl TransfersOfFungibleResource {
    pub(crate) fn sample_mainnet() -> Self {
        Self::new(
            FungibleResource::sample_mainnet(),
            [
                FungibleTransfer::sample_mainnet(),
                FungibleTransfer::sample_mainnet_other(),
            ],
        )
    }

    pub(crate) fn sample_mainnet_other() -> Self {
        Self::new(
            FungibleResource::sample_mainnet_other(),
            [FungibleTransfer::sample_mainnet_other()],
        )
    }

    pub(crate) fn sample_stokenet() -> Self {
        Self::new(
            FungibleResource::sample_stokenet(),
            [
                FungibleTransfer::sample_stokenet(),
                FungibleTransfer::sample_stokenet_other(),
            ],
        )
    }

    pub(crate) fn sample_stokenet_other() -> Self {
        Self::new(
            FungibleResource::sample_stokenet_other(),
            [FungibleTransfer::sample_stokenet_other()],
        )
    }
}

impl HasSampleValues for TransfersOfFungibleResource {
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    fn sample_other() -> Self {
        Self::sample_stokenet_other()
    }
}
