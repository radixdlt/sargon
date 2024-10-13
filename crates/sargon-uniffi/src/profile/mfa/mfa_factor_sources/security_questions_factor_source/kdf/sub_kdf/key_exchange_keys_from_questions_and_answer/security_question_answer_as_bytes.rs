use crate::prelude::*;
use sargon::SecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes as InternalSecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes;

/// ❗️ NOT PRODUCTION READY YET ❗️
/// An answer **as bytes** to some security question, being the output of some
/// set of functions mapping answer -> bytes.
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(
     Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Record,
)]
#[allow(non_camel_case_types)]
pub struct SecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes {
    pub bytes: BagOfBytes,
}