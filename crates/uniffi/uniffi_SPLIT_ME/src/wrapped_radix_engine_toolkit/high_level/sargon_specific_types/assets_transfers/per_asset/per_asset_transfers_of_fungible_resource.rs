use crate::prelude::*;
use sargon::PerAssetTransfersOfFungibleResource as InternalPerAssetTransfersOfFungibleResource;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct PerAssetTransfersOfFungibleResource {
    pub resource: PerAssetFungibleResource,
    pub transfers: Vec<PerAssetFungibleTransfer>,
}
