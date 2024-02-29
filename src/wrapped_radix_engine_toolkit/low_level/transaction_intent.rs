use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct TransactionIntent {
    pub header: TransactionHeader,
    pub manifest: TransactionManifest,
    pub message: Message,
}

impl TransactionIntent {
    pub fn new(
        header: TransactionHeader,
        manifest: TransactionManifest,
        message: Message,
    ) -> Self {
        Self {
            header,
            manifest,
            message,
        }
    }

    pub fn intent_hash(&self) -> TransactionHash {
        todo!()
    }

    pub fn compile(&self) -> Result<BagOfBytes> {
        todo!()
    }
}

impl HasSampleValues for TransactionIntent {
    fn sample() -> Self {
        Self::new(
            TransactionHeader::sample(),
            TransactionManifest::sample(),
            Message::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            TransactionHeader::sample_other(),
            TransactionManifest::sample_other(),
            Message::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionIntent;

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
    #[should_panic(expected = "not yet implemented")]
    fn intent_hash() {
        _ = SUT::sample().intent_hash()
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn compile() {
        _ = SUT::sample().compile()
    }
}
