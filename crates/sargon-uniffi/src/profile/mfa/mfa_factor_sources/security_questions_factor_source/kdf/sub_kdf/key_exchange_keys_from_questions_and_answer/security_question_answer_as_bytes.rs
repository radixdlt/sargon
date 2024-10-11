use crate::prelude::*;
use sargon::SecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes as InternalSecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes;

/// ❗️ NOT PRODUCTION READY YET ❗️
/// An answer **as bytes** to some security question, being the output of some
/// set of functions mapping answer -> bytes.
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(
     Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record,
)]
#[allow(non_camel_case_types)]
pub struct SecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes {
    pub bytes: BagOfBytes,
}

impl From<InternalSecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes> for SecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes {
    fn from(value: InternalSecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes) -> Self {
        Self {
            bytes: value.bytes.into(),
        }
    }
}

impl Into<InternalSecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes> for SecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes {
    fn into(self) -> InternalSecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes {
        InternalSecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes {
            bytes: self.bytes.into(),
        }
    }
}