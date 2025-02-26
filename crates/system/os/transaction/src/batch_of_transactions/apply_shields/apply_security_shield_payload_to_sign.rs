use crate::prelude::*;

pub type AddressOfPayerOfShieldApplication = AddressOfAccessControllerOrAccount;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityShieldApplicationWithTransactionIntents {
    pub content: SecurityShieldApplicationWithTransactionIntentsContent,
    pub(crate) intent_set_id: IntentSetID,
}
impl SecurityShieldApplicationWithTransactionIntents {
    pub fn new(
        content: SecurityShieldApplicationWithTransactionIntentsContent,
    ) -> Self {
        Self {
            content,
            intent_set_id: IntentSetID::new(),
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq, EnumAsInner)]
pub enum SecurityShieldApplicationWithTransactionIntentsContent {
    /// Application for an unsecurified entity.
    ForUnsecurifiedEntity(
        SecurityShieldApplicationForUnsecurifiedEntityWithTransactionIntent,
    ),

    /// Application for a securified entity.
    ForSecurifiedEntity(
        SecurityShieldApplicationForSecurifiedEntityWithTransactionIntents,
    ),
}
impl SecurityShieldApplicationWithTransactionIntentsContent {
    pub fn paying_account(&self) -> ApplicationInputPayingAccount {
        match self {
            Self::ForUnsecurifiedEntity(application) => {
                application.paying_account()
            }
            Self::ForSecurifiedEntity(application) => {
                application.paying_account()
            }
        }
    }
}

impl SecurityShieldApplication {
    pub fn unsecurified(
        application: SecurityShieldApplicationForUnsecurifiedEntity,
    ) -> Self {
        Self::ForUnsecurifiedEntity(application)
    }

    pub fn securified(
        application: SecurityShieldApplicationForSecurifiedEntity,
    ) -> Self {
        Self::ForSecurifiedEntity(application)
    }
}

/// An application of a security shield to an unsecurified account
///
/// Essentially holds a manifest for exercising the primary role,
/// to create an AccessController with factors specified in the shield.
#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
pub struct AbstractSecurityShieldApplicationForUnsecurifiedEntity<E: IsEntity> {
    #[allow(dead_code)]
    #[doc(hidden)]
    #[debug(skip)]
    hidden: HiddenConstructor,

    pub entity_applying_shield: AbstractUnsecurifiedEntity<E>,
    pub paying_account: ApplicationInputPayingAccount,
    fee_tip_percentage: Option<u16>,

    /// Manifest for exercising the primary role. This manifest will
    /// create an AccessController with factors specified in the shield.
    ///
    /// # CREATION
    /// Created by `TransactionManifest::apply_security_shield_for_unsecurified_entity`
    ///
    /// # MODIFICATIONS
    /// But we have made two modifications to the manifest output by it:
    ///
    /// ## 1st modification - lock fee
    /// Locking fee against `paying_account
    ///
    /// ## 2nd modification - top up AC XRD vault
    /// `modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_unsecurified_entity_paid_by_account` has been called with `paying_account`
    pub modified_manifest: TransactionManifest,
}

impl<E: IsEntity> AbstractSecurityShieldApplicationForUnsecurifiedEntity<E> {
    pub fn entity_applying_shield(&self) -> AbstractUnsecurifiedEntity<E> {
        self.entity_applying_shield.clone()
    }
    pub fn fee_tip_percentage(&self) -> Option<u16> {
        self.fee_tip_percentage
    }
    pub fn with_modified_manifest(
        entity_applying_shield: AbstractUnsecurifiedEntity<E>,
        paying_account: ApplicationInputPayingAccount,
        modified_manifest: TransactionManifest,
        fee_tip_percentage: impl Into<Option<u16>>,
    ) -> Self {
        Self {
            hidden: HiddenConstructor,
            entity_applying_shield,
            paying_account,
            modified_manifest,
            fee_tip_percentage: fee_tip_percentage.into(),
        }
    }
}

pub type SecurityShieldApplicationForUnsecurifiedAccount =
    AbstractSecurityShieldApplicationForUnsecurifiedEntity<Account>;

pub type SecurityShieldApplicationForUnsecurifiedPersona =
    AbstractSecurityShieldApplicationForUnsecurifiedEntity<Persona>;

pub(crate) trait HasFeeTipPercentage {
    fn fee_tip_percentage(&self) -> Option<u16>;
}

/// The specified Persona to apply the shield for and the account that will
/// pay for the topping up up the AccessControllers XRD vault.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityShieldApplicationForSecurifiedEntityWithPayingAccount<
    E: IsEntity,
> {
    /// The entity to apply the shield for.
    pub entity: AbstractSecurifiedEntity<E>,

    fee_tip_percentage: Option<u16>,

    /// The account topping up the XRD vault of `persona`s AccessControllers
    /// XRD vault.
    pub account_topping_up_xrd_vault_of_access_controller:
        ApplicationInputPayingAccount,
}
impl<E: IsEntity>
    SecurityShieldApplicationForSecurifiedEntityWithPayingAccount<E>
{
    pub fn new(
        entity: AbstractSecurifiedEntity<E>,
        account_topping_up_xrd_vault_of_access_controller: ApplicationInputPayingAccount,
        fee_tip_percentage: impl Into<Option<u16>>,
    ) -> Self {
        Self {
            entity,
            account_topping_up_xrd_vault_of_access_controller,
            fee_tip_percentage: fee_tip_percentage.into(),
        }
    }
}

impl<E: IsEntity> HasFeeTipPercentage
    for SecurityShieldApplicationForSecurifiedEntityWithPayingAccount<E>
{
    fn fee_tip_percentage(&self) -> Option<u16> {
        self.fee_tip_percentage
    }
}

pub type SecurityShieldApplicationForSecurifiedPersonaWithPayingAccount =
    SecurityShieldApplicationForSecurifiedEntityWithPayingAccount<Persona>;
pub type SecurityShieldApplicationForSecurifiedAccountWithPayingAccount =
    SecurityShieldApplicationForSecurifiedEntityWithPayingAccount<Account>;
