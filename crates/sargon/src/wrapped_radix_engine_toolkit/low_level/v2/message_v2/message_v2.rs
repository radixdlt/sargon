use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, EnumAsInner, Eq, Hash, uniffi::Enum)]
pub enum MessageV2 {
    PlainText { plaintext: PlaintextMessage },
    None,
}

impl MessageV2 {
    pub fn plain_text(message: impl AsRef<str>) -> Self {
        Self::PlainText {
            plaintext: PlaintextMessage::new(message),
        }
    }
}

impl MessageV2 {
    pub fn as_plaintext(&self) -> Option<String> {
        match self {
            MessageV2::PlainText { plaintext } => plaintext.as_string(),
            MessageV2::None => None,
        }
    }
}

impl From<MessageV2> for ScryptoMessageV2 {
    fn from(value: MessageV2) -> Self {
        match value {
            MessageV2::PlainText { plaintext } => {
                ScryptoMessageV2::Plaintext(plaintext.into())
            }
            MessageV2::None => ScryptoMessageV2::None,
        }
    }
}

impl TryFrom<ScryptoMessageV2> for MessageV2 {
    type Error = crate::CommonError;

    fn try_from(value: ScryptoMessageV2) -> Result<Self, Self::Error> {
        match value {
            ScryptoMessageV2::None => Ok(Self::None),
            ScryptoMessageV2::Plaintext(p) => Ok(Self::PlainText {
                plaintext: p.into(),
            }),
            ScryptoMessageV2::Encrypted(_) => {
                Err(CommonError::EncryptedMessagesAreNotYetSupported)
            }
        }
    }
}

impl HasSampleValues for MessageV2 {
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

    use radix_transactions::model::{EncryptedMessageV1, EncryptedMessageV2};

    #[allow(clippy::upper_case_acronyms)]
    type SUT = MessageV2;

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
            |s: SUT| SUT::try_from(ScryptoMessageV2::from(s)).unwrap();
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
            ScryptoMessageV2::from(SUT::sample()),
            ScryptoMessageV2::Plaintext(ScryptoPlaintextMessage {
                message: ScryptoMessageContents::String(
                    "Hello Radix!".to_owned()
                ),
                mime_type: "text/plain".to_owned(),
            })
        );
    }

    #[test]
    fn encrypted_msg_are_not_yet_supported() {
        let dummy = EncryptedMessageV2 {
            encrypted: radix_transactions::prelude::AesGcmPayload(vec![]),
            decryptors_by_curve: [].into(),
        };
        assert_eq!(
            TryInto::<SUT>::try_into(ScryptoMessageV2::Encrypted(dummy)),
            Err(CommonError::EncryptedMessagesAreNotYetSupported)
        );
    }
}
