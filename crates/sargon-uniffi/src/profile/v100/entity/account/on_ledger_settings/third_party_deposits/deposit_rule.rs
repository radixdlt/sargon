use crate::prelude::*;

/// The general deposit rule to apply
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
pub enum DepositRule {
    /// The account accepts **all** assets by default, except for exceptions (if any) which might not deposit/be deposited into this account.
    AcceptKnown,
    /// The account accepts **known** assets by default, except for exceptions (if any) which might not deposit/be deposited into this account. By known we mean assets this account has received in the past.
    AcceptAll,
    /// The account denies **all** assets by default, except for exceptions (if any) which might in fact deposit/be deposited into this account.
    DenyAll,
}

use crate::prelude::*;

json_string_convertible!(DepositRule, "super invalid json string");

#[uniffi::export]
pub fn new_deposit_rule_sample() -> DepositRule {
    DepositRule::sample()
}

#[uniffi::export]
pub fn new_deposit_rule_sample_other() -> DepositRule {
    DepositRule::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DepositRule;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_deposit_rule_sample(),
                new_deposit_rule_sample_other(),
                // duplicates should get removed
                new_deposit_rule_sample(),
                new_deposit_rule_sample_other(),
            ])
            .len(),
            2
        );
    }
}
