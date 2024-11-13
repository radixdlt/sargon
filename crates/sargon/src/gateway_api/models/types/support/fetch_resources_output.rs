use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct FetchResourcesOutput {
    pub fungible: Vec<FungibleResourcesCollectionItem>,
    pub non_fungible: Vec<NonFungibleResourcesCollectionItem>,
}

impl FetchResourcesOutput {
    pub fn new(
        fungible: Vec<FungibleResourcesCollectionItem>,
        non_fungible: Vec<NonFungibleResourcesCollectionItem>,
    ) -> Self {
        Self {
            fungible,
            non_fungible,
        }
    }
}
