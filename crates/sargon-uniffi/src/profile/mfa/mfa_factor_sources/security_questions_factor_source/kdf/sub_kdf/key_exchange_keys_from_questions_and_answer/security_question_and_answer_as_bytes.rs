use crate::prelude::*;

/// ❗️ NOT PRODUCTION READY YET ❗️
/// A pair of security question and answer
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(
    Clone, PartialEq, Eq, Hash, Debug, uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
pub struct SecurityQuestion_NOT_PRODUCTION_READY_AndAnswerAsBytes {
    pub question: Security_NOT_PRODUCTION_READY_Question,
    pub answer: SecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes,
}
