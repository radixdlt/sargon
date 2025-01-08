use crate::prelude::*;
use sargon::NotarizedTransaction as InternalNotarizedTransaction;

#[derive(Clone, Eq, PartialEq, InternalConversion, uniffi::Record)]
pub struct NotarizedTransaction {
    signed_intent: SignedIntent,
    pub notary_signature: NotarySignature,
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
