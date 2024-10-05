use crate::prelude::*;

/// The preferences the user has configured off-ledger for a given `AuthorizedDapp`.
/// Allows users, for example, to hide direct deposit claims for a given Dapp.
#[derive(
    Clone,
    PartialEq,
    Eq,
    Debug,
    Hash,
    derive_more::Display,
    Default,
    uniffi::Record,
)]
pub struct AuthorizedDappPreferences {
    pub deposits: AuthorizedDappPreferenceDeposits,
}