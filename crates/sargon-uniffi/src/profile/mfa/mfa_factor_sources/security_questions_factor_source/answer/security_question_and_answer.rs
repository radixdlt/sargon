use crate::prelude::*;

/// ❗️ NOT PRODUCTION READY YET ❗️
/// A pair of security question and answer
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(
    Clone, PartialEq, Eq, Hash,   uniffi::Record,
)]
pub struct Security_NOT_PRODUCTION_READY_QuestionAndAnswer {
    pub question: Security_NOT_PRODUCTION_READY_Question,
    pub answer: String,
}