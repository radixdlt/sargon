use crate::prelude::*;

// ========================
// SECURIFIED
// ========================

/// Without Intents (with **single** Manifest) | With balance
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationInputForSecurifiedEntity {
    Account(ApplicationInputForSecurifiedAccount),
    Persona(ApplicationInputForSecurifiedPersona),
}

impl From<ApplicationInputForSecurifiedAccount>
    for ApplicationInputForSecurifiedEntity
{
    fn from(value: ApplicationInputForSecurifiedAccount) -> Self {
        Self::Account(value)
    }
}
impl From<ApplicationInputForSecurifiedPersona>
    for ApplicationInputForSecurifiedEntity
{
    fn from(value: ApplicationInputForSecurifiedPersona) -> Self {
        Self::Persona(value)
    }
}

/// Without Intents (with **single** Manifest) | With balance
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationInputForSecurifiedAccount {
    pub reviewed_manifest: TransactionManifest,
    pub estimated_xrd_fee: Decimal,
    pub entity_input: SecurifiedAccountEntityInput,
    pub maybe_paying_account: Option<ApplicationInputPayingAccount>,
}

/// Without Intents (with **single** Manifest) | With balance
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationInputForSecurifiedPersona {
    pub reviewed_manifest: TransactionManifest,
    pub estimated_xrd_fee: Decimal,
    pub entity_input: SecurifiedPersonaEntityInput,
    pub maybe_paying_account: Option<ApplicationInputPayingAccount>,
}


impl IsSecurifiedWithXrdOfVaultMarker for ApplicationInputForSecurifiedPersona {
    fn xrd_of_vault_of_access_controller(&self) -> Decimal {
        self.entity_input.xrd_balance_of_access_controller
    }
}


impl IsSecurifiedWithXrdOfVaultMarker for ApplicationInputForSecurifiedAccount {
    fn xrd_of_vault_of_access_controller(&self) -> Decimal {
        self.entity_input.xrd_balance_of_access_controller
    }
}


// ========================
// ENTITY INPUT
// ========================
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurifiedAccountEntityInput {
    /// The Account applying the shield
    pub securified_account: SecurifiedAccount,

    /// XRD balance of the AccessControllers of `securified_account`'s XRD Vault
    pub xrd_balance_of_access_controller: Decimal,

    /// XRD balance of the Account applying the shield
    pub xrd_balance_of_account: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurifiedPersonaEntityInput {
    /// The Persona applying the shield
    pub securified_persona: SecurifiedPersona,

    /// XRD balance of the AccessControllers of `securified_persona`'s XRD Vault
    pub xrd_balance_of_access_controller: Decimal,
}
