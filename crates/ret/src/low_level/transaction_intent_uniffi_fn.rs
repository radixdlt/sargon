use crate::prelude::*;

#[uniffi::export]
pub fn new_transaction_intent_sample() -> TransactionIntent {
    TransactionIntent::sample()
}

#[uniffi::export]
pub fn new_transaction_intent_sample_other() -> TransactionIntent {
    TransactionIntent::sample_other()
}

#[uniffi::export]
pub fn transaction_intent_hash(intent: &TransactionIntent) -> IntentHash {
    intent.intent_hash()
}

#[uniffi::export]
pub fn transaction_intent_compile(intent: &TransactionIntent) -> BagOfBytes {
    intent.compile()
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
