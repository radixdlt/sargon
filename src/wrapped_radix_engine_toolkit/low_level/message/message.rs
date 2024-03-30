use crate::prelude::*;

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

impl Message {
    pub fn as_plaintext(&self) -> Option<String> {
        match self {
            Message::PlainText { plaintext } => plaintext.as_string(),
            Message::None => None,
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

    use transaction::model::EncryptedMessageV1;

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
    fn to_from_scrypto() {
        let roundtrip =
            |s: SUT| SUT::try_from(ScryptoMessage::from(s)).unwrap();
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other());
    }

    #[test]
    fn as_string() {
        assert_eq!(
            SUT::sample().as_plaintext(),
            Some("Hello Radix!".to_owned())
        );
        assert_eq!(
            SUT::PlainText {
                plaintext: PlaintextMessage::sample_binary()
            }
            .as_plaintext(),
            None
        );
        assert_eq!(SUT::None.as_plaintext(), None);
    }

    #[test]
    fn into_scrypto() {
        assert_eq!(
            ScryptoMessage::from(SUT::sample()),
            ScryptoMessage::Plaintext(ScryptoPlaintextMessage {
                message: ScryptoMessageContents::String(
                    "Hello Radix!".to_owned()
                ),
                mime_type: "text/plain".to_owned(),
            })
        );
    }

    #[test]
    fn encrypted_msg_are_not_yet_supported() {
        let dummy = EncryptedMessageV1 {
            encrypted: transaction::prelude::AesGcmPayload(vec![]),
            decryptors_by_curve: [].into(),
        };
        assert_eq!(
            TryInto::<SUT>::try_into(ScryptoMessage::Encrypted(dummy)),
            Err(CommonError::EncryptedMessagesAreNotYetSupported)
        );
    }
}
