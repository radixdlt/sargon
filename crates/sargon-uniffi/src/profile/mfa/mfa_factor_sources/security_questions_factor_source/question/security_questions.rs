use crate::prelude::*;
use sargon::Security_NOT_PRODUCTION_READY_Question as InternalSecurity_NOT_PRODUCTION_READY_Question;

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
    InternalSecurity_NOT_PRODUCTION_READY_Questions::from_iter(
        InternalSecurity_NOT_PRODUCTION_READY_Question::all(),
    )
    .into_vec()
}
