use crate::prelude::*;
use sargon::PerAssetFungibleResource as InternalPerAssetFungibleResource;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct PerAssetFungibleResource {
    pub resource_address: ResourceAddress,
    pub divisibility: Option<u8>,
}
