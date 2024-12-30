use sargon_core::decl_bool_type;

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
    pub(crate) use_free_credit: UseFreeCredit,
    pub(crate) assume_all_signature_proofs: AssumeAllSignatureProofs,
    pub(crate) skip_epoch_check: SkipEpochCheck,
}

decl_bool_type!(UseFreeCredit, true);
decl_bool_type!(AssumeAllSignatureProofs, false);
decl_bool_type!(SkipEpochCheck, false);
