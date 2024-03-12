use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct PerAssetTransfersOfFungibleResource {
    pub resource: PerAssetFungibleResource,
    pub transfers: Vec<PerAssetFungibleTransfer>,
}

impl PerAssetTransfersOfFungibleResource {
    pub(crate) fn expanded(
        &mut self,
        transfer: impl Into<PerAssetFungibleTransfer>,
    ) {
        self.transfers.push(transfer.into());
    }
}

impl PerAssetTransfersOfFungibleResource {
    pub fn new(
        resource: PerAssetFungibleResource,
        transfers: impl IntoIterator<Item = PerAssetFungibleTransfer>,
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

impl From<(&AssetsTransfersRecipient, PerRecipientFungibleTransfer)>
    for PerAssetTransfersOfFungibleResource
{
    fn from(
        value: (&AssetsTransfersRecipient, PerRecipientFungibleTransfer),
    ) -> Self {
        let (recipient, fungible_with_amount) = value;
        Self::new(
            PerAssetFungibleResource::new(
                fungible_with_amount.resource_address,
                fungible_with_amount.divisibility,
            ),
            [PerAssetFungibleTransfer::new(
                recipient.clone(),
                fungible_with_amount.use_try_deposit_or_abort,
                fungible_with_amount.amount,
            )],
        )
    }
}

impl PerAssetTransfersOfFungibleResource {
    pub(crate) fn sample_mainnet() -> Self {
        Self::new(
            PerAssetFungibleResource::sample_mainnet(),
            [
                PerAssetFungibleTransfer::sample_mainnet(),
                PerAssetFungibleTransfer::sample_mainnet_other(),
            ],
        )
    }

    pub(crate) fn sample_mainnet_other() -> Self {
        Self::new(
            PerAssetFungibleResource::sample_mainnet_other(),
            [PerAssetFungibleTransfer::sample_mainnet_other()],
        )
    }

    pub(crate) fn sample_stokenet() -> Self {
        Self::new(
            PerAssetFungibleResource::sample_stokenet(),
            [
                PerAssetFungibleTransfer::sample_stokenet(),
                PerAssetFungibleTransfer::sample_stokenet_other(),
            ],
        )
    }

    pub(crate) fn sample_stokenet_other() -> Self {
        Self::new(
            PerAssetFungibleResource::sample_stokenet_other(),
            [PerAssetFungibleTransfer::sample_stokenet_other()],
        )
    }
}

impl HasSampleValues for PerAssetTransfersOfFungibleResource {
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    fn sample_other() -> Self {
        Self::sample_stokenet_other()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PerAssetTransfersOfFungibleResource;

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
}
