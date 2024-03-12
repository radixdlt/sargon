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

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PerAssetTransfersOfNonFungibleResource;

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
