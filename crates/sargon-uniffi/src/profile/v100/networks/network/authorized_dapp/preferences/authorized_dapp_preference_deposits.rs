use crate::prelude::*;
use sargon::AuthorizedDappPreferenceDeposits as InternalAuthorizedDappPreferenceDeposits;

/// Indicates whether the Wallet should show direct deposit claims for the given Dapp.
#[derive(
    Clone,
    
    
    PartialEq,
    Eq,
    Hash,
    uniffi::Enum,
)]
pub enum AuthorizedDappPreferenceDeposits {
    Hidden,
    Visible,
}

impl From<InternalAuthorizedDappPreferenceDeposits> for AuthorizedDappPreferenceDeposits {
    fn from(value: InternalAuthorizedDappPreferenceDeposits) -> Self {
        match value {
            InternalAuthorizedDappPreferenceDeposits::Hidden => AuthorizedDappPreferenceDeposits::Hidden,
            InternalAuthorizedDappPreferenceDeposits::Visible => AuthorizedDappPreferenceDeposits::Visible,
        }
    }
}

impl Into<InternalAuthorizedDappPreferenceDeposits> for AuthorizedDappPreferenceDeposits {
    fn into(self) -> InternalAuthorizedDappPreferenceDeposits {
        match self {
            AuthorizedDappPreferenceDeposits::Hidden => InternalAuthorizedDappPreferenceDeposits::Hidden,
            AuthorizedDappPreferenceDeposits::Visible => InternalAuthorizedDappPreferenceDeposits::Visible,
        }
    }
}