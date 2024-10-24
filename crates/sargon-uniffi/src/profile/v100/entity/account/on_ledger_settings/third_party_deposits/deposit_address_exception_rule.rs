use crate::prelude::*;
use sargon::DepositAddressExceptionRule as InternalDepositAddressExceptionRule;

/// The exception kind for deposit address
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum DepositAddressExceptionRule {
    /// A resource can always be deposited in to the account by third-parties
    Allow,
    /// A resource can never be deposited in to the account by third-parties
    Deny,
}
