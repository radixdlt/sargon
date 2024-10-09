use crate::prelude::*;
use sargon::SignedIntent as InternalSignedIntent;

#[derive(Clone, Debug, PartialEq, Eq,  uniffi::Record)]
pub struct SignedIntent {
    intent: TransactionIntent,
    pub intent_signatures: IntentSignatures,
}

impl From<InternalSignedIntent> for SignedIntent {
    fn from(value: InternalSignedIntent) -> Self {
        Self {
            intent: value.intent.into(),
            intent_signatures: value.intent_signatures.into(),
        }
    }
}

impl Into<InternalSignedIntent> for SignedIntent {
    fn into(self) -> InternalSignedIntent {
        InternalSignedIntent {
            intent: self.intent.into(),
            intent_signatures: self.intent_signatures.into(),
        }
    }
}

#[uniffi::export]
pub fn signed_intent_hash(signed_intent: &SignedIntent) -> SignedIntentHash {
    signed_intent.into::<InternalSignedIntent>().hash().into()
}

#[uniffi::export]
pub fn new_signed_intent_sample() -> SignedIntent {
    InternalSignedIntent::sample().into()
}

#[uniffi::export]
pub fn new_signed_intent_sample_other() -> SignedIntent {
    InternalSignedIntent::sample_other().into()
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
