use crate::prelude::*;

/// We explicitly mark content as either String or Bytes - this distinguishes (along with the mime type)
/// whether the message is intended to be displayable as text, or not.
///
/// This data model ensures that messages intended to be displayable as text are valid unicode strings.
#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumAsInner)]
pub enum MessageContents {
    StringMessage { string: String },
    BinaryMessage { bag_of_bytes: BagOfBytes },
}

impl MessageContents {
    pub fn as_string(&self) -> Option<String> {
        match self {
            MessageContents::StringMessage { string } => {
                Some(string.to_owned())
            }
            MessageContents::BinaryMessage { bag_of_bytes: _ } => None,
        }
    }
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

impl HasSampleValues for MessageContents {
    fn sample() -> Self {
        Self::StringMessage {
            string: "Hello Radix!".to_owned(),
        }
    }

    fn sample_other() -> Self {
        Self::BinaryMessage {
            bag_of_bytes: BagOfBytes::from_hex("deadbeef").unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = MessageContents;

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
    fn as_string() {
        assert_eq!(SUT::sample().as_string(), Some("Hello Radix!".to_owned()));
        assert_eq!(SUT::sample_other().as_string(), None);
    }

    #[test]
    fn to_from_scrypto() {
        let roundtrip = |s: SUT| SUT::from(ScryptoMessageContents::from(s));
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other());
    }
}
