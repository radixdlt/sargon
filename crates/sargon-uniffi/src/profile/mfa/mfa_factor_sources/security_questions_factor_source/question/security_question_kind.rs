use crate::prelude::*;
use sargon::SecurityQuestionKind as InternalSecurityQuestionKind;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum SecurityQuestionKind {
    Freeform,
}
