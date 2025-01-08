use crate::prelude::*;
use sargon::ThirdPartyDeposits as InternalThirdPartyDeposits;

/// Controls the ability of third-parties to deposit into a certain account, this is
/// useful for users who wish to not be able to receive airdrops.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct ThirdPartyDeposits {
    /// Controls the ability of third-parties to deposit into this account
    pub deposit_rule: DepositRule,

    /// Denies or allows third-party deposits of specific assets by ignoring the `depositMode`
    /// `nil` means that the account was "recovered" using "Account Recovery Scan" features,
    /// thus the value is unknown.
    pub assets_exception_list: Option<Vec<AssetException>>,

    /// Allows certain third-party depositors to deposit assets freely.
    /// Note: There is no `deny` counterpart for this.
    /// `nil` means that the account was "recovered" using "Account Recovery Scan" features,
    /// thus the value is unknown.
    pub depositors_allow_list: Option<Vec<ResourceOrNonFungible>>,
}

#[uniffi::export]
pub fn new_third_party_deposits_sample() -> ThirdPartyDeposits {
    InternalThirdPartyDeposits::sample().into()
}

#[uniffi::export]
pub fn new_third_party_deposits_sample_other() -> ThirdPartyDeposits {
    InternalThirdPartyDeposits::sample_other().into()
}

#[uniffi::export]
pub fn new_third_party_deposits_default() -> ThirdPartyDeposits {
    InternalThirdPartyDeposits::default().into()
}
