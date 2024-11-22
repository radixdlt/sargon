use crate::prelude::*;
use sargon::PreAuthorizationStatus as InternalPreAuthorizationStatus;

/// This is the result of the Pre-Authorization status polling for a given Subintent.
#[derive(Clone, PartialEq, InternalConversion, uniffi::Enum)]
pub enum PreAuthorizationStatus {
    /// The Pre-Authorization has successfully been submitted in a transaction with the given intent hash.
    Success { intent_hash: TransactionIntentHash },

    /// The Pre-Authorization has expired before being successfully submitted.
    Expired,
}
