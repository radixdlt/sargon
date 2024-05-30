use crate::prelude::*;

decl_identified_vec_of!(
    /// ❗️ NOT PRODUCTION READY YET ❗️
    /// An ordered set of [`SecurityQuestion`]s user has selected to use
    /// in their `SecurityQuestionsFactorSource` - a factor they typically
    /// use for the `ConfirmationRole`.
    /// ❗️ NOT PRODUCTION READY YET ❗️
    Security_NOT_PRODUCTION_READY_QuestionsAndAnswers,
    Security_NOT_PRODUCTION_READY_QuestionAndAnswer
);

impl HasSampleValues for Security_NOT_PRODUCTION_READY_QuestionsAndAnswers {
    fn sample() -> Self {
        // Self::from_iter([
        //     Security_NOT_PRODUCTION_READY_QuestionAndAnswer::sample(),
        //     Security_NOT_PRODUCTION_READY_QuestionAndAnswer::sample_other(),
        // ])
        todo!()
    }
    fn sample_other() -> Self {
        // Self::from_iter([
        //     Security_NOT_PRODUCTION_READY_Question::with_details(
        //         Uuid::from_bytes([0x5e; 16]),
        //         1,
        //         SecurityQuestionKind::Freeform,
        //         "In what city did your parents meet?",
        //     ),
        //     Security_NOT_PRODUCTION_READY_Question::sample(),
        // ])
        todo!()
    }
}
