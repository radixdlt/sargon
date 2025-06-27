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

impl ApplicationInputForSecurifiedEntity {
    pub fn fee_tip_percentage(&self) -> Option<u16> {
        match self {
            Self::Account(a) => a.fee_tip_percentage,
            Self::Persona(p) => p.fee_tip_percentage,
        }
    }
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
pub struct AbstractApplicationInputForSecurifiedEntity<EntityInput> {
    pub reviewed_manifest: TransactionManifest,
    pub estimated_xrd_fee: Decimal,
    pub entity_input: EntityInput,
    pub paying_account: ApplicationInputPayingAccount,
    fee_tip_percentage: Option<u16>,
}

impl<EntityInput> AbstractApplicationInputForSecurifiedEntity<EntityInput> {
    pub fn new(
        reviewed_manifest: TransactionManifest,
        estimated_xrd_fee: Decimal,
        entity_input: EntityInput,
        paying_account: ApplicationInputPayingAccount,
        fee_tip_percentage: impl Into<Option<u16>>,
    ) -> Self {
        Self {
            reviewed_manifest,
            estimated_xrd_fee,
            entity_input,
            paying_account,
            fee_tip_percentage: fee_tip_percentage.into(),
        }
    }

    pub fn fee_tip_percentage(&self) -> Option<u16> {
        self.fee_tip_percentage
    }
}

pub type ApplicationInputForSecurifiedAccount =
    AbstractApplicationInputForSecurifiedEntity<SecurifiedAccountEntityInput>;

pub type ApplicationInputForSecurifiedPersona =
    AbstractApplicationInputForSecurifiedEntity<SecurifiedPersonaEntityInput>;

impl IsSecurifiedWithXrdOfVaultMarker for ApplicationInputForSecurifiedPersona {
    fn xrd_of_vault_of_access_controller(&self) -> Decimal {
        self.entity_input.xrd_balance_of_access_controller
    }
}

impl ApplicationInputForSecurifiedPersona {
    pub fn xrd_balance_and_account_topping_up(
        &self,
    ) -> ApplicationInputPayingAccount {
        self.paying_account.clone()
    }
}

impl IsSecurifiedWithXrdOfVaultMarker for ApplicationInputForSecurifiedAccount {
    fn xrd_of_vault_of_access_controller(&self) -> Decimal {
        self.entity_input.xrd_balance_of_access_controller
    }
}

impl ApplicationInputForSecurifiedAccount {
    pub fn xrd_balance_and_account_topping_up(
        &self,
    ) -> XrdBalanceOfEntity<Account> {
        XrdBalanceOfEntity::new(
            self.paying_account.account(),
            self.paying_account.xrd_balance_of_account(),
        )
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
