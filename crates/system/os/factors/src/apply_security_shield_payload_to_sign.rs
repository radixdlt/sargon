use enum_as_inner::EnumAsInner;
use factor_instances_provider::address_union;
use serde_json::value::Index;

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PayerOfTransaction {
    /// `None` is invalid if `entity_applying_shield` is a Persona.
    /// Some(Account) if `entity_applying_shield` is an Account means "use this other account instead"
    /// None if `entity_applying_shield` is an Account means "use the account applying the shield"
    pub payer: Option<AccountAddress>,
    pub manifest: TransactionManifest,
}

// #[derive(Clone, Debug, PartialEq, Eq, Hash)]
// struct PayerOfTransactionAccount {
//     payer: Account,
//     payer_is_same_as_entity_applying_shield: bool, // MUST be `false` if payer is a Persona.
//     roles: RolesExercisableInTransactionManifestCombination,
//     manifest: TransactionManifest,
// }

#[async_trait::async_trait]
pub trait BatchApplySecurityShieldSigning {
    /// Host has previously called the function
    ///     `make_interaction_for_applying_security_shield`
    /// and specified the `security_shield_id` and `addresses` of the entities
    /// for which they want to apply the security shield. Which returns a Vec
    /// of TransactionManifests, one for each entity. If the entity is securified
    /// already the "variant" `RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithRecovery` is used.
    ///
    /// Host presents batch TX review UI, and user needs to select payer for each manifest,
    /// MUST be done for Personas and in case of entity being an Account, the payer might
    /// be the same account as the entity applying the shield. That information is passed
    /// when user slides to sign back to Sargon via the tuples of `PayerOfTransaction`.
    ///
    /// We will map from `Vec<Manifest>` -> `Vec<Vec<Manifest>>` where for each entity
    /// being unsecurified the inner Vec will be unchanged - one single manifest. But
    /// for each securified entity - which has a manifest which was create with `InitiateWithPrimaryCompleteWithRecovery` variant, we will map to 4 manifests, where
    /// the three new manifests are created by specifying:
    /// - `InitiateWithPrimaryCompleteWithConfirmation`
    /// - `InitiateWithRecoveryCompleteWithConfirmation`
    /// - `InitiateWithRecoveryDelayedCompletion`
    ///
    /// Then we will inner map of the `Vec<Vec<Manifest>>` to
    /// perform look up of all `payer` address and get the Account from
    /// Profile - and depending on if that payer account is already securified or not
    /// we will use `modify_add_lock_fee` for Unsecurified accounts and for securified
    /// accounts we will use `modify_manifest_add_lock_fee_against_xrd_vault_of_access_controller`.
    ///
    /// Then we will build TransactionIntent for all of these - with broad enough
    /// an epoch window so that we can submit these with delay in between.
    ///
    /// We will compile them and we will start the process of signing them. Which will be the job of `SigningManager` - many instances of `SignaturesCollector` using one Role at a time.
    ///
    /// Can work with single transaction of course...
    async fn sign_and_enqueue_batch_of_transactions_applying_security_shield(
        &self,
        manifest_and_payer_tuples: IndexSet<PayerOfTransaction>,
    ) -> Result<IndexSet<TransactionIntentHash>>;
}


#[async_trait::async_trait]
impl BatchApplySecurityShieldSigning for SargonOS {
    async fn sign_and_enqueue_batch_of_transactions_applying_security_shield(
        &self,
        manifest_and_payer_tuples: IndexSet<PayerOfTransaction>,
    ) -> Result<IndexSet<TransactionIntentHash>> {
       let manifest_and_payer_tuples = manifest_and_payer_tuples.into_iter().map(|t| {
            let address_of_ac_or_entity_applying_shield = extract_address_of_entity_updating_shield(&t.manifest)?;
            
       }).collect::<Result<IndexSet<SecurityShieldApplication>>>()?;
    }
}

/// A ApplySecurityShieldPayloadToSign for applying a security shield to many accounts or personas
/// (mixed), either securified or unsecurified (mixed). Which per `application` in
/// `applications` will hold tuples of manifests (or a single for unsecurified),
/// being combinations of roles to be exercised.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplySecurityShieldPayloadToSign {
    /// A collection of all applications for the shield, one per entity,
    /// the element type `SecurityShieldApplication` essentially holds four
    /// different kinds of value, application for:
    /// - unsecurified account
    /// - unsecurified persona
    /// - securified account
    /// - securified persona
    pub applications: Vec<SecurityShieldApplication>,
}

/// Securiy shield application is the application of a security shield
/// to some entity. This entity can be either an account or a persona.
/// Essentially holds four
/// different kinds of value, application for:
/// - unsecurified account
/// - unsecurified persona
/// - securified account
/// - securified persona
#[derive(Debug, Clone, PartialEq, Eq, EnumAsInner)]
pub enum SecurityShieldApplication {
    /// Application for an unsecurified entity.
    ForUnsecurifiedEntity(SecurityShieldApplicationForUnsecurifiedEntity),
    /// Application for a securified entity.
    ForSecurifiedEntity(SecurityShieldApplicationForSecurifiedEntity),
}

/// An application of a security shield to an unsecurified entity
/// holds a single manifest for exercising the primary role (since
/// no other roles are available for unsecurified entities).
///
/// Split into Account vs Persona since for Persona a TX fee payer
/// and AccessControl XRD vault top-up account is needed.
#[derive(Debug, Clone, PartialEq, Eq, EnumAsInner)]
pub enum SecurityShieldApplicationForUnsecurifiedEntity {
    /// Application for an unsecurified account.
    Account(SecurityShieldApplicationForUnsecurifiedAccount),
    /// Application for an unsecurified persona - the associated type
    /// specifies the account that will pay the TX fee and top up the
    /// AccessControl XRD vault.
    Persona(SecurityShieldApplicationForUnsecurifiedPersona),
}

/// An application of a security shield to a securified entity
/// holds many tuples of manifests for each combination of role.
///
/// Split into Account vs Persona since for Persona a TX fee payer
/// and AccessControl XRD vault top-up account is needed.
#[derive(Debug, Clone, PartialEq, Eq, EnumAsInner)]
pub enum SecurityShieldApplicationForSecurifiedEntity {
    /// Application for a securified account.
    Account(SecurityShieldApplicationForSecurifiedAccount),
    /// Application for a securified persona - the associated type
    /// specifies the account that will pay the TX fee and top up the
    /// AccessControl XRD vault.
    Persona(SecurityShieldApplicationForSecurifiedPersona),
}

/// An application of a security shield to an unsecurified account
///
/// Essentially holds a manifest for exercising the primary role,
/// to create an AccessController with factors specified in the shield.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityShieldApplicationForUnsecurifiedAccount {
    #[allow(dead_code)]
    #[doc(hidden)]
    #[debug(skip)]
    hidden: HiddenConstructor,

    /// The account we are applying the shield for.
    pub account_applying_shield: UnsecurifiedAccount,

    /// Optionally user can chose to use a different account to pay
    /// TX fee paying + AC vault filling account. If `Some` it is ensured
    /// that `address_of_paying_account != address_of_account_applying_shield`
    pub paying_account: Option<Account>,

    /// Manifest for exercising the primary role. This manifest will
    /// create an AccessController with factors specified in the shield.
    ///
    /// # CREATION
    /// Created by `TransactionManifest::apply_security_shield_for_unsecurified_entity`
    ///
    /// # MOFIFICATIONS
    /// But we have made two modifications to the manifest output by it:
    ///
    /// ## 1st modification - lock fee
    /// If `address_of_paying_account` is `None` then `modify_manifest_lock_fee(address_of_account_applying_shield)` has been used to modify the manifest.
    /// If `address_of_paying_account` is `Some(UNSECURIFIED_ACCOUNT)` then `modify_manifest_lock_fee(UNSECURIFIED_ACCOUNT)` has been used.
    /// If `address_of_paying_account` is `Some(SECURIFIED_ACCOUNT)` then `modify_manifest_add_lock_fee_against_xrd_vault_of_access_controller(SECURIFIED_ACCOUNT)` has been used.
    ///
    /// ## 2nd modification - top up AC XRD vault
    /// `modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_unsecurified_account_paid_by_account` has been called with `address_of_paying_account` or `address_of_account_applying_shield` as payer.
    pub modified_manifest: TransactionManifest,
}
impl SecurityShieldApplicationForUnsecurifiedAccount {
    /// # Panics
    /// Panics if `address_of_paying_account` is `Some` and `address_of_paying_account == address_of_account_applying_shield`
    fn with_modified_manifest(
        account_applying_shield: UnsecurifiedAccount,
        paying_account: impl Into<Option<Account>>,
        modified_manifest: TransactionManifest,
    ) -> Self {
        if let Some(payer) = paying_account.as_ref() {
            assert_ne!(payer.address(), address_of_account_applying_shield.address(), "Specify None as payer if it is the same as address_of_account_applying_shield");
        }

        Self {
            hidden: HiddenConstructor::new(),
            account_applying_shield,
            paying_account,
            modified_manifest,
        }
    }
}

/// An application of a security shield to an unsecurified persona
/// with a specified account that will pay the TX fee and top up
/// the AccessControl XRD vault.
///
/// Essentially holds a manifest for exercising the primary role,
/// to create an AccessController with factors specified in the shield.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityShieldApplicationForUnsecurifiedPersona {
    /// The persona we are applying the shield for.
    pub persona: UnsecurifiedPersona,

    /// Address of TX fee paying + AC vault filling account
    pub paying_account: Account,

    /// Manifest for exercising the primary role. This manifest will
    /// create an AccessController with factors specified in the shield.
    ///
    /// # CREATION
    /// Created by `TransactionManifest::apply_security_shield_for_unsecurified_entity`
    ///
    /// # MOFIFICATIONS
    /// But we have made two modifications to the manifest output by it:
    ///
    /// ## 1st modification - lock fee
    /// If `paying_account` is `Some(SECURIFIED_ACCOUNT)` then `modify_manifest_add_lock_fee_against_xrd_vault_of_access_controller(SECURIFIED_ACCOUNT)` has been used.
    /// If `paying_account` is `Some(UNSECURIFIED_ACCOUNT)` then `modify_manifest_lock_fee(UNSECURIFIED_ACCOUNT)` has been used.
    ///
    /// ## 2nd modification - top up AC XRD vault
    /// `modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_unsecurified_account_paid_by_account` has been called with `paying_account` as payer.
    pub modified_manifest: TransactionManifest,
}

macro_rules! create_application_for_securified_entity {
    (
        $name:ident,
        $(
            #[doc = $expr: expr]
        )*
        $entity_name:ident: $entity_type:ty
    ) => {

        preinterpret::preinterpret!{
            /// This struct hold `4` different combinations of manifests for different
            /// combinations of roles.
            ///
            /// Later when we want to sign these manifests using the `SignaturesCollector`,
            /// which currently (2025-01-16) can only be used with `1` Role at a time (later
            /// we might change this). Meaning we need to do `3` passes to the  SignaturesCollector, to sign the different manifests.
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct $name {
                $(
                    #[doc = $expr]
                )*
                pub $entity_name: $entity_type,

                /// Modified to include lock_fee against XRD vault of Access controller of entity.
                ///
                /// Also modified modified to perform topping up of XRD vault paid by
                pub initating_with_primary_complete_with_recovery: TransactionManifest,
                pub initating_with_primary_complete_with_confirmation: TransactionManifest,

                pub initating_with_recovery_complete_with_confirmation: TransactionManifest,

                /// Use Time
                pub initating_with_recovery_delayed_completion: TransactionManifest,
            }
        }
    }
}

/// The specified Persona to apply the shield for and the account that will
/// pay for the topping up up the AccessControllers XRD vault.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityShieldApplicationForSecurifiedPersonaWithPayingAccount {
    /// The Persona to apply the shield for.
    pub persona: SecurifiedPersona,

    /// The account topping up the XRD vault of `persona`s AccessControllers
    /// XRD vault.
    pub account_topping_up_xrd_vault_of_access_controller: Option<Account>,
}

/// The specified Account to apply the shield for and optionally another
/// account to pay the topping up of the AccessControllers XRD vault
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityShieldApplicationForSecurifiedAccountWithOptionalPayingAccount
{
    /// The account to apply the shield for.
    pub account: SecurifiedAccount,

    /// An optional account topping up the XRD vault of `accounts`s AccessControllers
    /// XRD vault - if `Some(other)` then `other != account`.
    pub account_topping_up_xrd_vault_of_access_controller: Option<Account>,
}

create_application_for_securified_entity! {
    SecurityShieldApplicationForSecurifiedAccount,
    /// The account we are applying the shield for and an optional other payer
    account_with_optional_paying_account: SecurityShieldApplicationForSecurifiedAccountWithOptionalPayingAccount
}

create_application_for_securified_entity! {
    SecurityShieldApplicationForSecurifiedPersona,
    /// The persona we are applyying the shield for
    /// and the account that will pay the topping up of the AccessControllers XRD vault.
    persona_with_paying_account: SecurityShieldApplicationForSecurifiedPersonaWithPayingAccount
}
