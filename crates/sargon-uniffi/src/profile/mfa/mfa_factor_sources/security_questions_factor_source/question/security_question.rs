use crate::prelude::*;
use sargon::SecurityQuestion_NOT_PRODUCTION_READY_ExpectedAnswerFormat as InternalSecurityQuestion_NOT_PRODUCTION_READY_ExpectedAnswerFormat;
use sargon::Security_NOT_PRODUCTION_READY_Question as InternalSecurity_NOT_PRODUCTION_READY_Question;

decl_vec_samples_for!(Security_NOT_PRODUCTION_READY_Questions, Security_NOT_PRODUCTION_READY_Question);

/// ❗️ NOT PRODUCTION READY YET ❗️
/// A security question
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
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
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
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
