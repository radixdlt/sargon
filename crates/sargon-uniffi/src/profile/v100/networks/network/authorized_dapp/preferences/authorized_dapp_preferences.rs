use crate::prelude::*;
use sargon::AuthorizedDappPreferences as InternalAuthorizedDappPreferences;

/// The preferences the user has configured off-ledger for a given `AuthorizedDapp`.
/// Allows users, for example, to hide direct deposit claims for a given Dapp.
#[derive(
    Clone,
    PartialEq,
    Eq,
    Debug,
    Hash,
     uniffi::Record,
)]
pub struct AuthorizedDappPreferences {
    pub deposits: AuthorizedDappPreferenceDeposits,
}

impl From<InternalAuthorizedDappPreferences> for AuthorizedDappPreferences {
    fn from(value: InternalAuthorizedDappPreferences) -> Self {
        Self {
            deposits: value.deposits.into(),
        }
    }
}

impl Into<InternalAuthorizedDappPreferences> for AuthorizedDappPreferences {
    fn into(self) -> InternalAuthorizedDappPreferences {
        InternalAuthorizedDappPreferences {
            deposits: self.deposits.into(),
        }
    }
}