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

impl Message {
    pub fn plain_text(message: impl AsRef<str>) -> Self {
        Self::PlainText {
            string: message.as_ref().to_owned(),
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
}
