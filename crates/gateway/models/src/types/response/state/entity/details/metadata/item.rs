use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct EntityMetadataItem {
    pub key: String,
    pub value: EntityMetadataItemValue,
}
