use crate::prelude::*;

/// ❗️ NOT PRODUCTION READY YET ❗️
/// An answer **as bytes** to some security question, being the output of some
/// set of functions mapping answer -> bytes.
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(
     Clone, PartialEq, Eq, Hash, Debug,  uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
pub struct SecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes {
    pub bytes: BagOfBytes,
}