use std::{
    cell::{Cell, RefCell},
    collections::BTreeSet,
};

use serde::{Deserialize, Serialize};

use super::{
    asset_exception::AssetException, deposit_rule::DepositRule, depositor_address::DepositorAddress,
};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct ThirdPartyDeposits {
    /// Controls the ability of third-parties to deposit into this account
    deposit_rule: Cell<DepositRule>,

    /// Denies or allows third-party deposits of specific assets by ignoring the `depositMode`
    assets_exception_list: RefCell<BTreeSet<AssetException>>,

    /// Allows certain third-party depositors to deposit assets freely.
    /// Note: There is no `deny` counterpart for this.
    depositors_allow_list: RefCell<BTreeSet<DepositorAddress>>,
}

impl ThirdPartyDeposits {
    pub fn with_rule_and_lists(
        deposit_rule: DepositRule,
        assets_exception_list: BTreeSet<AssetException>,
        depositors_allow_list: BTreeSet<DepositorAddress>,
    ) -> Self {
        Self {
            deposit_rule: Cell::new(deposit_rule),
            assets_exception_list: RefCell::new(assets_exception_list),
            depositors_allow_list: RefCell::new(depositors_allow_list),
        }
    }
}

// Getters
impl ThirdPartyDeposits {
    pub fn get_deposit_rule(&self) -> DepositRule {
        self.deposit_rule.get().clone()
    }

    pub fn get_assets_exception_list(&self) -> BTreeSet<AssetException> {
        self.assets_exception_list.borrow().clone()
    }

    pub fn get_depositors_allow_list(&self) -> BTreeSet<DepositorAddress> {
        self.depositors_allow_list.borrow().clone()
    }
}

// Setters
impl ThirdPartyDeposits {
    pub fn set_deposit_rule(&self, new: DepositRule) {
        self.deposit_rule.set(new);
    }

    pub fn set_assets_exception_list(&self, new: BTreeSet<AssetException>) {
        *self.assets_exception_list.borrow_mut() = new
    }

    pub fn set_depositors_allow_list(&self, new: BTreeSet<DepositorAddress>) {
        *self.depositors_allow_list.borrow_mut() = new
    }
}

#[cfg(test)]
mod tests {}
