use crate::prelude::*;

// ========================
// UNSECURIFIED
// ========================

/// Without Intents (with **single** Manifest) | With balance
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationInputForUnsecurifiedEntity {
    Account(ApplicationInputForUnsecurifiedAccount),
    Persona(ApplicationInputForUnsecurifiedPersona),
}
impl ApplicationInputForUnsecurifiedEntity {
    pub fn xrd_balance_of_paying_account(&self) -> Decimal {
        match self {
            Self::Account(a) => a.xrd_balance_of_paying_account(),
            Self::Persona(p) => p.xrd_balance_of_paying_account(),
        }
    }
}

impl From<ApplicationInputForUnsecurifiedAccount>
    for ApplicationInputForUnsecurifiedEntity
{
    fn from(value: ApplicationInputForUnsecurifiedAccount) -> Self {
        Self::Account(value)
    }
}
impl From<ApplicationInputForUnsecurifiedPersona>
    for ApplicationInputForUnsecurifiedEntity
{
    fn from(value: ApplicationInputForUnsecurifiedPersona) -> Self {
        Self::Persona(value)
    }
}

/// Without Intents (with **single** Manifest) | With balance
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationInputForUnsecurifiedAccount {
    pub reviewed_manifest: TransactionManifest,
    pub estimated_xrd_fee: Decimal,
    pub entity_input: UnsecurifiedAccountEntityInput,
    pub maybe_paying_account: Option<ApplicationInputPayingAccount>,
}
impl ApplicationInputForUnsecurifiedAccount {
    /// we do NOT take Xrd of `maybe_paying_account`'s Xrd Vault - if it is securified.
    pub fn xrd_balance_of_paying_account(&self) -> Decimal {
        self.maybe_paying_account
            .as_ref()
            .map(|p| p.xrd_balance_of_account())
            .unwrap_or(self.entity_input.xrd_balance_of_account)
    }
}

/// Without Intents (with **single** Manifest) | With balance
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationInputForUnsecurifiedPersona {
    pub reviewed_manifest: TransactionManifest,
    pub estimated_xrd_fee: Decimal,
    pub entity_input: UnsecurifiedPersona,
    pub paying_account: ApplicationInputPayingAccount,
}

impl ApplicationInputForUnsecurifiedPersona {
    /// we do NOT take Xrd of `paying_account`'s Xrd Vault - if it is securified.
    pub fn xrd_balance_of_paying_account(&self) -> Decimal {
        self.paying_account.xrd_balance_of_account()
    }
}

trait HasEstimatedXrdFee {
    fn estimated_xrd_fee(&self) -> Decimal;
}
trait HasXrdAmountForInitialXrdVaultContributionAndFee {
    fn xrd_needed_for_tx_fee_and_initial_xrd_vault_contributition(
        &self,
    ) -> Decimal192;
}
impl<T: HasEstimatedXrdFee + IsUnsecurifiedMarker>
    HasXrdAmountForInitialXrdVaultContributionAndFee for T
{
    fn xrd_needed_for_tx_fee_and_initial_xrd_vault_contributition(
        &self,
    ) -> Decimal192 {
        xrd_amount_for_initial_xrd_contribution_of_vault_of_access_controller()
            + self.estimated_xrd_fee()
    }
}

trait HasXrdAmountForXrdVaultTopUpAndFee {
    fn xrd_needed_for_tx_fee_and_xrd_vault_top_up(&self) -> Decimal192;
}
impl<T: HasEstimatedXrdFee + IsSecurifiedWithXrdOfVaultMarker>
    HasXrdAmountForXrdVaultTopUpAndFee for T
{
    fn xrd_needed_for_tx_fee_and_xrd_vault_top_up(&self) -> Decimal192 {
        let target = xrd_target_for_access_controller();
        let current = self.xrd_of_vault_of_access_controller();

        let vault_top_up = if current < target {
            target - current
        } else {
            Decimal::zero()
        };

        vault_top_up + self.estimated_xrd_fee()
    }
}

pub trait IsSecurifiedMarker {}
pub trait IsSecurifiedWithXrdOfVaultMarker: IsSecurifiedMarker {
    fn xrd_of_vault_of_access_controller(&self) -> Decimal;
}

pub trait IsUnsecurifiedMarker {}
pub trait IsAccountMarker {}
pub trait IsPersonaMarker {}
impl IsSecurifiedMarker for ApplicationInputForSecurifiedEntity {}
impl IsSecurifiedMarker for ApplicationInputForSecurifiedAccount {}
impl IsSecurifiedMarker for ApplicationInputForSecurifiedPersona {}
impl IsUnsecurifiedMarker for ApplicationInputForUnsecurifiedEntity {}
impl IsUnsecurifiedMarker for ApplicationInputForUnsecurifiedAccount {}
impl IsUnsecurifiedMarker for ApplicationInputForUnsecurifiedPersona {}
impl IsAccountMarker for ApplicationInputForSecurifiedAccount {}
impl IsAccountMarker for ApplicationInputForUnsecurifiedAccount {}
impl IsPersonaMarker for ApplicationInputForSecurifiedPersona {}
impl IsPersonaMarker for ApplicationInputForUnsecurifiedPersona {}

impl HasEstimatedXrdFee for ApplicationInputForUnsecurifiedAccount {
    fn estimated_xrd_fee(&self) -> Decimal {
        self.estimated_xrd_fee
    }
}
impl HasEstimatedXrdFee for ApplicationInputForUnsecurifiedPersona {
    fn estimated_xrd_fee(&self) -> Decimal {
        self.estimated_xrd_fee
    }
}
impl HasEstimatedXrdFee for ApplicationInputForSecurifiedAccount {
    fn estimated_xrd_fee(&self) -> Decimal {
        self.estimated_xrd_fee
    }
}
impl HasEstimatedXrdFee for ApplicationInputForSecurifiedPersona {
    fn estimated_xrd_fee(&self) -> Decimal {
        self.estimated_xrd_fee
    }
}

// ========================
// ENTITY INPUT
// ========================
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsecurifiedAccountEntityInput {
    /// The entity applying the shield
    pub unsecurified_entity: UnsecurifiedAccount,

    /// XRD balance of the entity applying the shield
    pub xrd_balance_of_account: Decimal,
}
