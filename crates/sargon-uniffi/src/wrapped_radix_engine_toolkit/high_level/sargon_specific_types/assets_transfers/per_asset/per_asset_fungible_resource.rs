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


