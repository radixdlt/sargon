use crate::prelude::*;
use sargon::Subintent as InternalSubintent;
use sargon::TransactionSignRequestInput as InternalTransactionSignRequestInput;

type InternalTransactionSignRequestInputForSubintent =
    InternalTransactionSignRequestInput<InternalSubintent>;

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct TransactionSignRequestInputForSubintent {
    /// Compiled sub intent
    pub payload: CompiledSubintent,

    /// ID of factor to use to sign
    pub factor_source_id: FactorSourceIDFromHash,

    /// The derivation paths to use to derive the private keys to sign with. The
    /// `factor_source_id` of each item must match `factor_source_id`.
    pub owned_factor_instances: Vec<OwnedFactorInstance>,
}

impl TransactionSignRequestInputForSubintent {
    pub fn into_internal(
        &self,
    ) -> InternalTransactionSignRequestInputForSubintent {
        self.clone().into()
    }
}

impl From<InternalTransactionSignRequestInputForSubintent>
    for TransactionSignRequestInputForSubintent
{
    fn from(value: InternalTransactionSignRequestInputForSubintent) -> Self {
        Self {
            payload: value.payload.into(),
            factor_source_id: value.factor_source_id.into(),
            owned_factor_instances: value
                .owned_factor_instances
                .iter()
                .map(|instance| instance.clone().into())
                .collect(),
        }
    }
}

impl From<TransactionSignRequestInputForSubintent>
    for InternalTransactionSignRequestInputForSubintent
{
    fn from(value: TransactionSignRequestInputForSubintent) -> Self {
        InternalTransactionSignRequestInputForSubintent {
            payload: value.payload.into_internal(),
            factor_source_id: value.factor_source_id.into_internal(),
            owned_factor_instances: value
                .owned_factor_instances
                .into_internal(),
        }
    }
}

#[uniffi::export]
pub fn new_transaction_sign_request_input_for_subintent_sample(
) -> TransactionSignRequestInputForSubintent {
    InternalTransactionSignRequestInputForSubintent::sample().into()
}

#[uniffi::export]
pub fn new_transaction_sign_request_input_for_subintent_sample_other(
) -> TransactionSignRequestInputForSubintent {
    InternalTransactionSignRequestInputForSubintent::sample_other().into()
}
