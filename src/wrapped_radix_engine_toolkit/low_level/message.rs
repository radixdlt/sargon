use crate::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    EnumAsInner,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    uniffi::Enum,
)]
pub enum Message {
    PlainText { string: String },
}
