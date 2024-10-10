use crate::prelude::*;

#[derive(
    Clone, PartialEq, Eq, Hash,  uniffi::Enum,
)]
pub enum SecurityQuestionKind {
    Freeform,
}
