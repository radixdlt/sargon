use crate::prelude::*;

#[uniffi::export]
pub fn new_intent_hash_sample() -> TransactionIntentHash {
    TransactionIntentHash::sample()
}

#[uniffi::export]
pub fn new_intent_hash_sample_other() -> TransactionIntentHash {
    TransactionIntentHash::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionIntentHash;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_intent_hash_sample(),
                new_intent_hash_sample_other(),
                // duplicates should get removed
                new_intent_hash_sample(),
                new_intent_hash_sample_other(),
            ])
            .len(),
            2
        );
    }
}
