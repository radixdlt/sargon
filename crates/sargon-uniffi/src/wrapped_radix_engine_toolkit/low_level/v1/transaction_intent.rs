use crate::prelude::*;
use sargon::TransactionIntent as InternalTransactionIntent;

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct TransactionIntent {
    pub header: TransactionHeader,
    pub manifest: TransactionManifest,
    pub message: Message,
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
pub fn transaction_intent_hash(
    intent: &TransactionIntent,
) -> TransactionIntentHash {
    intent.into_internal().transaction_intent_hash().into()
}

#[uniffi::export]
pub fn transaction_intent_compile(intent: &TransactionIntent) -> BagOfBytes {
    intent.into_internal().compile().into()
}
