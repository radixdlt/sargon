use crate::prelude::*;

#[uniffi::export]
pub fn new_signed_intent_hash_sample() -> SignedTransactionIntentHash {
    SignedTransactionIntentHash::sample()
}

#[uniffi::export]
pub fn new_signed_intent_hash_sample_other() -> SignedTransactionIntentHash {
    SignedTransactionIntentHash::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SignedTransactionIntentHash;

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

    #[test]
    fn test_new_signed_intent_hash() {
        assert_eq!(new_signed_transaction_intent_hash_from_string("signedintent_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6sxsk6nl".to_owned()).unwrap(), SUT::sample());
    }
}
