use crate::prelude::*;

#[derive(
    Deserialize,
    Serialize, /* Serialize so we can test roundtrip of JSON vectors */
    Clone,
    PartialEq,
    Eq,
    Debug,
    derive_more::Display,
)]
#[display("{message}")]
pub struct TransactionPreviewResponseLogsInner {
    pub level: String,
    pub message: String,
}
