use crate::prelude::*;
use sargon::NotarizedTransaction as InternalNotarizedTransaction;

#[derive(Debug, Clone, Eq, PartialEq,  uniffi::Record)]
pub struct NotarizedTransaction {
    signed_intent: SignedIntent,
    pub notary_signature: NotarySignature,
}

impl From<InternalNotarizedTransaction> for NotarizedTransaction {
    fn from(value: InternalNotarizedTransaction) -> Self {
        Self {
            signed_intent: value.signed_intent.into(),
            notary_signature: value.notary_signature.into(),
        }
    }
}

impl Into<InternalNotarizedTransaction> for NotarizedTransaction {
    fn into(self) -> InternalNotarizedTransaction {
        InternalNotarizedTransaction {
            signed_intent: self.signed_intent.into(),
            notary_signature: self.notary_signature.into(),
        }
    }
}

#[uniffi::export]
pub fn new_notarized_transaction_sample() -> NotarizedTransaction {
    InternalNotarizedTransaction::sample().into()
}

#[uniffi::export]
pub fn new_notarized_transaction_sample_other() -> NotarizedTransaction {
    InternalNotarizedTransaction::sample_other().into()
}

#[uniffi::export]
pub fn notarized_transaction_compile(
    notarized_transaction: &NotarizedTransaction,
) -> CompiledNotarizedIntent {
    notarized_transaction.into_internal().compile().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NotarizedTransaction;

    #[test]
    fn inequality() {
        assert_ne!(
            new_notarized_transaction_sample(),
            new_notarized_transaction_sample_other(),
        );
    }

    #[test]
    fn equality() {
        assert_eq!(
            new_notarized_transaction_sample(),
            new_notarized_transaction_sample()
        );
        assert_eq!(
            new_notarized_transaction_sample_other(),
            new_notarized_transaction_sample_other()
        );
    }

    #[test]
    fn test_compile() {
        let sut = SUT::sample();
        assert_eq!(notarized_transaction_compile(&sut), sut.compile())
    }
}
