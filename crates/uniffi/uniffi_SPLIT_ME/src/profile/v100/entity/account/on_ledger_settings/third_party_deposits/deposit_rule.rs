use crate::prelude::*;
use sargon::DepositRule as InternalDepositRule;

/// The general deposit rule to apply
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum DepositRule {
    /// The account accepts **all** assets by default, except for exceptions (if any) which might not deposit/be deposited into this account.
    AcceptKnown,
    /// The account accepts **known** assets by default, except for exceptions (if any) which might not deposit/be deposited into this account. By known we mean assets this account has received in the past.
    AcceptAll,
    /// The account denies **all** assets by default, except for exceptions (if any) which might in fact deposit/be deposited into this account.
    DenyAll,
}

json_string_convertible!(DepositRule);

#[uniffi::export]
pub fn new_deposit_rule_sample() -> DepositRule {
    InternalDepositRule::sample().into()
}

#[uniffi::export]
pub fn new_deposit_rule_sample_other() -> DepositRule {
    InternalDepositRule::sample_other().into()
}
