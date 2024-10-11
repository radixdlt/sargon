use crate::prelude::*;
use sargon::SecurityQuestionKind as InternalSecurityQuestionKind;

#[derive(
    Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum,
)]
pub enum SecurityQuestionKind {
    Freeform,
}

impl From<InternalSecurityQuestionKind> for SecurityQuestionKind {
    fn from(value: InternalSecurityQuestionKind) -> Self {
        match value {
            InternalSecurityQuestionKind::Freeform => SecurityQuestionKind::Freeform,
        }
    }
}

impl Into<InternalSecurityQuestionKind> for SecurityQuestionKind {
    fn into(self) -> InternalSecurityQuestionKind {
        match self {
            SecurityQuestionKind::Freeform => InternalSecurityQuestionKind::Freeform,
        }
    }
}
