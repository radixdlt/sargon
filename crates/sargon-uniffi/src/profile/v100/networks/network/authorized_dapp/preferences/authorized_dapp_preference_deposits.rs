use crate::prelude::*;

/// Indicates whether the Wallet should show direct deposit claims for the given Dapp.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    enum_iterator::Sequence,
    derive_more::Display,
    uniffi::Enum,
)]
pub enum AuthorizedDappPreferenceDeposits {
    Hidden,
    Visible,
}