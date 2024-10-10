use crate::prelude::*;
use sargon::DepositAddressExceptionRule as InternalDepositAddressExceptionRule;

/// The exception kind for deposit address
#[derive(
    Clone,
    
    
    PartialEq,
    Eq,
    Hash,
    uniffi::Enum,
)]
pub enum DepositAddressExceptionRule {
    /// A resource can always be deposited in to the account by third-parties
    Allow,
    /// A resource can never be deposited in to the account by third-parties
    Deny,
}

impl From<DepositAddressExceptionRule> for InternalDepositAddressExceptionRule {
    fn from(value: DepositAddressExceptionRule) -> Self {
        match value {
            DepositAddressExceptionRule::Allow => InternalDepositAddressExceptionRule::Allow,
            DepositAddressExceptionRule::Deny => InternalDepositAddressExceptionRule::Deny,
        }
    }
}

impl Into<DepositAddressExceptionRule> for InternalDepositAddressExceptionRule {
    fn into(self) -> DepositAddressExceptionRule {
        match self {
            InternalDepositAddressExceptionRule::Allow => DepositAddressExceptionRule::Allow,
            InternalDepositAddressExceptionRule::Deny => DepositAddressExceptionRule::Deny,
        }
    }
}