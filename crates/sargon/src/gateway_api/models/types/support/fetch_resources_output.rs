use crate::prelude::*;

/// A helper struct to grpup all the resources of a given account.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct FetchResourcesOutput {
    pub fungibles: Vec<FungibleResourcesCollectionItem>,
    pub non_fungibles: Vec<NonFungibleResourcesCollectionItem>,
}

impl FetchResourcesOutput {
    pub fn new(
        fungibles: Vec<FungibleResourcesCollectionItem>,
        non_fungibles: Vec<NonFungibleResourcesCollectionItem>,
    ) -> Self {
        Self {
            fungibles,
            non_fungibles,
        }
    }
}
