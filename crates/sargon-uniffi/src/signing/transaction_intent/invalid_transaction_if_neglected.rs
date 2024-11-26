use crate::prelude::*;
use sargon::InvalidTransactionIfNeglected as InternalInvalidTransactionIfNeglected;
use sargon::TransactionIntentHash as InternalTransactionIntentHash;

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct InvalidTransactionIfNeglectedForTransactionIntent {
    /// The intent hash of the transaction which would be invalid if a
    /// certain factor source would be neglected, either if user
    /// explicitly skipped it or implicitly neglected due to failure.
    pub intent_hash: TransactionIntentHash,

    /// The entities in the transaction which would fail auth.
    pub entities_which_would_fail_auth: Vec<AddressOfAccountOrPersona>,
}

impl InvalidTransactionIfNeglectedForTransactionIntent {
    pub fn into_internal(
        &self,
    ) -> InternalInvalidTransactionIfNeglected<InternalTransactionIntentHash>
    {
        self.clone().into()
    }
}

impl From<InternalInvalidTransactionIfNeglected<InternalTransactionIntentHash>>
    for InvalidTransactionIfNeglectedForTransactionIntent
{
    fn from(
        value: InternalInvalidTransactionIfNeglected<
            InternalTransactionIntentHash,
        >,
    ) -> Self {
        Self {
            intent_hash: value.signable_id.into(),
            entities_which_would_fail_auth: value
                .entities_which_would_fail_auth
                .iter()
                .map(|addr| addr.clone().into())
                .collect(),
        }
    }
}

impl From<InvalidTransactionIfNeglectedForTransactionIntent>
    for InternalInvalidTransactionIfNeglected<InternalTransactionIntentHash>
{
    fn from(value: InvalidTransactionIfNeglectedForTransactionIntent) -> Self {
        Self {
            signable_id: value.intent_hash.into_internal(),
            entities_which_would_fail_auth: value
                .entities_which_would_fail_auth
                .iter()
                .map(|addr| addr.into_internal())
                .collect(),
        }
    }
}
