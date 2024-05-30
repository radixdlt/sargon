use crate::prelude::*;

decl_identified_vec_of!(
    /// ❗️ NOT PRODUCTION READY YET ❗️
    /// An ordered set of [`SecurityQuestion`]s user has selected to use
    /// in their `SecurityQuestionsFactorSource` - a factor they typically
    /// use for the `ConfirmationRole`.
    /// ❗️ NOT PRODUCTION READY YET ❗️
    Security_NOT_PRODUCTION_READY_Questions,
    Security_NOT_PRODUCTION_READY_Question
);

impl HasSampleValues for Security_NOT_PRODUCTION_READY_Questions {
    fn sample() -> Self {
        Self::from_iter([
            Security_NOT_PRODUCTION_READY_Question::sample(),
            Security_NOT_PRODUCTION_READY_Question::sample_other(),
        ])
    }
    fn sample_other() -> Self {
        Self::from_iter([
            Security_NOT_PRODUCTION_READY_Question::with_details(
                Uuid::from_bytes([0x5e; 16]),
                1,
                SecurityQuestionKind::Freeform,
                "In what city did your parents meet?",
            ),
            Security_NOT_PRODUCTION_READY_Question::sample(),
        ])
    }
}
