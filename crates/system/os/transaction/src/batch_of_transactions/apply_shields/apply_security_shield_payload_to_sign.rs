use crate::prelude::*;

pub type AddressOfPayerOfShieldApplication = AddressOfVaultOrAccount;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Eq, EnumAsInner)]
pub enum SecurityShieldApplicationWithTransactionIntents {
    /// Application for an unsecurified entity.
    ForUnsecurifiedEntity(
        SecurityShieldApplicationForUnsecurifiedEntityWithTransactionIntent,
    ),

    /// Application for a securified entity.
    ForSecurifiedEntity(
        SecurityShieldApplicationForSecurifiedEntityWithTransactionIntents,
    ),
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
pub struct AbstractSecurityShieldApplicationForUnsecurifiedEntity<
    E: IsBaseEntity + std::hash::Hash + Eq + Clone,
> {
    #[allow(dead_code)]
    #[doc(hidden)]
    #[debug(skip)]
    hidden: HiddenConstructor,

    pub entity_applying_shield: AbstractUnsecurifiedEntity<E>,
    pub paying_account: ApplicationInputPayingAccount,
    pub modified_manifest: TransactionManifest,
}

impl<E: IsBaseEntity + std::hash::Hash + Eq + Clone>
    AbstractSecurityShieldApplicationForUnsecurifiedEntity<E>
{
    pub fn with_modified_manifest(
        entity_applying_shield: AbstractUnsecurifiedEntity<E>,
        paying_account: ApplicationInputPayingAccount,
        modified_manifest: TransactionManifest,
    ) -> Self {
        Self {
            hidden: HiddenConstructor,
            entity_applying_shield,
            paying_account,
            modified_manifest,
        }
    }
}

pub type SecurityShieldApplicationForUnsecurifiedAccount =
    AbstractSecurityShieldApplicationForUnsecurifiedEntity<Account>;

pub type SecurityShieldApplicationForUnsecurifiedPersona =
    AbstractSecurityShieldApplicationForUnsecurifiedEntity<Persona>;

/// The specified Persona to apply the shield for and the account that will
/// pay for the topping up up the AccessControllers XRD vault.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityShieldApplicationForSecurifiedEntityWithPayingAccount<
    E: IsBaseEntity + std::hash::Hash + Eq + Clone,
> {
    /// The entity to apply the shield for.
    pub entity: AbstractSecurifiedEntity<E>,

    /// The account topping up the XRD vault of `persona`s AccessControllers
    /// XRD vault.
    pub account_topping_up_xrd_vault_of_access_controller:
        ApplicationInputPayingAccount,
}
impl<E: IsBaseEntity + std::hash::Hash + Eq + Clone>
    SecurityShieldApplicationForSecurifiedEntityWithPayingAccount<E>
{
    pub fn new(
        entity: AbstractSecurifiedEntity<E>,
        account_topping_up_xrd_vault_of_access_controller: ApplicationInputPayingAccount,
    ) -> Self {
        Self {
            entity,
            account_topping_up_xrd_vault_of_access_controller,
        }
    }
}

pub type SecurityShieldApplicationForSecurifiedPersonaWithPayingAccount =
    SecurityShieldApplicationForSecurifiedEntityWithPayingAccount<Persona>;
pub type SecurityShieldApplicationForSecurifiedAccountWithPayingAccount =
    SecurityShieldApplicationForSecurifiedEntityWithPayingAccount<Account>;
