use crate::prelude::*;
use sargon::Security_NOT_PRODUCTION_READY_Question as InternalSecurity_NOT_PRODUCTION_READY_Question;
use sargon::Security_NOT_PRODUCTION_READY_Questions as InternalSecurity_NOT_PRODUCTION_READY_Questions;

#[uniffi::export]
pub fn security_questions_all() -> Vec<Security_NOT_PRODUCTION_READY_Question> {
    InternalSecurity_NOT_PRODUCTION_READY_Questions::from_iter(
        InternalSecurity_NOT_PRODUCTION_READY_Question::all(),
    )
    .into_type()
}
