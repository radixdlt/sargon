use crate::prelude::*;

pub type AddressOfPayerOfShieldApplication = AddressOfVaultOrAccount;

#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
pub struct AbstractShieldApplicationInputWithOrWithoutBalance<
    Entity: HasEntityAddress + Clone,
    XrdBalance,
> {
    #[allow(dead_code)]
    #[doc(hidden)]
    #[debug(skip)]
    hidden: HiddenConstructor,
    /// `None` is invalid if `entity_applying_shield` is a Persona.
    /// Some(Account) if `entity_applying_shield` is an Account means "use this other account instead"
    /// None if `entity_applying_shield` is an Account means "use the account applying the shield"
    ///
    /// N.B.
    /// MUST be `Some` if `balance_of_payer` is `Some`.
    /// MUST be `None` if `balance_of_payer` is `None`.
    _payer: Option<Account>,
    pub entity_applying_shield: Entity,
    pub manifest: TransactionManifest,

    /// N.B.
    /// MUST be `Some` if `payer` is `Some`.
    /// MUST be `None` if `payer` is `None`.
    _balance_of_payer: Option<XrdBalance>,

    _balance_of_entity_applying_shield: XrdBalance,

    pub estimated_xrd_fee: Decimal,
}

impl<Entity: HasEntityAddress + Clone> AbstractShieldApplicationInput<Entity> {
    pub fn modifying_manifest(
        self,
        modify: impl FnOnce(TransactionManifest) -> Result<TransactionManifest>,
    ) -> Result<Self> {
        let mut _self = self;
        _self.manifest = modify(_self.manifest)?;
        Ok(_self)
    }
}

impl<E: HasEntityAddress + Clone, X> HasEntityAddress
    for AbstractShieldApplicationInputWithOrWithoutBalance<E, X>
{
    fn address_erased(&self) -> AddressOfAccountOrPersona {
        self.entity_applying_shield.address_erased()
    }
}

pub type AbstractShieldApplicationInput<Entity> =
    AbstractShieldApplicationInputWithOrWithoutBalance<Entity, Decimal>;

impl<Entity: HasEntityAddress + Clone> AbstractShieldApplicationInput<Entity> {
    pub fn needed_xrd_for_fee_and_topup(&self) -> Decimal192 {
        xrd_amount_for_initial_xrd_vault_of_access_controller()
            + self.estimated_xrd_fee
    }

    pub fn maybe_other_payer_and_balance(
        &self,
    ) -> Option<XrdBalanceOfEntity<Account>> {
        self._payer.as_ref().map(|payer| XrdBalanceOfEntity {
            entity: payer.clone(),
            balance: self
                ._balance_of_payer
                .expect("Must be Some if payer is Some"),
        })
    }

    pub fn get_entity_applying_shield_and_balance(
        &self,
    ) -> XrdBalanceOfEntity<Entity> {
        XrdBalanceOfEntity {
            entity: self.entity_applying_shield.clone(),
            balance: self._balance_of_entity_applying_shield,
        }
    }
}

impl UnsecurifiedAccountShieldApplicationInput {
    /// Payer might be same as entity applying the shield, or it might be another account.
    pub fn payer_with_balance(&self) -> XrdBalanceOfEntity<Account> {
        self.maybe_other_payer_and_balance().unwrap_or(
            self.get_entity_applying_shield_and_balance().into_account(),
        )
    }
}

impl SecurifiedAccountShieldApplicationInput {
    /// Payer might be same as entity applying the shield, or it might be another account.
    pub fn payer_with_balance(&self) -> XrdBalanceOfEntity<Account> {
        self.maybe_other_payer_and_balance().unwrap_or(
            self.get_entity_applying_shield_and_balance().into_account(),
        )
    }
}

impl UnsecurifiedPersonaShieldApplicationInput {
    /// Payer might be same as entity applying the shield, or it might be another account.
    pub fn payer_with_balance(&self) -> Result<XrdBalanceOfEntity<Account>> {
        self.maybe_other_payer_and_balance()
            .ok_or(CommonError::Unknown)
    }
}

impl SecurifiedPersonaShieldApplicationInput {
    /// Payer might be same as entity applying the shield, or it might be another account.
    pub fn payer_with_balance(&self) -> Result<XrdBalanceOfEntity<Account>> {
        self.maybe_other_payer_and_balance()
            .ok_or(CommonError::Unknown)
    }
}

impl ShieldApplicationInputWithoutXrdBalance {
    pub fn get_payer(&self) -> Option<Account> {
        self._payer.clone()
    }
    pub fn get_entity_applying_shield(&self) -> EntityApplyingShield {
        self.entity_applying_shield.clone()
    }
}

impl From<(ShieldApplicationInput, AnyUnsecurifiedEntity)>
    for AnyUnsecurifiedShieldApplicationInput
{
    fn from(
        (some, entity_applying_shield): (
            ShieldApplicationInput,
            AnyUnsecurifiedEntity,
        ),
    ) -> Self {
        Self::with_entity_applying_shield(some, entity_applying_shield)
    }
}

impl From<(ShieldApplicationInput, AnySecurifiedEntity)>
    for AnySecurifiedShieldApplicationInput
{
    fn from(
        (some, entity_applying_shield): (
            ShieldApplicationInput,
            AnySecurifiedEntity,
        ),
    ) -> Self {
        Self::with_entity_applying_shield(some, entity_applying_shield)
    }
}

impl From<(AnySecurifiedShieldApplicationInput, SecurifiedAccount)>
    for SecurifiedAccountShieldApplicationInput
{
    fn from(
        (some, entity_applying_shield): (
            AnySecurifiedShieldApplicationInput,
            SecurifiedAccount,
        ),
    ) -> Self {
        assert!(some.address_erased().is_account(), "Must be Account");
        Self::with_entity_applying_shield(some, entity_applying_shield)
    }
}

impl From<(AnySecurifiedShieldApplicationInput, SecurifiedPersona)>
    for SecurifiedPersonaShieldApplicationInput
{
    fn from(
        (some, entity_applying_shield): (
            AnySecurifiedShieldApplicationInput,
            SecurifiedPersona,
        ),
    ) -> Self {
        assert!(some.address_erased().is_identity(), "Must be Persona");
        Self::with_entity_applying_shield(some, entity_applying_shield)
    }
}

pub type ShieldApplicationInput =
    AbstractShieldApplicationInput<EntityApplyingShield>;

pub type ShieldApplicationInputWithoutXrdBalance =
    AbstractShieldApplicationInputWithOrWithoutBalance<
        EntityApplyingShield,
        (),
    >;

impl ShieldApplicationInputWithoutXrdBalance {
    pub fn addresses_to_fetch_xrd_balance_for(
        &self,
    ) -> IndexSet<AddressOfPayerOfShieldApplication> {
        let mut addresses = IndexSet::new();

        if let Some(payer) = self._payer.as_ref() {
            match payer.security_state() {
                EntitySecurityState::Securified { value: sec } => {
                    addresses.insert(AddressOfPayerOfShieldApplication::Vault(
                        sec.xrd_vault_address(),
                    ));
                }
                EntitySecurityState::Unsecured { .. } => {
                    addresses.insert(
                        AddressOfPayerOfShieldApplication::Account(
                            payer.address(),
                        ),
                    );
                }
            }
        }

        match self.get_entity_applying_shield() {
            EntityApplyingShield::Securified(e) => {
                addresses.insert(AddressOfPayerOfShieldApplication::Vault(
                    e.securified_entity_control.xrd_vault_address(),
                ));
            }
            EntityApplyingShield::Unsecurified(e) => {
                match e.entity {
                    AccountOrPersona::PersonaEntity(_) => {
                        // nothing to do
                    }
                    AccountOrPersona::AccountEntity(a) => {
                        addresses.insert(
                            AddressOfPayerOfShieldApplication::Account(
                                a.address(),
                            ),
                        );
                    }
                }
            }
        }

        addresses
    }
}

pub trait IsSecurifiedMarker {}
impl IsSecurifiedMarker for SecurifiedAccount {}
impl IsSecurifiedMarker for SecurifiedPersona {}
impl IsSecurifiedMarker for AnySecurifiedEntity {}
pub trait IsUnsecurifiedMarker {}
impl IsUnsecurifiedMarker for AnyUnsecurifiedEntity {}
impl IsUnsecurifiedMarker for UnsecurifiedAccount {}
impl IsUnsecurifiedMarker for UnsecurifiedPersona {}

pub type AnyUnsecurifiedShieldApplicationInput =
    AbstractShieldApplicationInput<AnyUnsecurifiedEntity>;

pub type UnsecurifiedAccountShieldApplicationInput =
    AbstractShieldApplicationInput<UnsecurifiedAccount>;

pub type UnsecurifiedPersonaShieldApplicationInput =
    AbstractShieldApplicationInput<UnsecurifiedPersona>;

pub type AnySecurifiedShieldApplicationInput =
    AbstractShieldApplicationInput<AnySecurifiedEntity>;

pub type SecurifiedAccountShieldApplicationInput =
    AbstractShieldApplicationInput<SecurifiedAccount>;

pub type SecurifiedPersonaShieldApplicationInput =
    AbstractShieldApplicationInput<SecurifiedPersona>;

impl ShieldApplicationInputWithoutXrdBalance {
    pub fn new(
        payer: impl Into<Option<Account>>,
        entity_applying_shield: EntityApplyingShield,
        manifest: TransactionManifest,
        estimated_xrd_fee: Decimal,
    ) -> Self {
        let payer = payer.into();
        let balance_of_payer: Option<()> = payer.as_ref().map(|_| ());
        Self {
            hidden: HiddenConstructor,
            _payer: payer,
            entity_applying_shield,
            manifest,
            _balance_of_payer: balance_of_payer,
            _balance_of_entity_applying_shield: (),
            estimated_xrd_fee,
        }
    }
}

impl<Entity: HasEntityAddress + Clone> AbstractShieldApplicationInput<Entity> {
    pub fn new(
        payer_with_balance: impl Into<Option<XrdBalanceOfEntity<Account>>>,
        entity_applying_shield_and_balance: XrdBalanceOfEntity<Entity>,
        manifest: TransactionManifest,
        estimated_xrd_fee: Decimal,
    ) -> Self {
        let payer_with_balance = payer_with_balance.into();
        Self {
            hidden: HiddenConstructor,
            _payer: payer_with_balance.as_ref().map(|p| p.entity.clone()),
            entity_applying_shield: entity_applying_shield_and_balance.entity,
            manifest,
            _balance_of_payer: payer_with_balance.map(|p| p.balance),
            _balance_of_entity_applying_shield:
                entity_applying_shield_and_balance.balance,
            estimated_xrd_fee,
        }
    }
    fn with_entity_applying_shield<T: HasEntityAddress + Clone>(
        some: AbstractShieldApplicationInput<T>,
        entity_applying_shield: impl Into<Entity>,
    ) -> Self
where {
        let entity_applying_shield = entity_applying_shield.into();

        let uncasted_entity_with_balance =
            some.get_entity_applying_shield_and_balance();

        assert_eq!(
            uncasted_entity_with_balance.address_erased(),
            entity_applying_shield.address_erased()
        );

        let casted_entity_with_balance = XrdBalanceOfEntity {
            entity: entity_applying_shield,
            balance: uncasted_entity_with_balance.balance,
        };

        Self::new(
            some.maybe_other_payer_and_balance(),
            casted_entity_with_balance,
            some.manifest,
            some.estimated_xrd_fee,
        )
    }
}

impl From<(AnyUnsecurifiedShieldApplicationInput, UnsecurifiedAccount)>
    for UnsecurifiedAccountShieldApplicationInput
{
    fn from(
        (some, entity_applying_shield): (
            AnyUnsecurifiedShieldApplicationInput,
            UnsecurifiedAccount,
        ),
    ) -> Self {
        Self::with_entity_applying_shield(some, entity_applying_shield)
    }
}

impl From<(AnyUnsecurifiedShieldApplicationInput, UnsecurifiedPersona)>
    for UnsecurifiedPersonaShieldApplicationInput
{
    fn from(
        (some, entity_applying_shield): (
            AnyUnsecurifiedShieldApplicationInput,
            UnsecurifiedPersona,
        ),
    ) -> Self {
        Self::with_entity_applying_shield(some, entity_applying_shield)
    }
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
impl SecurityShieldApplicationForUnsecurifiedEntity {
    pub fn manifest(&self) -> &TransactionManifest {
        match self {
            SecurityShieldApplicationForUnsecurifiedEntity::Account(a) => {
                &a.modified_manifest
            }
            SecurityShieldApplicationForUnsecurifiedEntity::Persona(p) => {
                &p.modified_manifest
            }
        }
    }
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
impl SecurityShieldApplicationForSecurifiedEntity {
    pub fn initiate_with_recovery_complete_with_confirmation(
        &self,
    ) -> &TransactionManifest {
        match self {
            SecurityShieldApplicationForSecurifiedEntity::Account(a) => {
                &a.initiate_with_recovery_complete_with_confirmation
            }
            SecurityShieldApplicationForSecurifiedEntity::Persona(p) => {
                &p.initiate_with_recovery_complete_with_confirmation
            }
        }
    }
    pub fn initiate_with_recovery_complete_with_primary(
        &self,
    ) -> &TransactionManifest {
        match self {
            SecurityShieldApplicationForSecurifiedEntity::Account(a) => {
                &a.initiate_with_recovery_complete_with_primary
            }
            SecurityShieldApplicationForSecurifiedEntity::Persona(p) => {
                &p.initiate_with_recovery_complete_with_primary
            }
        }
    }
    pub fn initiate_with_recovery_delayed_completion(
        &self,
    ) -> &TransactionManifest {
        match self {
            SecurityShieldApplicationForSecurifiedEntity::Account(a) => {
                &a.initiate_with_recovery_delayed_completion
            }
            SecurityShieldApplicationForSecurifiedEntity::Persona(p) => {
                &p.initiate_with_recovery_delayed_completion
            }
        }
    }
    pub fn initiate_with_primary_complete_with_confirmation(
        &self,
    ) -> &TransactionManifest {
        match self {
            SecurityShieldApplicationForSecurifiedEntity::Account(a) => {
                &a.initiate_with_primary_complete_with_confirmation
            }
            SecurityShieldApplicationForSecurifiedEntity::Persona(p) => {
                &p.initiate_with_primary_complete_with_confirmation
            }
        }
    }
    pub fn initiate_with_primary_delayed_completion(
        &self,
    ) -> &TransactionManifest {
        match self {
            SecurityShieldApplicationForSecurifiedEntity::Account(a) => {
                &a.initiate_with_primary_delayed_completion
            }
            SecurityShieldApplicationForSecurifiedEntity::Persona(p) => {
                &p.initiate_with_primary_delayed_completion
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct SecurityShieldApplicationForUnsecurifiedAccountWithTransactionIntent
{
    pub application: SecurityShieldApplicationForUnsecurifiedAccount,
    pub transaction_intent: TransactionIntent,
}
impl SecurityShieldApplicationForUnsecurifiedAccountWithTransactionIntent {
    pub fn new(
        application: SecurityShieldApplicationForUnsecurifiedAccount,
        transaction_intent: TransactionIntent,
    ) -> Self {
        Self {
            application,
            transaction_intent,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct SecurityShieldApplicationForUnsecurifiedPersonaWithTransactionIntent
{
    pub application: SecurityShieldApplicationForUnsecurifiedPersona,
    pub transaction_intent: TransactionIntent,
}
impl SecurityShieldApplicationForUnsecurifiedPersonaWithTransactionIntent {
    pub fn new(
        application: SecurityShieldApplicationForUnsecurifiedPersona,
        transaction_intent: TransactionIntent,
    ) -> Self {
        Self {
            application,
            transaction_intent,
        }
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
    pub fn with_modified_manifest(
        account_applying_shield: UnsecurifiedAccount,
        paying_account: impl Into<Option<Account>>,
        modified_manifest: TransactionManifest,
    ) -> Self {
        let paying_account = paying_account.into();
        if let Some(payer) = paying_account.as_ref() {
            assert_ne!(payer.address(), account_applying_shield.entity.address(), "Specify None as payer if it is the same as address_of_account_applying_shield");
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

impl SecurityShieldApplicationForUnsecurifiedPersona {
    pub fn with_modified_manifest(
        persona_applying_shield: UnsecurifiedPersona,
        paying_account: Account,
        modified_manifest: TransactionManifest,
    ) -> Self {
        Self {
            persona: persona_applying_shield,
            paying_account,
            modified_manifest,
        }
    }
}

macro_rules! create_application_for_securified_entity_with_payloads {
    (
        $name:ident,
        $payload_ty: ty,
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

                pub initiate_with_recovery_complete_with_primary: $payload_ty,
                pub initiate_with_recovery_complete_with_confirmation: $payload_ty,
                pub initiate_with_recovery_delayed_completion: $payload_ty,
                pub initiate_with_primary_complete_with_confirmation: $payload_ty,
                pub initiate_with_primary_delayed_completion: $payload_ty,

            }

            impl $name {
                pub fn new(
                    $entity_name: $entity_type,
                    initiate_with_recovery_complete_with_primary: $payload_ty,
                    initiate_with_recovery_complete_with_confirmation: $payload_ty,
                    initiate_with_recovery_delayed_completion: $payload_ty,
                    initiate_with_primary_complete_with_confirmation: $payload_ty,
                    initiate_with_primary_delayed_completion: $payload_ty,
                ) -> Self {
                    Self {
                        $entity_name,
                        initiate_with_recovery_complete_with_primary,
                        initiate_with_recovery_complete_with_confirmation,
                        initiate_with_recovery_delayed_completion,
                        initiate_with_primary_complete_with_confirmation,
                        initiate_with_primary_delayed_completion,
                    }
                }
            }
        }
    }
}

macro_rules! create_application_for_securified_entity_with_intents {
    (
        $name:ident,
        $(
            #[doc = $expr: expr]
        )*
        $entity_name:ident: $entity_type:ty
    ) => {
        create_application_for_securified_entity_with_payloads!(
            $name,
            TransactionIntent,
            $(
                #[doc = $expr]
            )*
            $entity_name: $entity_type
        );
    }
}

macro_rules! create_application_for_securified_entity_with_manifests {
    (
        $name:ident,
        $(
            #[doc = $expr: expr]
        )*
        $entity_name:ident: $entity_type:ty
    ) => {
        create_application_for_securified_entity_with_payloads!(
            $name,
            TransactionManifest,
            $(
                #[doc = $expr]
            )*
            $entity_name: $entity_type
        );
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
impl SecurityShieldApplicationForSecurifiedPersonaWithPayingAccount {
    pub fn new(
        persona: SecurifiedPersona,
        account_topping_up_xrd_vault_of_access_controller: impl Into<
            Option<Account>,
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
    pub account_topping_up_xrd_vault_of_access_controller: Option<Account>,
}
impl SecurityShieldApplicationForSecurifiedAccountWithOptionalPayingAccount {
    pub fn new(
        account: SecurifiedAccount,
        account_topping_up_xrd_vault_of_access_controller: impl Into<
            Option<Account>,
        >,
    ) -> Self {
        Self {
            account,
            account_topping_up_xrd_vault_of_access_controller:
                account_topping_up_xrd_vault_of_access_controller.into(),
        }
    }
}

create_application_for_securified_entity_with_manifests! {
    SecurityShieldApplicationForSecurifiedAccount,
    /// The account we are applying the shield for and an optional other payer
    account_with_optional_paying_account: SecurityShieldApplicationForSecurifiedAccountWithOptionalPayingAccount
}

create_application_for_securified_entity_with_manifests! {
    SecurityShieldApplicationForSecurifiedPersona,
    /// The persona we are applyying the shield for
    /// and the account that will pay the topping up of the AccessControllers XRD vault.
    persona_with_paying_account: SecurityShieldApplicationForSecurifiedPersonaWithPayingAccount
}

create_application_for_securified_entity_with_intents! {
    SecurityShieldApplicationTransactionIntentsForSecurifiedPersona,
    /// The persona we are applyying the shield for
    /// and the account that will pay the topping up of the AccessControllers XRD vault.
    persona_with_paying_account: SecurityShieldApplicationForSecurifiedPersonaWithPayingAccount
}

create_application_for_securified_entity_with_intents! {
    SecurityShieldApplicationTransactionIntentsForSecurifiedAccount,
    /// The account we are applying the shield for and an optional other payer
    account_with_optional_paying_account: SecurityShieldApplicationForSecurifiedAccountWithOptionalPayingAccount
}

#[derive(Debug, PartialEq, Eq, EnumAsInner)]
pub enum SecurityShieldApplicationForUnsecurifiedEntityWithTransactionIntent {
    Account(
        SecurityShieldApplicationForUnsecurifiedAccountWithTransactionIntent,
    ),
    Persona(
        SecurityShieldApplicationForUnsecurifiedPersonaWithTransactionIntent,
    ),
}

impl SecurityShieldApplicationForUnsecurifiedEntityWithTransactionIntent {
    pub fn with_intent(
        without: SecurityShieldApplicationForUnsecurifiedEntity,
        intent: TransactionIntent,
    ) -> Self {
        match without {
            SecurityShieldApplicationForUnsecurifiedEntity::Account(a) => {
                Self::Account(SecurityShieldApplicationForUnsecurifiedAccountWithTransactionIntent::new(a, intent))
            }
            SecurityShieldApplicationForUnsecurifiedEntity::Persona(p) => {
                Self::Persona(SecurityShieldApplicationForUnsecurifiedPersonaWithTransactionIntent::new(p, intent))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, EnumAsInner)]
pub enum SecurityShieldApplicationForSecurifiedEntityWithTransactionIntents {
    Account(SecurityShieldApplicationTransactionIntentsForSecurifiedAccount),
    Persona(SecurityShieldApplicationTransactionIntentsForSecurifiedPersona),
}

impl SecurityShieldApplicationForSecurifiedEntityWithTransactionIntents {
    pub fn with_intents(
        without: SecurityShieldApplicationForSecurifiedEntity,
        initiate_with_recovery_complete_with_primary: TransactionIntent,
        initiate_with_recovery_complete_with_confirmation: TransactionIntent,
        initiate_with_recovery_delayed_completion: TransactionIntent,
        initiate_with_primary_complete_with_confirmation: TransactionIntent,
        initiate_with_primary_delayed_completion: TransactionIntent,
    ) -> Self {
        match without {
            SecurityShieldApplicationForSecurifiedEntity::Account(a) => {
                Self::Account(SecurityShieldApplicationTransactionIntentsForSecurifiedAccount::new(a.account_with_optional_paying_account, initiate_with_recovery_complete_with_primary, initiate_with_recovery_complete_with_confirmation, initiate_with_recovery_delayed_completion, initiate_with_primary_complete_with_confirmation, initiate_with_primary_delayed_completion))
            }
            SecurityShieldApplicationForSecurifiedEntity::Persona(p) => {
                Self::Persona(SecurityShieldApplicationTransactionIntentsForSecurifiedPersona::new(p.persona_with_paying_account, initiate_with_recovery_complete_with_primary, initiate_with_recovery_complete_with_confirmation, initiate_with_recovery_delayed_completion, initiate_with_primary_complete_with_confirmation, initiate_with_primary_delayed_completion))
            }
        }
    }
}
