use crate::prelude::*;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize, /* Deserialize so we can test roundtrip of JSON vectors */
)]
pub struct TransactionPreviewRequestOptIns {
    /** This flag controls whether the preview response will include a Radix Engine Toolkit serializable receipt or not. */
    pub(crate) radix_engine_toolkit_receipt: bool,
}
