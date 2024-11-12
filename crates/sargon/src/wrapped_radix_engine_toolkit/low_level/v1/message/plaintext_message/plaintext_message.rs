use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlaintextMessage {
    pub mime_type: String,
    pub message: MessageContents,
}

impl PlaintextMessage {
    pub fn as_string(&self) -> Option<String> {
        self.message.as_string()
    }
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

impl HasSampleValues for PlaintextMessage {
    fn sample() -> Self {
        Self::new("Hello Radix!")
    }

    fn sample_other() -> Self {
        Self::new("Lorem ipsum!!")
    }
}

impl PlaintextMessage {
    #[allow(unused)]
    pub(crate) fn sample_binary() -> Self {
        Self {
            mime_type: "".to_owned(),
            message: MessageContents::BinaryMessage {
                bag_of_bytes: BagOfBytes::sample(),
            },
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PlaintextMessage;

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
        assert_eq!(SUT::sample_binary().as_string(), None);
    }

    #[test]
    fn to_from_scrypto() {
        let roundtrip = |s: SUT| SUT::from(ScryptoPlaintextMessage::from(s));
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other());
    }
}
