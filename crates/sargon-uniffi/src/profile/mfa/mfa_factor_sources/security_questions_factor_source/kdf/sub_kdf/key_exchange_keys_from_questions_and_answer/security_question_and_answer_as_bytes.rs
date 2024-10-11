use crate::prelude::*;
use sargon::SecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes as InternalSecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes;

/// ❗️ NOT PRODUCTION READY YET ❗️
/// A pair of security question and answer
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(
    Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record,
)]
#[allow(non_camel_case_types)]
pub struct SecurityQuestion_NOT_PRODUCTION_READY_AndAnswerAsBytes {
    pub question: Security_NOT_PRODUCTION_READY_Question,
    pub answer: SecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes,
}

impl From<InternalSecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes> for SecurityQuestion_NOT_PRODUCTION_READY_AndAnswerAsBytes {
    fn from(value: InternalSecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes) -> Self {
        Self {
            question: value.question.into(),
            answer: value.answer.into(),
        }
    }
}

impl Into<InternalSecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes> for SecurityQuestion_NOT_PRODUCTION_READY_AndAnswerAsBytes {
    fn into(self) -> InternalSecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes {
        InternalSecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes {
            question: self.question.into(),
            answer: self.answer.into(),
        }
    }
}