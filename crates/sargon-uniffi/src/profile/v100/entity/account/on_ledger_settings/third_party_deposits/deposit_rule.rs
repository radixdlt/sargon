use crate::prelude::*;
use sargon::DepositRule as InternalDepositRule;

/// The general deposit rule to apply
#[derive(
    Clone,
    
    Debug,
    PartialEq,
    Eq,
    Hash,
    uniffi::Enum,
)]
pub enum DepositRule {
    /// The account accepts **all** assets by default, except for exceptions (if any) which might not deposit/be deposited into this account.
    AcceptKnown,
    /// The account accepts **known** assets by default, except for exceptions (if any) which might not deposit/be deposited into this account. By known we mean assets this account has received in the past.
    AcceptAll,
    /// The account denies **all** assets by default, except for exceptions (if any) which might in fact deposit/be deposited into this account.
    DenyAll,
}

impl From<DepositRule> for InternalDepositRule {
    fn from(value: DepositRule) -> Self {
        match value {
            DepositRule::AcceptKnown => InternalDepositRule::AcceptKnown,
            DepositRule::AcceptAll => InternalDepositRule::AcceptAll,
            DepositRule::DenyAll => InternalDepositRule::DenyAll,
        }
    }
}

impl Into<DepositRule> for InternalDepositRule {
    fn into(self) -> DepositRule {
        match self {
            InternalDepositRule::AcceptKnown => DepositRule::AcceptKnown,
            InternalDepositRule::AcceptAll => DepositRule::AcceptAll,
            InternalDepositRule::DenyAll => DepositRule::DenyAll,
        }
    }
}

json_string_convertible!(DepositRule, "super invalid json string");

#[uniffi::export]
pub fn new_deposit_rule_sample() -> DepositRule {
    InternalDepositRule::sample().into()
}

#[uniffi::export]
pub fn new_deposit_rule_sample_other() -> DepositRule {
    InternalDepositRule::sample_other().into()
}

