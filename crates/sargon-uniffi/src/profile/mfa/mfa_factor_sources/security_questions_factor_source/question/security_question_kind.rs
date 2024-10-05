use crate::prelude::*;

#[derive(
    Clone, PartialEq, Eq, Hash, Debug, uniffi::Enum,
)]
pub enum SecurityQuestionKind {
    Freeform,
}
