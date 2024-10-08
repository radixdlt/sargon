use crate::prelude::*;
use sargon::TransactionIntent as InternalTransactionIntent;

#[derive(Clone, PartialEq, Eq,  uniffi::Record)]
#[debug("header:\n{:?}\n\nmessage:\n{:?}\n\nmanifest:\n{}\n\n", self.header, self.message, self.manifest.instructions_string())]
pub struct TransactionIntent {
    pub header: TransactionHeader,
    manifest: TransactionManifest,
    pub message: Message,
}

impl From<InternalTransactionIntent> for TransactionIntent {
    fn from(value: InternalTransactionIntent) -> Self {
        Self {
            header: value.header.into(),
            manifest: value.manifest.into(),
            message: value.message.into(),
        }
    }
}

impl Into<InternalTransactionIntent> for TransactionIntent {
    fn into(self) -> InternalTransactionIntent {
        InternalTransactionIntent {
            header: self.header.into(),
            manifest: self.manifest.into(),
            message: self.message.into(),
        }
    }
}

#[uniffi::export]
pub fn new_transaction_intent_sample() -> TransactionIntent {
    InternalTransactionIntent::sample().into()
}

#[uniffi::export]
pub fn new_transaction_intent_sample_other() -> TransactionIntent {
    InternalTransactionIntent::sample_other().into()
}

#[uniffi::export]
pub fn transaction_intent_hash(intent: &TransactionIntent) -> IntentHash {
    intent.into::<InternalTransactionIntent>().intent_hash().into()
}

#[uniffi::export]
pub fn transaction_intent_compile(intent: &TransactionIntent) -> BagOfBytes {
    intent.into::<InternalTransactionIntent>().compile().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionIntent;

    #[test]
    fn inequality() {
        assert_ne!(
            new_transaction_intent_sample(),
            new_transaction_intent_sample_other(),
        );
    }

    #[test]
    fn equality() {
        assert_eq!(
            new_transaction_intent_sample(),
            new_transaction_intent_sample()
        );
        assert_eq!(
            new_transaction_intent_sample_other(),
            new_transaction_intent_sample_other()
        );
    }

    #[test]
    fn test_transaction_intent_hash() {
        let sut = SUT::sample();
        assert_eq!(transaction_intent_hash(&sut), sut.intent_hash())
    }

    #[test]
    fn test_transaction_intent_compile() {
        let sut = SUT::sample();
        assert_eq!(transaction_intent_compile(&sut), sut.compile())
    }
}
