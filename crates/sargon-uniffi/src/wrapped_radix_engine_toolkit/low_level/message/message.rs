use crate::prelude::*;
use sargon::Message as InternalMessage;

#[derive(Clone, Debug, PartialEq, EnumAsInner, Eq, Hash, uniffi::Enum)]
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
    message.into::<InternalMessage>().as_plaintext()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inequality() {
        assert_ne!(
            new_message_plaintext_sample(),
            new_message_plaintext_sample_other()
        );
    }

    #[test]
    fn new_message_plaintext_string_then_as_plaintext() {
        let text = "Hello Unit Test".to_owned();
        assert_eq!(
            message_as_plaintext(&new_message_plaintext_string(text.clone())),
            Some(text)
        );
    }
}
