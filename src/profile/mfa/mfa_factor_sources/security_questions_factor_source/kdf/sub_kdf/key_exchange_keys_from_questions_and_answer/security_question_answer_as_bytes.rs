use crate::prelude::*;

/// ❗️ NOT PRODUCTION READY YET ❗️
/// An answer **as bytes** to some security question, being the output of some
/// set of functions mapping answer -> bytes.
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(
    Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug, uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
pub struct SecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes {
    pub bytes: BagOfBytes,
}

impl SecurityQuestion_NOT_PRODUCTION_READY_AnswerAsBytes {
    fn bytes_from_trimmed_answer(freeform_answer: TrimmedAnswer) -> BagOfBytes {
        BagOfBytes::from(freeform_answer.trimmed_answer.into_bytes())
    }

    pub fn validate_conversion_to_bytes_of(
        answer: impl AsRef<str>,
    ) -> Result<Self> {
        let answer = answer.as_ref().to_owned();
        if answer.is_empty() {
            return Err(CommonError::AnswersToSecurityQuestionsCannotBeEmpty);
        }
        let trimmed = TrimmedAnswer::new(answer)?;
        let bytes = Self::bytes_from_trimmed_answer(trimmed);
        Ok(Self { bytes })
    }
}
