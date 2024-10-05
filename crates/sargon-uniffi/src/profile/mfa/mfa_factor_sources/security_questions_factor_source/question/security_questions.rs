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

#[uniffi::export]
pub fn security_questions_all() -> Security_NOT_PRODUCTION_READY_Questions {
    Security_NOT_PRODUCTION_READY_Questions::from_iter(
        Security_NOT_PRODUCTION_READY_Question::all(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        assert_eq!(security_questions_all().len(), 17);
    }
}
