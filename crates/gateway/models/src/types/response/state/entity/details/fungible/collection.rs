use crate::prelude::*;

#[derive(
    Deserialize,
    Serialize, /* Serialize so we can test roundtrip of JSON vectors */
    Clone,
    PartialEq,
    Eq,
    Debug,
)]
pub struct FungibleResourcesCollection {
    /// Total number of items in underlying collection, fragment of which is available in `items` collection.
    pub total_count: Option<u64>,

    /// If specified, contains a cursor to query next page of the `items` collection.
    pub next_cursor: Option<String>,

    /// Collection of fungible resources.
    pub items: Vec<FungibleResourcesCollectionItem>,
}

impl FungibleResourcesCollection {
    pub fn new(
        total_count: impl Into<Option<u64>>,
        next_cursor: impl Into<Option<String>>,
        items: impl IntoIterator<Item = FungibleResourcesCollectionItem>,
    ) -> FungibleResourcesCollection {
        FungibleResourcesCollection {
            total_count: total_count.into(),
            next_cursor: next_cursor.into(),
            items: items.into_iter().collect(),
        }
    }
}
