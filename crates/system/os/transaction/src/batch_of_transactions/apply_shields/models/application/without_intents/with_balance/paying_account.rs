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
pub struct ApplicationInputPayingAccountSecurified {
    pub account: SecurifiedAccount,
    pub access_controller_address: AccessControllerAddress,
    pub xrd_vault_address: VaultAddress,

    /// XRD balance of `xrd_vault_address`
    pub xrd_balance_of_access_controller: Decimal,

    /// XRD balance of `account`
    pub xrd_balance_of_account: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationInputPayingAccountUnsecurified {
    pub account: UnsecurifiedAccount,

    /// XRD balance of `account`
    pub xrd_balance_of_account: Decimal,
}
