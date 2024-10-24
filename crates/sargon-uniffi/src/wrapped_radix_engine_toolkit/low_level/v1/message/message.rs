use crate::prelude::*;
use sargon::Message as InternalMessage;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum Message {
    PlainText { plaintext: PlaintextMessage },
    None,
}

#[uniffi::export]
pub fn new_message_plaintext_sample() -> Message {
    InternalMessage::sample().into()
}

#[uniffi::export]
pub fn new_message_plaintext_sample_other() -> Message {
    InternalMessage::sample_other().into()
}

#[uniffi::export]
pub fn new_message_plaintext_string(string: String) -> Message {
    InternalMessage::plain_text(string).into()
}

#[uniffi::export]
pub fn message_as_plaintext(message: &Message) -> Option<String> {
    message.into_internal().as_plaintext()
}
