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

    pub fn fee_tip_percentage(&self) -> Option<u16> {
        match self {
            Self::Account(a) => a.fee_tip_percentage(),
            Self::Persona(p) => p.fee_tip_percentage(),
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
pub struct AbstractApplicationInputForUnsecurifiedEntity<EntityInput> {
    pub reviewed_manifest: TransactionManifest,
    pub estimated_xrd_fee: Decimal,
    pub entity_input: EntityInput,
    pub paying_account: ApplicationInputPayingAccount,
    fee_tip_percentage: Option<u16>,
}

pub type ApplicationInputForUnsecurifiedAccount =
    AbstractApplicationInputForUnsecurifiedEntity<
        UnsecurifiedAccountEntityInput,
    >;

pub type ApplicationInputForUnsecurifiedPersona =
    AbstractApplicationInputForUnsecurifiedEntity<UnsecurifiedPersona>;

impl<EntityInput> AbstractApplicationInputForUnsecurifiedEntity<EntityInput> {
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

    /// we do NOT take Xrd of `paying_account`'s Xrd Vault - if it is securified.
    pub fn xrd_balance_of_paying_account(&self) -> Decimal {
        self.paying_account.xrd_balance_of_account()
    }

    pub fn payer(&self) -> Account {
        self.paying_account.account()
    }

    pub fn fee_tip_percentage(&self) -> Option<u16> {
        self.fee_tip_percentage
    }
}

trait HasEstimatedXrdFee {
    fn estimated_xrd_fee(&self) -> Decimal;
}
pub trait HasXrdAmountForInitialXrdVaultContributionAndFee {
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

pub trait HasXrdAmountForXrdVaultTopUpAndFee {
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

pub trait ReviewedManifestOwner {
    fn get_reviewed_manifest(&self) -> TransactionManifest;
    fn set_manifest(&mut self, manifest: TransactionManifest);
}

impl<EntityInput> ReviewedManifestOwner
    for AbstractApplicationInputForUnsecurifiedEntity<EntityInput>
{
    fn get_reviewed_manifest(&self) -> TransactionManifest {
        self.reviewed_manifest.clone()
    }
    fn set_manifest(&mut self, manifest: TransactionManifest) {
        self.reviewed_manifest = manifest;
    }
}

impl<EntityInput> ReviewedManifestOwner
    for AbstractApplicationInputForSecurifiedEntity<EntityInput>
{
    fn get_reviewed_manifest(&self) -> TransactionManifest {
        self.reviewed_manifest.clone()
    }
    fn set_manifest(&mut self, manifest: TransactionManifest) {
        self.reviewed_manifest = manifest;
    }
}

pub trait ManifestModying: Sized {
    fn try_modifying_manifest(
        self,
        modified: impl FnOnce(TransactionManifest) -> Result<TransactionManifest>,
    ) -> Result<Self>;

    fn modifying_manifest(
        self,
        modified: impl FnOnce(TransactionManifest) -> TransactionManifest,
    ) -> Result<Self> {
        self.try_modifying_manifest(|m| Ok(modified(m)))
    }
}
impl<T: ReviewedManifestOwner> ManifestModying for T {
    fn try_modifying_manifest(
        self,
        modified: impl FnOnce(TransactionManifest) -> Result<TransactionManifest>,
    ) -> Result<Self> {
        let reviewed_manifest = self.get_reviewed_manifest();
        let modified_manifest = modified(reviewed_manifest)?;
        let mut self_ = self;
        self_.set_manifest(modified_manifest);
        Ok(self_)
    }
}

pub trait IsUnsecurifiedMarker {}
pub trait IsAccountMarker {}
pub trait IsPersonaMarker {}
impl IsSecurifiedMarker for ApplicationInputForSecurifiedEntity {}
impl IsUnsecurifiedMarker for ApplicationInputForUnsecurifiedEntity {}
impl IsAccountMarker for ApplicationInputForSecurifiedAccount {}
impl IsAccountMarker for ApplicationInputForUnsecurifiedAccount {}
impl IsPersonaMarker for ApplicationInputForSecurifiedPersona {}
impl IsPersonaMarker for ApplicationInputForUnsecurifiedPersona {}

impl<EntityInput> HasEstimatedXrdFee
    for AbstractApplicationInputForUnsecurifiedEntity<EntityInput>
{
    fn estimated_xrd_fee(&self) -> Decimal {
        self.estimated_xrd_fee
    }
}

impl<EntityInput> HasEstimatedXrdFee
    for AbstractApplicationInputForSecurifiedEntity<EntityInput>
{
    fn estimated_xrd_fee(&self) -> Decimal {
        self.estimated_xrd_fee
    }
}

impl<EntityInput> IsUnsecurifiedMarker
    for AbstractApplicationInputForUnsecurifiedEntity<EntityInput>
{
}

impl<EntityInput> IsSecurifiedMarker
    for AbstractApplicationInputForSecurifiedEntity<EntityInput>
{
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
