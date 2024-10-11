use crate::prelude::*;
use sargon::AuthorizedDappPreferenceDeposits as InternalAuthorizedDappPreferenceDeposits;

/// Indicates whether the Wallet should show direct deposit claims for the given Dapp.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
pub enum AuthorizedDappPreferenceDeposits {
    Hidden,
    Visible,
}
