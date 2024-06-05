use crate::prelude::*;

/// ❗️ NOT PRODUCTION READY YET ❗️
/// An pair of security question and answer
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(
    Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug, uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
pub struct Security_NOT_PRODUCTION_READY_QuestionAndAnswer {
    pub question: Security_NOT_PRODUCTION_READY_Question,
    pub answer: String,
}

impl Security_NOT_PRODUCTION_READY_QuestionAndAnswer {
    pub fn new(
        question: Security_NOT_PRODUCTION_READY_Question,
        answer: impl AsRef<str>,
    ) -> Self {
        Self {
            question,
            answer: answer.as_ref().to_owned(),
        }
    }
}

impl HasSampleValues for Security_NOT_PRODUCTION_READY_QuestionAndAnswer {
    fn sample() -> Self {
        Self::new(
            Security_NOT_PRODUCTION_READY_Question::first_concert(),
            "Jean-Michel Jarre, Paris La Défense, 1990",
        )
    }

    fn sample_other() -> Self {
        Self::new(
            Security_NOT_PRODUCTION_READY_Question::stuffed_animal(),
            "Oinky piggy pig",
        )
    }
}

impl Identifiable for Security_NOT_PRODUCTION_READY_QuestionAndAnswer {
    type ID = <Security_NOT_PRODUCTION_READY_Question as Identifiable>::ID;

    fn id(&self) -> Self::ID {
        self.question.id()
    }
}
