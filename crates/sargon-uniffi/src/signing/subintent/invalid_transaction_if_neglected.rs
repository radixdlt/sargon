use crate::prelude::*;
use sargon::InvalidTransactionIfNeglected as InternalInvalidTransactionIfNeglected;
use sargon::SubintentHash as InternalSubintentHash;

type InternalInvalidTransactionIfNeglectedForSubintent =
    InternalInvalidTransactionIfNeglected<InternalSubintentHash>;

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
    ) -> InternalInvalidTransactionIfNeglectedForSubintent {
        self.clone().into()
    }
}

impl From<InternalInvalidTransactionIfNeglectedForSubintent>
    for InvalidTransactionIfNeglectedForSubintent
{
    fn from(value: InternalInvalidTransactionIfNeglectedForSubintent) -> Self {
        Self {
            subintent_hash: value.signable_id.into(),
            entities_which_would_fail_auth: value
                .entities_which_would_fail_auth
                .into_type(),
        }
    }
}

impl From<InvalidTransactionIfNeglectedForSubintent>
    for InternalInvalidTransactionIfNeglectedForSubintent
{
    fn from(value: InvalidTransactionIfNeglectedForSubintent) -> Self {
        Self {
            signable_id: value.subintent_hash.into_internal(),
            entities_which_would_fail_auth: value
                .entities_which_would_fail_auth
                .into_internal(),
        }
    }
}

decl_conversion_tests_for!(InvalidTransactionIfNeglectedForSubintent);
