use crate::prelude::*;
use sargon::SecurityQuestionKind as InternalSecurityQuestionKind;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
pub enum SecurityQuestionKind {
    Freeform,
}