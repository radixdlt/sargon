use crate::prelude::*;
use base64::prelude::BASE64_STANDARD_NO_PAD;
use base64::Engine;
use sargon::CompiledTransactionIntent as InternalCompiledTransactionIntent;

#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct CompiledTransactionIntent {
    /// A base-64 encoded version of the compiled intent
    secret_magic: String,
}

impl From<InternalCompiledTransactionIntent> for CompiledTransactionIntent {
    fn from(value: InternalCompiledTransactionIntent) -> Self {
        Self {
            secret_magic: BASE64_STANDARD_NO_PAD.encode(value.bytes().bytes()),
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
        let decoded = BASE64_STANDARD_NO_PAD
            .decode(self.secret_magic)
            .expect("Should always be able to decode base-64 encoded bytes");

        InternalCompiledTransactionIntent::new(sargon::BagOfBytes::from(
            decoded,
        ))
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
