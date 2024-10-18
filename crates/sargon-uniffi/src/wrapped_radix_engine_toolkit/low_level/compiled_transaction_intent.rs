use crate::prelude::*;
use sargon::CompiledTransactionIntent as InternalCompiledTransactionIntent;

uniffi::custom_newtype!(CompiledTransactionIntent, BagOfBytes);

#[derive(Clone, PartialEq, Eq)]
pub struct CompiledTransactionIntent(BagOfBytes);

impl From<InternalCompiledTransactionIntent> for CompiledTransactionIntent {
    fn from(value: InternalCompiledTransactionIntent) -> Self {
        Self(value.bytes().into())
    }
}

impl CompiledTransactionIntent {
    pub fn into_internal(&self) -> InternalCompiledTransactionIntent {
        self.clone().into()
    }
}

impl Into<InternalCompiledTransactionIntent> for CompiledTransactionIntent {
    fn into(self) -> InternalCompiledTransactionIntent {
        InternalCompiledTransactionIntent::new(self.0.into())
    }
}

#[uniffi::export]
pub fn new_compiled_transaction_intent_sample() -> CompiledTransactionIntent {
    InternalCompiledTransactionIntent::sample().into()
}

#[uniffi::export]
pub fn new_compiled_transaction_intent_sample_other(
) -> CompiledTransactionIntent {
    InternalCompiledTransactionIntent::sample_other().into()
}

#[uniffi::export]
pub fn compiled_transaction_intent_decompile(
    compiled_intent: &CompiledTransactionIntent,
) -> TransactionIntent {
    compiled_intent.into_internal().decompile().into()
}
