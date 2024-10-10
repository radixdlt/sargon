use crate::prelude::*;

/// ❗️ NOT PRODUCTION READY YET ❗️
/// A specification of expected format for an answer to a security question.
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(
    Clone, PartialEq, Eq, Hash,   uniffi::Record,
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