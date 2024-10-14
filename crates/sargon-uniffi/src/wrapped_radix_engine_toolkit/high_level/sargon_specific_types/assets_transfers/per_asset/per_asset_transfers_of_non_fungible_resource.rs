use crate::prelude::*;
use sargon::PerAssetTransfersOfNonFungibleResource as InternalPerAssetTransfersOfNonFungibleResource;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Record)]
pub struct PerAssetTransfersOfNonFungibleResource {
    pub resource: ResourceAddress,
    pub transfers: Vec<PerAssetNonFungibleTransfer>,
}
