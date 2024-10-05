use crate::prelude::*;

/// Controls the ability of third-parties to deposit into a certain account, this is
/// useful for users who wish to not be able to receive airdrops.
#[derive(
    Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
)]
pub struct ThirdPartyDeposits {
    /// Controls the ability of third-parties to deposit into this account
    pub deposit_rule: DepositRule,

    /// Denies or allows third-party deposits of specific assets by ignoring the `depositMode`
    /// `nil` means that the account was "recovered" using "Account Recovery Scan" features,
    /// thus the value is unknown.
    pub assets_exception_list: Option<AssetsExceptionList>,

    /// Allows certain third-party depositors to deposit assets freely.
    /// Note: There is no `deny` counterpart for this.
    /// `nil` means that the account was "recovered" using "Account Recovery Scan" features,
    /// thus the value is unknown.
    pub depositors_allow_list: Option<DepositorsAllowList>,
}

#[uniffi::export]
pub fn new_third_party_deposits_sample() -> ThirdPartyDeposits {
    ThirdPartyDeposits::sample()
}

#[uniffi::export]
pub fn new_third_party_deposits_sample_other() -> ThirdPartyDeposits {
    ThirdPartyDeposits::sample_other()
}

#[uniffi::export]
pub fn new_third_party_deposits_default() -> ThirdPartyDeposits {
    ThirdPartyDeposits::default()
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ThirdPartyDeposits;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_third_party_deposits_sample(),
                new_third_party_deposits_sample_other(),
                // duplicates should get removed
                new_third_party_deposits_sample(),
                new_third_party_deposits_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(new_third_party_deposits_default(), SUT::default())
    }
}
