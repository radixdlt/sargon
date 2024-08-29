use crate::prelude::*;

#[derive(
    Deserialize, Serialize, Clone, PartialEq, Eq, Debug, uniffi::Record,
)]
pub struct EntityMetadataItem {
    pub key: String,
    pub value: EntityMetadataItemValue,
}
