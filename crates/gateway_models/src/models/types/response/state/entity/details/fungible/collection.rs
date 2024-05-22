use crate::prelude::*;

#[derive(
    Deserialize,
    Serialize, /* Serialize so we can test roundtrip of JSON vectors */
    Clone,
    PartialEq,
    Eq,
    Debug,
    uniffi::Record,
)]
pub struct FungibleResourcesCollection {
    pub items: Vec<FungibleResourcesCollectionItem>,
}
