use crate::prelude::*;
use sargon::PerAssetTransfersOfFungibleResource as InternalPerAssetTransfersOfFungibleResource;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct PerAssetTransfersOfFungibleResource {
    pub resource: PerAssetFungibleResource,
    pub transfers: Vec<PerAssetFungibleTransfer>,
}

impl From<InternalPerAssetTransfersOfFungibleResource> for PerAssetTransfersOfFungibleResource {
    fn from(value: InternalPerAssetTransfersOfFungibleResource) -> Self {
        Self {
            resource: value.resource.into(),
            transfers: value.transfers.into_vec(),
        }
    }
}

impl Into<InternalPerAssetTransfersOfFungibleResource> for PerAssetTransfersOfFungibleResource {
    fn into(self) -> InternalPerAssetTransfersOfFungibleResource {
        InternalPerAssetTransfersOfFungibleResource {
            resource: self.resource.into(),
            transfers: self.transfers.into_internal_vec(),
        }
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
