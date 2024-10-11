use crate::prelude::*;
use sargon::TransactionIntent as InternalTransactionIntent;

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct TransactionIntent {
    pub header: TransactionHeader,
    pub manifest: TransactionManifest,
    pub message: Message,
}

impl From<InternalTransactionIntent> for TransactionIntent {
    fn from(value: InternalTransactionIntent) -> Self {
        Self {
            header: value.header.into(),
            manifest: value.manifest.into(),
            message: value.message.into(),
        }
    }
}

impl Into<InternalTransactionIntent> for TransactionIntent {
    fn into(self) -> InternalTransactionIntent {
        InternalTransactionIntent {
            header: self.header.into(),
            manifest: self.manifest.into(),
            message: self.message.into(),
        }
    }
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
pub fn transaction_intent_hash(intent: &TransactionIntent) -> IntentHash {
    intent.into_internal().intent_hash().into()
}

#[uniffi::export]
pub fn transaction_intent_compile(intent: &TransactionIntent) -> BagOfBytes {
    intent.into_internal().compile().into()
}
