use crate::prelude::*;
use sargon::PerAssetFungibleResource as InternalPerAssetFungibleResource;

#[derive(Clone, Debug, PartialEq, Eq, Hash,  uniffi::Record)]
pub struct PerAssetFungibleResource {
    pub resource_address: ResourceAddress,
    pub divisibility: Option<u8>,
}

impl From<InternalPerAssetFungibleResource> for PerAssetFungibleResource {
    fn from(value: InternalPerAssetFungibleResource) -> Self {
        Self {
            resource_address: value.resource_address.into(),
            divisibility: value.divisibility,
        }
    }
}

impl Into<InternalPerAssetFungibleResource> for PerAssetFungibleResource {
    fn into(self) -> InternalPerAssetFungibleResource {
        InternalPerAssetFungibleResource {
            resource_address: self.resource_address.into(),
            divisibility: self.divisibility,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PerAssetFungibleResource;

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
