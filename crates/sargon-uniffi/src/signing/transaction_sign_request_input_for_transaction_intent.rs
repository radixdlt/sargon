use crate::prelude::*;
use sargon::TransactionSignRequestInput as InternalTransactionSignRequestInput;
use sargon::TransactionIntent as InternalTransactionIntent;

type InternalTransactionSignRequestInputForTransactionIntent =
    InternalTransactionSignRequestInput<InternalTransactionIntent>;

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct TransactionSignRequestInputForTransactionIntent {
    /// Compiled Intent
    pub payload: CompiledTransactionIntent,

    /// ID of factor to use to sign
    pub factor_source_id: FactorSourceIDFromHash,

    /// The derivation paths to use to derive the private keys to sign with. The
    /// `factor_source_id` of each item must match `factor_source_id`.
    pub owned_factor_instances: Vec<OwnedFactorInstance>,
}

impl TransactionSignRequestInputForTransactionIntent {
    pub fn into_internal(&self) -> InternalTransactionSignRequestInputForTransactionIntent {
        self.clone().into()
    }
}

impl From<InternalTransactionSignRequestInputForTransactionIntent> for TransactionSignRequestInputForTransactionIntent {
    fn from(value: InternalTransactionSignRequestInputForTransactionIntent) -> Self {
        Self {
            payload: value.payload.into(),
            factor_source_id: value.factor_source_id.into(),
            owned_factor_instances: value.owned_factor_instances
                .iter()
                .map(|instance| instance.clone().into())
                .collect(),
        }
    }
}

impl From<TransactionSignRequestInputForTransactionIntent> for InternalTransactionSignRequestInputForTransactionIntent {
    fn from(value: TransactionSignRequestInputForTransactionIntent) -> Self {
        InternalTransactionSignRequestInputForTransactionIntent {
            payload: value.payload.into_internal(),
            factor_source_id: value.factor_source_id.into_internal(),
            owned_factor_instances: value.owned_factor_instances.into_internal(),
        }
    }
}

#[uniffi::export]
pub fn new_transaction_sign_request_input_for_transaction_intent_sample() -> TransactionSignRequestInputForTransactionIntent {
    InternalTransactionSignRequestInputForTransactionIntent::sample().into()
}

#[uniffi::export]
pub fn new_transaction_sign_request_input_for_transaction_intent_sample_other() -> TransactionSignRequestInputForTransactionIntent {
    InternalTransactionSignRequestInputForTransactionIntent::sample_other().into()
}