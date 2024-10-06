use crate::prelude::*;

#[derive(
    Deserialize, Serialize, Clone, PartialEq, Eq, Debug
)]
pub struct EntityMetadataItemValue {
    pub typed: MetadataTypedValue,
}
