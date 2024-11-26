use crate::prelude::*;
use sargon::InvalidTransactionIfNeglected as InternalInvalidTransactionIfNeglected;
use sargon::SubintentHash as InternalSubintentHash;

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct InvalidTransactionIfNeglectedForSubintent {
    /// The subintent hash of the subintent which would be invalid if a
    /// certain factor source would be neglected, either if user
    /// explicitly skipped it or implicitly neglected due to failure.
    pub subintent_hash: SubintentHash,

    /// The entities in the transaction which would fail auth.
    pub entities_which_would_fail_auth: Vec<AddressOfAccountOrPersona>,
}

impl InvalidTransactionIfNeglectedForSubintent {
    pub fn into_internal(
        &self,
    ) -> InternalInvalidTransactionIfNeglected<InternalSubintentHash> {
        self.clone().into()
    }
}

impl From<InternalInvalidTransactionIfNeglected<InternalSubintentHash>>
    for InvalidTransactionIfNeglectedForSubintent
{
    fn from(
        value: InternalInvalidTransactionIfNeglected<InternalSubintentHash>,
    ) -> Self {
        Self {
            subintent_hash: value.signable_id.into(),
            entities_which_would_fail_auth: value
                .entities_which_would_fail_auth
                .iter()
                .map(|addr| addr.clone().into())
                .collect(),
        }
    }
}

impl From<InvalidTransactionIfNeglectedForSubintent>
    for InternalInvalidTransactionIfNeglected<InternalSubintentHash>
{
    fn from(value: InvalidTransactionIfNeglectedForSubintent) -> Self {
        Self {
            signable_id: value.subintent_hash.into_internal(),
            entities_which_would_fail_auth: value
                .entities_which_would_fail_auth
                .iter()
                .map(|addr| addr.into_internal())
                .collect(),
        }
    }
}
