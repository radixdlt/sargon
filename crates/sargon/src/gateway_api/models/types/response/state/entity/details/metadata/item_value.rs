use crate::prelude::*;

#[derive(
    Deserialize, Serialize, Clone, PartialEq, Eq, Debug, uniffi::Record,
)]
pub struct EntityMetadataItemValue {
    pub typed: MetadataTypedValue,
}
