use crate::prelude::*;
use sargon::CompiledTransactionIntent as InternalCompiledTransactionIntent;

uniffi::custom_newtype!(CompiledTransactionIntent, BagOfBytes);

#[derive(Clone, PartialEq, Eq, InternalConversion)]
pub struct CompiledTransactionIntent(BagOfBytes);

#[uniffi::export]
pub fn new_compiled_transaction_intent_sample() -> CompiledTransactionIntent {
    InternalCompiledTransactionIntent::sample().into()
}

#[uniffi::export]
pub fn new_compiled_transaction_intent_sample_other() -> CompiledTransactionIntent {
    InternalCompiledTransactionIntent::sample_other().into()
}

#[uniffi::export]
pub fn compiled_transaction_intent_decompile(compiled_intent: &CompiledTransactionIntent) -> TransactionIntent {
    compiled_intent.into_internal().decompile().into()
}