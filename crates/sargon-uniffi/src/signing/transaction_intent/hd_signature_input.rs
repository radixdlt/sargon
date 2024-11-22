use crate::prelude::*;
use sargon::TransactionIntentHash as InternalTransactionIntentHash;
type InternalHDSignatureInputForTransactionIntent = sargon::HDSignatureInput<InternalTransactionIntentHash>;

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct HDSignatureInputForTransactionIntent {
    /// Hash which was signed.
    pub intent_hash: TransactionIntentHash,

    /// The account or identity address of the entity which signed the hash,
    /// with expected public key and with derivation path to derive PrivateKey
    /// with.
    pub owned_factor_instance: OwnedFactorInstance,
}

impl HDSignatureInputForTransactionIntent {
    pub fn into_internal(&self) -> InternalHDSignatureInputForTransactionIntent {
        self.clone().into()
    }
}

impl From<InternalHDSignatureInputForTransactionIntent> for HDSignatureInputForTransactionIntent {
    fn from(value: InternalHDSignatureInputForTransactionIntent) -> Self {
        Self {
            intent_hash: value.payload_id.into(),
            owned_factor_instance: value.owned_factor_instance.into(),
        }
    }
}

impl From<HDSignatureInputForTransactionIntent> for InternalHDSignatureInputForTransactionIntent {
    fn from(value: HDSignatureInputForTransactionIntent) -> Self {
        Self::new(
            value.intent_hash.into_internal(),
            value.owned_factor_instance.into_internal(),
        )
    }
}