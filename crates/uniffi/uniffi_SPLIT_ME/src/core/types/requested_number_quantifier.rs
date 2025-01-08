use crate::prelude::*;
use sargon::RequestedNumberQuantifier as InternalRequestedNumberQuantifier;

/// A quantifier of a quantity, either `atLeast` or `exactly`, as in
/// "I want AT LEAST 3" or "I want EXACTLY 10".
///
/// This is typically sent by a Dapp when requesting access to accounts
/// or PersonaData.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum RequestedNumberQuantifier {
    /// (Request access to) *exactly* N many of something, where quantity `N` is
    /// not part of this enum, e.g. "I want EXACTLY 2 accounts"
    Exactly,

    /// (Request access to) *at least* N many of something, where quantity `N` is
    /// not part of this enum, e.g. "I want AT LEAST 3 accounts"
    AtLeast,
}
