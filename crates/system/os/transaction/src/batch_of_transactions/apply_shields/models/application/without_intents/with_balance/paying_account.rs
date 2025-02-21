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
pub struct AbstractApplicationInputPayingAccountEntity<AccountKind> {
    /// The account  that will pay for the
    /// transaction and topping up the XRD vault of the AccessController.
    ///
    /// If it is securified - will use the `addresses_of_access_controller`
    /// of the SecuredEntityControl of this account to create proof
    /// for withdrawal and/or lock fee.
    pub account: AccountKind,

    /// XRD balance of `account` (not AccessController XRD vault)
    pub xrd_balance_of_account: Decimal,
}
impl<AccountKind> AbstractApplicationInputPayingAccountEntity<AccountKind> {
    pub fn new(account: AccountKind, xrd_balance_of_account: Decimal) -> Self {
        Self {
            account,
            xrd_balance_of_account,
        }
    }
}

pub type ApplicationInputPayingAccountSecurified =
    AbstractApplicationInputPayingAccountEntity<SecurifiedAccount>;
pub type ApplicationInputPayingAccountUnsecurified =
    AbstractApplicationInputPayingAccountEntity<UnsecurifiedAccount>;

impl ApplicationInputPayingAccountSecurified {
    /// Will be used to create proof for withdrawal and/or lock fee.
    pub fn access_controller_address(&self) -> AccessControllerAddress {
        self.account.access_controller_address()
    }
}
