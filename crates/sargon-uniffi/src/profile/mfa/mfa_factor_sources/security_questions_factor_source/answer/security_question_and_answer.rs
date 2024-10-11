use crate::prelude::*;
use sargon::Security_NOT_PRODUCTION_READY_QuestionAndAnswer as InternalSecurity_NOT_PRODUCTION_READY_QuestionAndAnswer;

/// ❗️ NOT PRODUCTION READY YET ❗️
/// A pair of security question and answer
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(
    Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record,
)]
pub struct Security_NOT_PRODUCTION_READY_QuestionAndAnswer {
    pub question: Security_NOT_PRODUCTION_READY_Question,
    pub answer: String,
}

impl From<InternalSecurity_NOT_PRODUCTION_READY_QuestionAndAnswer> for Security_NOT_PRODUCTION_READY_QuestionAndAnswer {
    fn from(value: InternalSecurity_NOT_PRODUCTION_READY_QuestionAndAnswer) -> Self {
        Self {
            question: value.question.into(),
            answer: value.answer,
        }
    }
}

impl Into<InternalSecurity_NOT_PRODUCTION_READY_QuestionAndAnswer> for Security_NOT_PRODUCTION_READY_QuestionAndAnswer {
    fn into(self) -> InternalSecurity_NOT_PRODUCTION_READY_QuestionAndAnswer {
        InternalSecurity_NOT_PRODUCTION_READY_QuestionAndAnswer {
            question: self.question.into(),
            answer: self.answer,
        }
    }
}