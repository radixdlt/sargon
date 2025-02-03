use crate::prelude::*;


// ========================
// PAYING ACCOUNT
// ========================
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationInputPayingAccount {
    Securified(ApplicationInputPayingAccountSecurified),
    Unsecurified(ApplicationInputPayingAccountUnsecurified),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationInputPayingAccountSecurified {
    pub account_address: AccountAddress,
    pub access_controller_address: AccessControllerAddress,
    pub xrd_vault_address: VaultAddress,

    /// XRD balance of `xrd_vault_address`
    pub xrd_balance_of_access_controller: Decimal,

    /// XRD balance of `account_address`
    pub xrd_balance_of_of_account: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationInputPayingAccountUnsecurified {
    pub account_address: AccountAddress,

    /// XRD balance of `account_address`
    pub xrd_balance_of_of_account: Decimal,
}
