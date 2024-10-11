use crate::prelude::*;
use sargon::SecurityQuestion_NOT_PRODUCTION_READY_ExpectedAnswerFormat as InternalSecurityQuestion_NOT_PRODUCTION_READY_ExpectedAnswerFormat;
use sargon::Security_NOT_PRODUCTION_READY_Question as InternalSecurity_NOT_PRODUCTION_READY_Question;

/// ❗️ NOT PRODUCTION READY YET ❗️
/// A security question
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(
    Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record,
)]
#[allow(non_camel_case_types)]
pub struct Security_NOT_PRODUCTION_READY_Question {
    pub id: u16,     // FIXME: newtype
    pub version: u8, // FIXME: newtype
    pub kind: SecurityQuestionKind,
    pub question: String,
    pub expected_answer_format:
        SecurityQuestion_NOT_PRODUCTION_READY_ExpectedAnswerFormat,
}

/// ❗️ NOT PRODUCTION READY YET ❗️
/// A specification of expected format for an answer to a security question.
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(
    Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record,
)]
#[allow(non_camel_case_types)]
pub struct SecurityQuestion_NOT_PRODUCTION_READY_ExpectedAnswerFormat {
    /// E.g. `"<CITY>, <YEAR>"`
    pub answer_structure: String,

    /// An example of a possible answer that matches `answer_structure`.
    /// E.g. `"Berlin, 1976"`
    pub example_answer: String,

    /// If user is about to select the question:
    /// `"What was the name of your first stuffed animal?"`
    ///
    /// Then we can discourage the user from selecting that question
    /// if the answer is in `["Teddy", "Peter Rabbit", "Winnie (the Poh)"]`
    pub unsafe_answers: Vec<String>,
}

impl From<InternalSecurity_NOT_PRODUCTION_READY_Question> for Security_NOT_PRODUCTION_READY_Question {
    fn from(value: InternalSecurity_NOT_PRODUCTION_READY_Question) -> Self {
        Self {
            id: value.id,
            version: value.version,
            kind: value.kind.into(),
            question: value.question,
            expected_answer_format: value.expected_answer_format.into(),
        }
    }
}

impl Into<InternalSecurity_NOT_PRODUCTION_READY_Question> for Security_NOT_PRODUCTION_READY_Question {
    fn into(self) -> InternalSecurity_NOT_PRODUCTION_READY_Question {
        InternalSecurity_NOT_PRODUCTION_READY_Question {
            id: self.id,
            version: self.version,
            kind: self.kind.into(),
            question: self.question,
            expected_answer_format: self.expected_answer_format.into(),
        }
    }
}

impl From<InternalSecurityQuestion_NOT_PRODUCTION_READY_ExpectedAnswerFormat> for SecurityQuestion_NOT_PRODUCTION_READY_ExpectedAnswerFormat {
    fn from(value: InternalSecurityQuestion_NOT_PRODUCTION_READY_ExpectedAnswerFormat) -> Self {
        Self {
            answer_structure: value.answer_structure,
            example_answer: value.example_answer,
            unsafe_answers: value.unsafe_answers,
        }
    }
}

impl Into<InternalSecurityQuestion_NOT_PRODUCTION_READY_ExpectedAnswerFormat> for SecurityQuestion_NOT_PRODUCTION_READY_ExpectedAnswerFormat {
    fn into(self) -> InternalSecurityQuestion_NOT_PRODUCTION_READY_ExpectedAnswerFormat {
        InternalSecurityQuestion_NOT_PRODUCTION_READY_ExpectedAnswerFormat {
            answer_structure: self.answer_structure,
            example_answer: self.example_answer,
            unsafe_answers: self.unsafe_answers,
        }
    }
}