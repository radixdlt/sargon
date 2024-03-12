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
