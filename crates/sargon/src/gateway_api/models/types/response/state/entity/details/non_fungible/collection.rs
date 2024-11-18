use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct NonFungibleResourcesCollection {
    /// Total number of items in underlying collection, fragment of which is available in `items` collection.
    pub total_count: Option<u64>,

    /// If specified, contains a cursor to query next page of the `items` collection.
    pub next_cursor: Option<String>,

    /// Collection of fungible resources.
    pub items: Vec<NonFungibleResourcesCollectionItem>,
}

impl NonFungibleResourcesCollection {
    pub fn new(
        total_count: impl Into<Option<u64>>,
        next_cursor: impl Into<Option<String>>,
        items: Vec<NonFungibleResourcesCollectionItem>,
    ) -> NonFungibleResourcesCollection {
        NonFungibleResourcesCollection {
            total_count: total_count.into(),
            next_cursor: next_cursor.into(),
            items,
        }
    }
}
