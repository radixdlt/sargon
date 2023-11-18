use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::{
    asset_exception::AssetException, deposit_rule::DepositRule, depositor_address::DepositorAddress,
};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct ThirdPartyDeposits {
    /// Controls the ability of third-parties to deposit into this account
    pub deposit_rule: DepositRule,

    /// Denies or allows third-party deposits of specific assets by ignoring the `depositMode`
    pub assets_exception_list: BTreeSet<AssetException>,

    /// Allows certain third-party depositors to deposit assets freely.
    /// Note: There is no `deny` counterpart for this.
    pub depositors_allow_list: BTreeSet<DepositorAddress>,
}

impl ThirdPartyDeposits {
    pub fn with_rule_and_lists(
        deposit_rule: DepositRule,
        assets_exception_list: BTreeSet<AssetException>,
        depositors_allow_list: BTreeSet<DepositorAddress>,
    ) -> Self {
        Self {
            deposit_rule,
            assets_exception_list,
            depositors_allow_list,
        }
    }
}
