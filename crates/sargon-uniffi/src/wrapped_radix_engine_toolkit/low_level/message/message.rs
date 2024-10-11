use crate::prelude::*;
use sargon::Message as InternalMessage;

#[derive(Clone,  PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum Message {
    PlainText { plaintext: PlaintextMessage },
    None,
}

impl From<InternalMessage> for Message {
    fn from(value: InternalMessage) -> Self {
        match value {
            InternalMessage::PlainText { plaintext } => Message::PlainText {
                plaintext: plaintext.into(),
            },
            InternalMessage::None => Message::None,
        }
    }
}

impl Into<InternalMessage> for Message {
    fn into(self) -> InternalMessage {
        match self {
            Message::PlainText { plaintext } => InternalMessage::PlainText {
                plaintext: plaintext.into(),
            },
            Message::None => InternalMessage::None,
        }
    }
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

