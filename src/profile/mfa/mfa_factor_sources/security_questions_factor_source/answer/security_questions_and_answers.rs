use crate::prelude::*;

decl_identified_vec_of!(
    /// ❗️ NOT PRODUCTION READY YET ❗️
    /// An ordered set of [`SecurityQuestionAndAnswer`]s user has selected to use
    /// in their `SecurityQuestionsFactorSource` - a factor they typically
    /// use for the `ConfirmationRole`.
    /// ❗️ NOT PRODUCTION READY YET ❗️
    Security_NOT_PRODUCTION_READY_QuestionsAndAnswers,
    Security_NOT_PRODUCTION_READY_QuestionAndAnswer
);

impl HasSampleValues for Security_NOT_PRODUCTION_READY_QuestionsAndAnswers {
    fn sample() -> Self {
        type Q = Security_NOT_PRODUCTION_READY_Question;
        type QA = Security_NOT_PRODUCTION_READY_QuestionAndAnswer;
        Self::from_iter([
            QA::new(Q::failed_exam(), "MIT, year 4, Python"),
            QA::new(Q::parents_met(), "London, 1973"),
            QA::new(
                Q::first_concert(),
                "Jean-Michel Jarre, Paris La Défense, 1990",
            ),
            QA::new(Q::first_kiss_whom(), "John Doe"),
            QA::new(
                Q::first_kiss_location(),
                "Behind the shed in the oak tree forrest.",
            ),
            QA::new(Q::spouse_met(), "Tokyo, 1989"),
        ])
    }

    fn sample_other() -> Self {
        type Q = Security_NOT_PRODUCTION_READY_Question;
        type QA = Security_NOT_PRODUCTION_READY_QuestionAndAnswer;
        Self::from_iter([
            QA::new(Q::child_middle_name(), "Joe"),
            QA::new(Q::stuffed_animal(), "Bobby"),
            QA::new(Q::oldest_cousin(), "Roxanne"),
            QA::new(Q::teacher_grade3(), "Ali"),
            QA::new(Q::applied_uni_no_attend(), "Oxford"),
            QA::new(Q::first_school(), "Hogwartz"),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Security_NOT_PRODUCTION_READY_QuestionsAndAnswers;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
