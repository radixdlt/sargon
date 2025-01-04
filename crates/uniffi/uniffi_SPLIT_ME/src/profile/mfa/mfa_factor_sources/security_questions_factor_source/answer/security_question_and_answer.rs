use crate::prelude::*;
use sargon::Security_NOT_PRODUCTION_READY_QuestionAndAnswer as InternalSecurity_NOT_PRODUCTION_READY_QuestionAndAnswer;

decl_vec_samples_for!(
    Security_NOT_PRODUCTION_READY_QuestionsAndAnswers,
    Security_NOT_PRODUCTION_READY_QuestionAndAnswer
);

/// ❗️ NOT PRODUCTION READY YET ❗️
/// A pair of security question and answer
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
#[allow(non_camel_case_types)]
pub struct Security_NOT_PRODUCTION_READY_QuestionAndAnswer {
    pub question: Security_NOT_PRODUCTION_READY_Question,
    pub answer: String,
}
