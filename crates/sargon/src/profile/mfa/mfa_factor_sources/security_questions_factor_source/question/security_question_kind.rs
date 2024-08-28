use crate::prelude::*;

#[derive(
    Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug, uniffi::Enum,
)]
#[serde(rename_all = "camelCase")]
pub enum SecurityQuestionKind {
    Freeform,
}
