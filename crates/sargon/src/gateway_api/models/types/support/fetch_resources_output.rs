use crate::prelude::*;

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
