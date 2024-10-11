use crate::prelude::*;
use sargon::PerAssetTransfersOfFungibleResource as InternalPerAssetTransfersOfFungibleResource;

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct PerAssetTransfersOfFungibleResource {
    pub resource: PerAssetFungibleResource,
    pub transfers: Vec<PerAssetFungibleTransfer>,
}

impl From<InternalPerAssetTransfersOfFungibleResource>
    for PerAssetTransfersOfFungibleResource
{
    fn from(value: InternalPerAssetTransfersOfFungibleResource) -> Self {
        Self {
            resource: value.resource.into(),
            transfers: value.transfers.into_vec(),
        }
    }
}

impl Into<InternalPerAssetTransfersOfFungibleResource>
    for PerAssetTransfersOfFungibleResource
{
    fn into(self) -> InternalPerAssetTransfersOfFungibleResource {
        InternalPerAssetTransfersOfFungibleResource {
            resource: self.resource.into(),
            transfers: self.transfers.into_internal_vec(),
        }
    }
}
