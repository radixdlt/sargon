use crate::prelude::*;
use sargon::PerAssetTransfersOfNonFungibleResource as InternalPerAssetTransfersOfNonFungibleResource;

#[derive(Clone, Debug, PartialEq, Eq, Hash,  uniffi::Record)]
pub struct PerAssetTransfersOfNonFungibleResource {
    pub resource: ResourceAddress,
    pub transfers: Vec<PerAssetNonFungibleTransfer>,
}

impl From<InternalPerAssetTransfersOfNonFungibleResource> for PerAssetTransfersOfNonFungibleResource {
    fn from(value: InternalPerAssetTransfersOfNonFungibleResource) -> Self {
        Self {
            resource: value.resource.into(),
            transfers: value.transfers.into_vec(),
        }
    }
}

impl Into<InternalPerAssetTransfersOfNonFungibleResource> for PerAssetTransfersOfNonFungibleResource {
    fn into(self) -> InternalPerAssetTransfersOfNonFungibleResource {
        InternalPerAssetTransfersOfNonFungibleResource {
            resource: self.resource.into(),
            transfers: self.transfers.into_internal_vec(),
        }
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
