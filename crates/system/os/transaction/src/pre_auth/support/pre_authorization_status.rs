use crate::prelude::*;

/// This is the result of the Pre-Authorization status polling for a given Subintent.
#[derive(Clone, Debug, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum PreAuthorizationStatus {
    /// The Pre-Authorization has successfully been submitted in a transaction with the given intent hash.
    Success { intent_hash: TransactionIntentHash },

    /// The Pre-Authorization has expired before being successfully submitted.
    Expired,
}
