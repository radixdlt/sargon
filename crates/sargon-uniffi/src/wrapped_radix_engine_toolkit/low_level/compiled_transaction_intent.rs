use crate::prelude::*;
use sargon::CompiledTransactionIntent as InternalCompiledTransactionIntent;

#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct CompiledTransactionIntent {
    secret_magic: BagOfBytes,
}

impl From<InternalCompiledTransactionIntent> for CompiledTransactionIntent {
    fn from(value: InternalCompiledTransactionIntent) -> Self {
        Self {
            secret_magic: value.bytes().into(),
        }
    }
}

impl CompiledTransactionIntent {
    pub fn into_internal(&self) -> InternalCompiledTransactionIntent {
        self.clone().into()
    }
}

impl Into<InternalCompiledTransactionIntent> for CompiledTransactionIntent {
    fn into(self) -> InternalCompiledTransactionIntent {
        InternalCompiledTransactionIntent::new(self.secret_magic.into())
            .expect("Should always be able to compile an Intent")
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
pub fn compiled_transaction_intent_bytes(
    compiled_intent: &CompiledTransactionIntent,
) -> BagOfBytes {
    compiled_intent.into_internal().bytes().into()
}

#[uniffi::export]
pub fn compiled_transaction_intent_decompile(
    compiled_intent: &CompiledTransactionIntent,
) -> TransactionIntent {
    compiled_intent.into_internal().decompile().into()
}

decl_conversion_tests_for!(CompiledTransactionIntent);
