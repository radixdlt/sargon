use crate::prelude::*;

#[uniffi::export]
pub fn signed_intent_hash(
    signed_intent: &SignedIntent,
) -> SignedTransactionIntentHash {
    signed_intent.hash()
}

#[uniffi::export]
pub fn new_signed_intent_sample() -> SignedIntent {
    SignedIntent::sample()
}

#[uniffi::export]
pub fn new_signed_intent_sample_other() -> SignedIntent {
    SignedIntent::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SignedIntent;

    #[test]
    fn inequality() {
        assert_ne!(
            new_signed_intent_sample(),
            new_signed_intent_sample_other()
        );
    }

    #[test]
    fn equality() {
        assert_eq!(new_signed_intent_sample(), new_signed_intent_sample());
    }

    #[test]
    fn hash() {
        assert_eq!(signed_intent_hash(&SUT::sample()), SUT::sample().hash())
    }
}
