use crate::prelude::*;

#[uniffi::export]
pub fn new_signed_intent_hash_sample() -> SignedIntentHash {
    SignedIntentHash::sample()
}

#[uniffi::export]
pub fn new_signed_intent_hash_sample_other() -> SignedIntentHash {
    SignedIntentHash::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SignedIntentHash;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_signed_intent_hash_sample(),
                new_signed_intent_hash_sample_other(),
                // duplicates should get removed
                new_signed_intent_hash_sample(),
                new_signed_intent_hash_sample_other(),
            ])
            .len(),
            2
        );
    }
}
