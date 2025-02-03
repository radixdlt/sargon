use crate::prelude::*;

pub type AddressOfPayerOfShieldApplication = AddressOfVaultOrAccount;

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
    pub paying_account: Option<ApplicationInputPayingAccount>,

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
    pub fn with_modified_manifest(
        account_applying_shield: UnsecurifiedAccount,
        paying_account: impl Into<Option<ApplicationInputPayingAccount>>,
        modified_manifest: TransactionManifest,
    ) -> Self {
        let paying_account = paying_account.into();
        if let Some(payer) = paying_account.as_ref() {
            assert_ne!(payer.account_address(), account_applying_shield.entity.address(), "Specify None as payer if it is the same as address_of_account_applying_shield");
        }

        Self {
            hidden: HiddenConstructor,
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
    pub paying_account: ApplicationInputPayingAccount,

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

impl SecurityShieldApplicationForUnsecurifiedPersona {
    pub fn with_modified_manifest(
        persona_applying_shield: UnsecurifiedPersona,
        paying_account: ApplicationInputPayingAccount,
        modified_manifest: TransactionManifest,
    ) -> Self {
        Self {
            persona: persona_applying_shield,
            paying_account,
            modified_manifest,
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
    pub account_topping_up_xrd_vault_of_access_controller: Option<ApplicationInputPayingAccount>,
}
impl SecurityShieldApplicationForSecurifiedPersonaWithPayingAccount {
    pub fn new(
        persona: SecurifiedPersona,
        account_topping_up_xrd_vault_of_access_controller: impl Into<
            Option<ApplicationInputPayingAccount>,
        >,
    ) -> Self {
        Self {
            persona,
            account_topping_up_xrd_vault_of_access_controller:
                account_topping_up_xrd_vault_of_access_controller.into(),
        }
    }
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
    pub account_topping_up_xrd_vault_of_access_controller: Option<ApplicationInputPayingAccount>,
}

impl SecurityShieldApplicationForSecurifiedAccountWithOptionalPayingAccount {
    pub fn new(
        account: SecurifiedAccount,
        account_topping_up_xrd_vault_of_access_controller: impl Into<
            Option<ApplicationInputPayingAccount>,
        >,
    ) -> Self {
        Self {
            account,
            account_topping_up_xrd_vault_of_access_controller:
                account_topping_up_xrd_vault_of_access_controller.into(),
        }
    }
}
