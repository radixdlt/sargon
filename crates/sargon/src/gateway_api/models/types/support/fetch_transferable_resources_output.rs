use crate::prelude::*;

/// A helper struct to group all the transferable resources of a given account.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct FetchTransferableResourcesOutput {
    /// The list of fungible resources that can be transferred.
    pub fungibles: Vec<FungibleResourcesCollectionItemGloballyAggregated>,

    /// The list of non-fungible resources that can be transferred.
    pub non_fungibles:
        Vec<NonFungibleResourcesCollectionItemGloballyAggregated>,

    /// The list of non-transferable resources.
    pub non_transferable_resources: Vec<ResourceAddress>,
}

impl FetchTransferableResourcesOutput {
    pub fn new(
        fungibles: Vec<FungibleResourcesCollectionItemGloballyAggregated>,
        non_fungibles: Vec<
            NonFungibleResourcesCollectionItemGloballyAggregated,
        >,
        non_transferable_resources: Vec<ResourceAddress>,
    ) -> Self {
        Self {
            fungibles,
            non_fungibles,
            non_transferable_resources,
        }
    }
}
