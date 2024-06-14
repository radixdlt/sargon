use crate::prelude::*;

#[derive(Debug, Eq, PartialEq, uniffi::Record)]
pub struct SignTransactionRequest {
    pub intent: TransactionIntent
}

impl SignTransactionRequest {
    pub fn new(
        intent: TransactionIntent
    ) -> Self {
        Self {
            intent
        }
    }

    pub fn data_to_sign(&self) -> BagOfBytes {
        self.intent.compile()
    }
}

impl HasSampleValues for SignTransactionRequest {
    fn sample() -> Self {
        Self::new(
            TransactionIntent::sample()
        )
    }

    fn sample_other() -> Self {
        Self::new(
            TransactionIntent::sample_other()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SignTransactionRequest;

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