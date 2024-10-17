use crate::prelude::*;
use sargon::AuthorizedDappPreferences as InternalAuthorizedDappPreferences;

/// The preferences the user has configured off-ledger for a given `AuthorizedDapp`.
/// Allows users, for example, to hide direct deposit claims for a given Dapp.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct AuthorizedDappPreferences {
    pub deposits: AuthorizedDappPreferenceDeposits,
}
