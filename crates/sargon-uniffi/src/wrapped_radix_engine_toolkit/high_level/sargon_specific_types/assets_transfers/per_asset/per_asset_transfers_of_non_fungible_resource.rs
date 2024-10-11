use crate::prelude::*;
use sargon::PerAssetTransfersOfNonFungibleResource as InternalPerAssetTransfersOfNonFungibleResource;

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct PerAssetTransfersOfNonFungibleResource {
    pub resource: ResourceAddress,
    pub transfers: Vec<PerAssetNonFungibleTransfer>,
}

impl From<InternalPerAssetTransfersOfNonFungibleResource>
    for PerAssetTransfersOfNonFungibleResource
{
    fn from(value: InternalPerAssetTransfersOfNonFungibleResource) -> Self {
        Self {
            resource: value.resource.into(),
            transfers: value.transfers.into_vec(),
        }
    }
}

impl Into<InternalPerAssetTransfersOfNonFungibleResource>
    for PerAssetTransfersOfNonFungibleResource
{
    fn into(self) -> InternalPerAssetTransfersOfNonFungibleResource {
        InternalPerAssetTransfersOfNonFungibleResource {
            resource: self.resource.into(),
            transfers: self.transfers.into_internal_vec(),
        }
    }
}
