use crate::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize, /* Deserialize so we can test roundtrip of JSON vectors */
)]
pub struct TransactionPreviewRequestFlags {
    pub(crate) use_free_credit: bool,
    pub(crate) assume_all_signature_proofs: bool,
    pub(crate) skip_epoch_check: bool,
}
