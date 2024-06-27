use crate::prelude::*;

/// ❗️ NOT PRODUCTION READY YET ❗️
/// A pair of security question and answer
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(
    Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug, uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
pub struct SecurityQuestion_NOT_PRODUCTION_READY_AndAnswerAsBytes {
    pub question: Security_NOT_PRODUCTION_READY_Question,
    pub answer: SecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes,
}

impl SecurityQuestion_NOT_PRODUCTION_READY_AndAnswerAsBytes {
    pub fn answer_to_question(
        freeform: impl AsRef<str>,
        question: Security_NOT_PRODUCTION_READY_Question,
    ) -> Result<Self> {
        let answer = SecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes::validate_conversion_to_bytes_of(freeform.as_ref())?;
        Ok(Self { question, answer })
    }
}
