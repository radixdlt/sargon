use crate::prelude::*;
use sargon::MessageV2 as InternalMessageV2;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum MessageV2 {
    PlainText { plaintext: PlaintextMessage },
    None,
}

#[uniffi::export]
pub fn new_message_v2_plaintext_sample() -> MessageV2 {
    InternalMessageV2::sample().into()
}

#[uniffi::export]
pub fn new_message_v2_plaintext_sample_other() -> MessageV2 {
    InternalMessageV2::sample_other().into()
}

#[uniffi::export]
pub fn new_message_v2_plaintext_string(string: String) -> MessageV2 {
    InternalMessageV2::plain_text(string).into()
}

#[uniffi::export]
pub fn message_v2_as_plaintext(message: &MessageV2) -> Option<String> {
    message.into_internal().as_plaintext()
}
