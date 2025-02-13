use crate::prelude::*;

// ========================
// PAYING ACCOUNT
// ========================
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationInputPayingAccount {
    Securified(ApplicationInputPayingAccountSecurified),
    Unsecurified(ApplicationInputPayingAccountUnsecurified),
}
impl ApplicationInputPayingAccount {
    pub fn account_address(&self) -> AccountAddress {
        self.account().address
    }

    pub fn fee_tip(&self) -> Option<Decimal> {
        match self {
            Self::Securified(input) => input.fee_tip(),
            Self::Unsecurified(input) => input.fee_tip(),
        }
    }

    pub fn account(&self) -> Account {
        match self {
            Self::Securified(input) => input.account.entity.clone(),
            Self::Unsecurified(input) => input.account.entity.clone(),
        }
    }

    pub fn xrd_balance_of_account(&self) -> Decimal {
        match self {
            Self::Securified(input) => input.xrd_balance_of_account,
            Self::Unsecurified(input) => input.xrd_balance_of_account,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationInputPayingAccountSecurified {
    /// The account - known to be securified - that will pay for the
    /// transaction and topping up the XRD vault of the AccessController.
    ///
    /// We will use the `addresses_of_access_controller` of the SecuredEntityControl
    /// of this account to create proof for withdrawal and/or lock fee.
    pub account: SecurifiedAccount,

    /// XRD balance of `account` (not AccessController XRD vault)
    pub xrd_balance_of_account: Decimal,

    fee_tip: Option<Decimal>,
}
impl ApplicationInputPayingAccountSecurified {
    pub fn new(
        account: SecurifiedAccount,
        xrd_balance_of_account: Decimal,
        fee_tip: impl Into<Option<Decimal>>,
    ) -> Self {
        Self {
            account,
            xrd_balance_of_account,
            fee_tip: fee_tip.into(),
        }
    }


    pub fn fee_tip(&self) -> Option<Decimal> {
       self.fee_tip.clone()
    }

    /// Will be used to create proof for withdrawal and/or lock fee.
    pub fn access_controller_address(&self) -> AccessControllerAddress {
        self.account.access_controller_address()
    }
    pub fn xrd_vault_address(&self) -> VaultAddress {
        self.account.xrd_vault_address()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationInputPayingAccountUnsecurified {
    /// The account - known to be unsecurified - that will pay for the
    /// transaction and topping up the XRD vault of the AccessController.
    pub account: UnsecurifiedAccount,

    /// XRD balance of `account`
    pub xrd_balance_of_account: Decimal,

}
impl ApplicationInputPayingAccountUnsecurified {
    pub fn new(
        account: UnsecurifiedAccount,
        xrd_balance_of_account: Decimal,
    ) -> Self {
        Self {
            account,
            xrd_balance_of_account,
        }
    }
}
