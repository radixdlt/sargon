use crate::prelude::*;

/// A helper struct to group all the transferable resources of a given account.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct FetchTransferableResourcesOutput {
    /// The list of fungible resources that can be transferred.
    pub fungibles: Vec<FungibleResourcesCollectionItem>,

    /// The list of non-fungible resources that can be transferred.
    pub non_fungibles: Vec<NonFungibleResourcesCollectionItem>,

    /// The list of non-transferable resources.
    pub non_transferable_resources: Vec<ResourceAddress>,
}

impl FetchTransferableResourcesOutput {
    pub fn new(
        fungibles: Vec<FungibleResourcesCollectionItem>,
        non_fungibles: Vec<NonFungibleResourcesCollectionItem>,
        non_transferable_resources: Vec<ResourceAddress>,
    ) -> Self {
        Self {
            fungibles,
            non_fungibles,
            non_transferable_resources,
        }
    }
}
