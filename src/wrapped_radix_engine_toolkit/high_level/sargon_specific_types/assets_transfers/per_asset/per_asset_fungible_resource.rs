use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct PerAssetFungibleResource {
    pub resource_address: ResourceAddress,
    pub divisibility: Option<i32>,
}

impl PerAssetFungibleResource {
    pub fn new(
        resource_address: ResourceAddress,
        divisibility: impl Into<Option<i32>>,
    ) -> Self {
        Self {
            resource_address,
            divisibility: divisibility.into(),
        }
    }
}

impl PerAssetFungibleResource {
    pub(crate) fn sample_mainnet() -> Self {
        Self::new(ResourceAddress::sample_mainnet_xrd(), None)
    }

    pub(crate) fn sample_mainnet_other() -> Self {
        Self::new(ResourceAddress::sample_mainnet_candy(), 4)
    }

    pub(crate) fn sample_stokenet() -> Self {
        Self::new(ResourceAddress::sample_stokenet_xrd(), None)
    }

    pub(crate) fn sample_stokenet_other() -> Self {
        Self::new(ResourceAddress::sample_stokenet_gum(), 6)
    }
}

impl HasSampleValues for PerAssetFungibleResource {
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
