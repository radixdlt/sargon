use crate::prelude::*;
use sargon::AuthorizedDappPreferenceDeposits as InternalAuthorizedDappPreferenceDeposits;

/// Indicates whether the Wallet should show direct deposit claims for the given Dapp.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum AuthorizedDappPreferenceDeposits {
    Hidden,
    Visible,
}
