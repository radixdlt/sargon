use transaction::model::{
    MessageContentsV1 as ScryptoMessageContents, MessageV1 as ScryptoMessage,
    PlaintextMessageV1 as ScryptoPlaintextMessage,
};

use crate::prelude::*;

/// We explicitly mark content as either String or Bytes - this distinguishes (along with the mime type)
/// whether the message is intended to be displayable as text, or not.
///
/// This data model ensures that messages intended to be displayable as text are valid unicode strings.
#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumAsInner, uniffi::Enum)]
pub enum MessageContents {
    StringMessage { string: String },
    BinaryMessage { bag_of_bytes: BagOfBytes },
}

impl From<ScryptoMessageContents> for MessageContents {
    fn from(value: ScryptoMessageContents) -> Self {
        match value {
            ScryptoMessageContents::String(string) => {
                Self::StringMessage { string }
            }
            ScryptoMessageContents::Bytes(vec) => Self::BinaryMessage {
                bag_of_bytes: vec.into(),
            },
        }
    }
}
impl From<MessageContents> for ScryptoMessageContents {
    fn from(value: MessageContents) -> Self {
        match value {
            MessageContents::StringMessage { string } => {
                ScryptoMessageContents::String(string)
            }
            MessageContents::BinaryMessage { bag_of_bytes } => {
                ScryptoMessageContents::Bytes(bag_of_bytes.to_vec())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct PlaintextMessage {
    pub mime_type: String,
    pub message: MessageContents,
}

impl PlaintextMessage {
    pub fn new(message: impl AsRef<str>) -> Self {
        Self {
            mime_type: "text/plain".to_owned(),
            message: MessageContents::StringMessage {
                string: message.as_ref().to_owned(),
            },
        }
    }
}

impl From<PlaintextMessage> for ScryptoPlaintextMessage {
    fn from(value: PlaintextMessage) -> Self {
        Self {
            mime_type: value.mime_type,
            message: value.message.into(),
        }
    }
}
impl From<ScryptoPlaintextMessage> for PlaintextMessage {
    fn from(value: ScryptoPlaintextMessage) -> Self {
        Self {
            mime_type: value.mime_type,
            message: value.message.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, EnumAsInner, Eq, Hash, uniffi::Enum)]
pub enum Message {
    PlainText { plaintext: PlaintextMessage },
    None,
}

impl Message {
    pub fn plain_text(message: impl AsRef<str>) -> Self {
        Self::PlainText {
            plaintext: PlaintextMessage::new(message),
        }
    }
}

impl From<Message> for ScryptoMessage {
    fn from(value: Message) -> Self {
        match value {
            Message::PlainText { plaintext } => {
                ScryptoMessage::Plaintext(plaintext.into())
            }
            Message::None => ScryptoMessage::None,
        }
    }
}

impl TryFrom<ScryptoMessage> for Message {
    type Error = crate::CommonError;

    fn try_from(value: ScryptoMessage) -> Result<Self, Self::Error> {
        match value {
            ScryptoMessage::None => Ok(Self::None),
            ScryptoMessage::Plaintext(p) => Ok(Self::PlainText {
                plaintext: p.into(),
            }),
            ScryptoMessage::Encrypted(_) => {
                Err(CommonError::EncryptedMessagesAreNotYetSupported)
            }
        }
    }
}

impl HasSampleValues for Message {
    fn sample() -> Self {
        Self::plain_text("Hello Radix!")
    }

    fn sample_other() -> Self {
        Self::plain_text("Lorem ipsum!!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Message;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn into_scrypto() {
        assert_eq!(
            Into::<ScryptoMessage>::into(SUT::sample()),
            ScryptoMessage::Plaintext(ScryptoPlaintextMessage {
                message: ScryptoMessageContents::String(
                    "Hello Radix!".to_owned()
                ),
                mime_type: "text/plain".to_owned(),
            })
        );
    }
}
