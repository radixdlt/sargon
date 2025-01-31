use enum_as_inner::EnumAsInner;
use sargon_os_factors::{
    extract_address_of_entity_updating_shield, AbstractSecurifiedEntity,
    AbstractUnsecurifiedEntity, AnySecurifiedEntity, AnyUnsecurifiedEntity,
    EntityApplyingShieldAddress, GetEntityAddressByAccessControllerAddress,
    IsSecurifiedEntity, ProfileEntitiesOfKindOnNetworkInKeySpace,
    SecurifiedAccount, SecurifiedPersona, UnsecurifiedAccount,
    UnsecurifiedPersona,
};

use crate::prelude::*;

impl HasEntityAddress for EntityApplyingShield {
    fn address_erased(&self) -> AddressOfAccountOrPersona {
        match self {
            EntityApplyingShield::Securified(e) => e.address_erased(),
            EntityApplyingShield::Unsecurified(e) => e.address_erased(),
        }
    }
}

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
    fn modifying_manifest(
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

pub struct XrdBalanceOfEntity<Entity: HasEntityAddress + Clone> {
    pub entity: Entity,
    pub balance: Decimal,
}

impl<Entity: HasEntityAddress + Clone + Into<Account>>
    XrdBalanceOfEntity<Entity>
{
    pub fn into_account(self) -> XrdBalanceOfEntity<Account> {
        XrdBalanceOfEntity {
            entity: self.entity.into(),
            balance: self.balance,
        }
    }
}
impl<Entity: HasEntityAddress + Clone> XrdBalanceOfEntity<Entity> {
    pub fn new(entity: impl Into<Entity>, balance: Decimal) -> Self {
        Self {
            entity: entity.into(),
            balance,
        }
    }
}

impl<Entity: HasEntityAddress + Clone> HasEntityAddress
    for XrdBalanceOfEntity<Entity>
{
    fn address_erased(&self) -> AddressOfAccountOrPersona {
        self.entity.address_erased()
    }
}

type AbstractShieldApplicationInput<Entity> =
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
    fn addresses_to_fetch_xrd_balance_for(
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
    fn new(
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
    fn new(
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ManifestWithPayerByAddress {
    /// `None` is invalid if `entity_applying_shield` is a Persona.
    /// Some(Account) if `entity_applying_shield` is an Account means "use this other account instead"
    /// None if `entity_applying_shield` is an Account means "use the account applying the shield"
    pub payer: Option<AccountAddress>,
    pub manifest: TransactionManifest,
    pub estimated_xrd_fee: Decimal,
}

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
    /// when user slides to sign back to Sargon via the tuples of `ManifestWithPayer`.
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
        network_id: NetworkID,
        manifest_and_payer_tuples: IndexSet<ManifestWithPayerByAddress>,
    ) -> Result<IndexSet<TransactionIntentHash>>;

    async fn sign_transaction_intents(
        &self,
        payload_to_sign: ApplySecurityShieldPayloadToSign,
    ) -> Result<ApplySecurityShieldSignedPayload>;

    async fn persist_notary_private_keys_to_be_able_to_cancel_transactions(
        &self,
        transaction_id_to_notary_private_key: &IndexMap<
            TransactionIntentHash,
            Ed25519PrivateKey,
        >,
    ) -> Result<()>;

    fn get_securified_entity_by_access_controller(
        &self,
        address: AccessControllerAddress,
    ) -> Result<AnySecurifiedEntity>;

    fn get_unsecurified_account_by_address(
        &self,
        address: AccountAddress,
    ) -> Result<UnsecurifiedAccount>;

    fn get_unsecurified_persona_by_address(
        &self,
        address: IdentityAddress,
    ) -> Result<UnsecurifiedPersona>;

    async fn get_xrd_balances(
        &self,
        network_id: NetworkID,
        manifests_with_entities_without_xrd_balances: Vec<
            ShieldApplicationInputWithoutXrdBalance,
        >,
    ) -> Result<Vec<ShieldApplicationInput>>;

    fn get_entity_applying_shield(
        &self,
        address: EntityApplyingShieldAddress,
    ) -> Result<EntityApplyingShield> {
        match address {
            EntityApplyingShieldAddress::AccessController(ac) => self
                .get_securified_entity_by_access_controller(ac)
                .map(EntityApplyingShield::securified),
            EntityApplyingShieldAddress::Account(a) => self
                .get_unsecurified_account_by_address(a)
                .map(EntityApplyingShield::unsecurified_account),
            EntityApplyingShieldAddress::Identity(i) => self
                .get_unsecurified_persona_by_address(i)
                .map(EntityApplyingShield::unsecurified_persona),
        }
    }

    fn _modify_manifest_add_fee<Entity>(
        input: AbstractShieldApplicationInput<Entity>,
        // None if unsecurified
        manifest_variant: Option<
            RolesExercisableInTransactionManifestCombination,
        >,
    ) -> Result<AbstractShieldApplicationInput<Entity>>
    where
        Entity: HasEntityAddress + Clone,
    {
        //    if let Some(other_payer) = input.
        todo!()
    }

    fn modify_manifest_add_fee_securified<T>(
        input: AbstractShieldApplicationInput<AbstractSecurifiedEntity<T>>,
        manifest_variant: RolesExercisableInTransactionManifestCombination,
    ) -> Result<AbstractShieldApplicationInput<AbstractSecurifiedEntity<T>>>
    where
        T: IsBaseEntity + std::hash::Hash + Eq + Clone,
    {
        Self::_modify_manifest_add_fee(input, Some(manifest_variant))
    }

    fn modify_manifest_add_fee_for_unsecurified_entity_applying_shield<T>(
        input: AbstractShieldApplicationInput<AbstractUnsecurifiedEntity<T>>,
    ) -> Result<AbstractShieldApplicationInput<AbstractUnsecurifiedEntity<T>>>
    where
        T: IsBaseEntity + std::hash::Hash + Eq + Clone,
    {
        Self::_modify_manifest_add_fee(input, None)
    }

    fn modify_manifest_add_xrd_vault_contribution_for_unsecurified_account_applying_shield(
        input: UnsecurifiedAccountShieldApplicationInput,
    ) -> Result<UnsecurifiedAccountShieldApplicationInput> {
        let unsecurified_account_applying_shield_with_balance =
            input.get_entity_applying_shield_and_balance();

        let payer_with_balance = input.payer_with_balance();

        if payer_with_balance.balance < input.needed_xrd_for_fee_and_topup() {
            return Err(CommonError::Unknown); // CommonError::InsufficientXrdBalance
        }

        let payer = payer_with_balance.entity;
        let unsecurified_account_applying_shield =
            unsecurified_account_applying_shield_with_balance.entity;

        input.modifying_manifest(|m| {
                let m = TransactionManifest::modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_unsecurified_account_paid_by_account(payer, unsecurified_account_applying_shield.into(), m, None);

                Ok(m)
            })
    }

    fn modify_manifest_add_xrd_vault_contribution_for_unsecurified_persona_applying_shield(
        input: UnsecurifiedPersonaShieldApplicationInput,
    ) -> Result<(UnsecurifiedPersonaShieldApplicationInput, Account)> {
        let payer_with_balance = input.payer_with_balance()?;

        if payer_with_balance.balance < input.needed_xrd_for_fee_and_topup() {
            return Err(CommonError::Unknown); // CommonError::InsufficientXrdBalance
        }

        let unsecurified_persona_applying_shield = input
            .get_entity_applying_shield_and_balance()
            .entity
            .clone()
            .into();

        input.modifying_manifest(|m| {
                let m = TransactionManifest::modify_manifest_add_withdraw_of_xrd_for_access_controller_xrd_vault_top_up_of_unsecurified_account_paid_by_account(payer_with_balance.entity.clone(), unsecurified_persona_applying_shield, m, None);

                Ok(m)
            }).map(|m| (m, payer_with_balance.entity))
    }

    fn shield_application_for_unsecurified_account(
        &self,
        input: UnsecurifiedAccountShieldApplicationInput,
    ) -> Result<SecurityShieldApplicationForUnsecurifiedAccount> {
        let input = Self::modify_manifest_add_fee_for_unsecurified_entity_applying_shield(input)?;
        let input = Self::modify_manifest_add_xrd_vault_contribution_for_unsecurified_account_applying_shield(input)?;

        Ok(SecurityShieldApplicationForUnsecurifiedAccount::with_modified_manifest(
            input.entity_applying_shield.clone(),
            input.maybe_other_payer_and_balance().map(|p| p.entity),
            input.manifest,
        ))
    }

    fn modify_manifest_add_xrd_vault_contribution_for_securified_account_applying_shield(
        input: SecurifiedAccountShieldApplicationInput,
        manifest_variant: RolesExercisableInTransactionManifestCombination,
    ) -> Result<SecurifiedAccountShieldApplicationInput> {
        todo!()
    }

    fn modify_manifest_add_xrd_vault_contribution_for_securified_persona_applying_shield(
        input: SecurifiedPersonaShieldApplicationInput,
        manifest_variant: RolesExercisableInTransactionManifestCombination,
    ) -> Result<SecurifiedPersonaShieldApplicationInput> {
        todo!()
    }

    fn shield_application_for_unsecurified_persona(
        &self,
        input: UnsecurifiedPersonaShieldApplicationInput,
    ) -> Result<SecurityShieldApplicationForUnsecurifiedPersona> {
        let input = Self::modify_manifest_add_fee_for_unsecurified_entity_applying_shield(input)?;
        let (input, paying_account) = Self::modify_manifest_add_xrd_vault_contribution_for_unsecurified_persona_applying_shield(input)?;

        Ok(SecurityShieldApplicationForUnsecurifiedPersona::with_modified_manifest(
            input.entity_applying_shield.clone(),
            paying_account,
            input.manifest,
        ))
    }

    fn shield_application_for_unsecurified_entity(
        &self,
        input: AnyUnsecurifiedShieldApplicationInput,
    ) -> Result<SecurityShieldApplicationForUnsecurifiedEntity> {
        let entity = &input.entity_applying_shield;
        match &entity.entity {
            AccountOrPersona::AccountEntity(a) => self
                .shield_application_for_unsecurified_account(
                    UnsecurifiedAccountShieldApplicationInput::from((
                        input.clone(),
                        UnsecurifiedAccount::with_unsecured_entity_control(
                            a.clone(),
                            entity.unsecured_entity_control.clone(),
                        ),
                    )),
                )
                .map(SecurityShieldApplicationForUnsecurifiedEntity::Account),
            AccountOrPersona::PersonaEntity(p) => self
                .shield_application_for_unsecurified_persona(
                    UnsecurifiedPersonaShieldApplicationInput::from((
                        input.clone(),
                        UnsecurifiedPersona::with_unsecured_entity_control(
                            p.clone(),
                            entity.unsecured_entity_control.clone(),
                        ),
                    )),
                )
                .map(SecurityShieldApplicationForUnsecurifiedEntity::Persona),
        }
    }

    fn shield_application_for_securified_account(
        &self,
        input: SecurifiedAccountShieldApplicationInput,
    ) -> Result<SecurityShieldApplicationForSecurifiedAccount> {
        let manifest_for_variant =
            |variant: RolesExercisableInTransactionManifestCombination| {
                let manifest_with = input.clone();

                let manifest_with = Self::modify_manifest_add_fee_securified(
                    manifest_with,
                    variant,
                )?;

                let manifest_with = Self::modify_manifest_add_xrd_vault_contribution_for_securified_account_applying_shield(manifest_with, variant)?;

                Ok(manifest_with.manifest)
            };

        let initiate_with_recovery_complete_with_primary = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithPrimary)?;

        let initiate_with_recovery_complete_with_confirmation = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithConfirmation)?;

        let initiate_with_recovery_delayed_completion = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion)?;

        let initiate_with_primary_complete_with_confirmation = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithConfirmation)?;

        let initiate_with_primary_delayed_completion = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryDelayedCompletion)?;

        let account_with_optional_paying_account = SecurityShieldApplicationForSecurifiedAccountWithOptionalPayingAccount::new(input.entity_applying_shield.clone(), input.maybe_other_payer_and_balance().map(|p| p.entity));

        Ok(SecurityShieldApplicationForSecurifiedAccount::new(
            account_with_optional_paying_account,
            initiate_with_recovery_complete_with_primary,
            initiate_with_recovery_complete_with_confirmation,
            initiate_with_recovery_delayed_completion,
            initiate_with_primary_complete_with_confirmation,
            initiate_with_primary_delayed_completion,
        ))
    }

    fn shield_application_for_securified_persona(
        &self,
        input: SecurifiedPersonaShieldApplicationInput,
    ) -> Result<SecurityShieldApplicationForSecurifiedPersona> {
        let manifest_for_variant =
            |variant: RolesExercisableInTransactionManifestCombination| {
                let manifest_with = input.clone();

                let manifest_with = Self::modify_manifest_add_fee_securified(
                    manifest_with,
                    variant,
                )?;

                let manifest_with = Self::modify_manifest_add_xrd_vault_contribution_for_securified_persona_applying_shield(manifest_with, variant)?;

                Ok(manifest_with.manifest)
            };

        let initiate_with_recovery_complete_with_primary = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithPrimary)?;

        let initiate_with_recovery_complete_with_confirmation = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryCompleteWithConfirmation)?;

        let initiate_with_recovery_delayed_completion = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithRecoveryDelayedCompletion)?;

        let initiate_with_primary_complete_with_confirmation = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryCompleteWithConfirmation)?;

        let initiate_with_primary_delayed_completion = manifest_for_variant(RolesExercisableInTransactionManifestCombination::InitiateWithPrimaryDelayedCompletion)?;

        let payer =
            SecurityShieldApplicationForSecurifiedPersonaWithPayingAccount::new(
                input.entity_applying_shield.clone(),
                input.maybe_other_payer_and_balance().map(|p| p.entity), // TODO Should we fail here if the an account doing top up is not specified?
            );

        Ok(SecurityShieldApplicationForSecurifiedPersona::new(
            payer,
            initiate_with_recovery_complete_with_primary,
            initiate_with_recovery_complete_with_confirmation,
            initiate_with_recovery_delayed_completion,
            initiate_with_primary_complete_with_confirmation,
            initiate_with_primary_delayed_completion,
        ))
    }

    fn shield_application_for_securified_entity(
        &self,
        input: AnySecurifiedShieldApplicationInput,
    ) -> Result<SecurityShieldApplicationForSecurifiedEntity> {
        let entity = &input.entity_applying_shield;
        match &entity.entity {
            AccountOrPersona::AccountEntity(a) => self
                .shield_application_for_securified_account(
                    SecurifiedAccountShieldApplicationInput::from((
                        input.clone(),
                        SecurifiedAccount::with_securified_entity_control(
                            a.clone(),
                            entity.securified_entity_control(),
                        ),
                    )),
                )
                .map(SecurityShieldApplicationForSecurifiedEntity::Account),
            AccountOrPersona::PersonaEntity(p) => self
                .shield_application_for_securified_persona(
                    SecurifiedPersonaShieldApplicationInput::from((
                        input.clone(),
                        SecurifiedPersona::with_securified_entity_control(
                            p.clone(),
                            entity.securified_entity_control(),
                        ),
                    )),
                )
                .map(SecurityShieldApplicationForSecurifiedEntity::Persona),
        }
    }

    fn assert_that_payer_is_not_in_batch_of_entities_applying_shield(
        &self,
        manifests_with_entities_without_xrd_balances: impl AsRef<
            [ShieldApplicationInputWithoutXrdBalance],
        >,
    ) -> Result<()>;

    async fn batch_fetch_xrd_balances_of_accounts_or_access_controllers(
        &self,
        network_id: NetworkID,
        addresses: IndexSet<AddressOfPayerOfShieldApplication>,
    ) -> Result<IndexMap<AddressOfPayerOfShieldApplication, Decimal>>;

    async fn build_transaction_intents(
        &self,
        network_id: NetworkID,
        manifests_with_entities_with_xrd_balance: Vec<
            SecurityShieldApplication,
        >,
    ) -> Result<ApplySecurityShieldPayloadToSign>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityApplyingShield {
    Unsecurified(AnyUnsecurifiedEntity),
    Securified(AnySecurifiedEntity),
}

impl EntityApplyingShield {
    fn securified(entity: AnySecurifiedEntity) -> Self {
        Self::Securified(entity)
    }

    fn unsecurified_account(account: UnsecurifiedAccount) -> Self {
        Self::Unsecurified(AnyUnsecurifiedEntity::from(account))
    }

    fn unsecurified_persona(persona: UnsecurifiedPersona) -> Self {
        Self::Unsecurified(AnyUnsecurifiedEntity::from(persona))
    }
}

#[async_trait::async_trait]
impl BatchApplySecurityShieldSigning for SargonOS {
    fn get_securified_entity_by_access_controller(
        &self,
        address: AccessControllerAddress,
    ) -> Result<AnySecurifiedEntity> {
        self.profile().and_then(|p| {
            p.get_securified_entity_by_access_controller_address(address)
        })
    }

    fn get_unsecurified_account_by_address(
        &self,
        address: AccountAddress,
    ) -> Result<UnsecurifiedAccount> {
        self.profile().and_then(|p| {
            p.unsecurified_accounts_on_network(address.network_id())
                .iter()
                .find(|a| a.entity.address == address)
                .ok_or(CommonError::UnknownAccount)
        })
    }

    fn get_unsecurified_persona_by_address(
        &self,
        address: IdentityAddress,
    ) -> Result<UnsecurifiedPersona> {
        self.profile().and_then(|p| {
            p.unsecurified_personas_on_network(address.network_id())
                .iter()
                .find(|a| a.entity.address == address)
                .ok_or(CommonError::UnknownPersona)
        })
    }

    fn assert_that_payer_is_not_in_batch_of_entities_applying_shield(
        &self,
        manifests_with_entities_without_xrd_balances: impl AsRef<
            [ShieldApplicationInputWithoutXrdBalance],
        >,
    ) -> Result<()> {
        let payer_addresses = manifests_with_entities_without_xrd_balances
            .as_ref()
            .iter()
            .filter_map(|i| i.get_payer())
            .map(|a| a.address())
            .map(AddressOfAccountOrPersona::from)
            .collect::<IndexSet<_>>();

        if manifests_with_entities_without_xrd_balances
            .as_ref()
            .iter()
            .any(|i| payer_addresses.contains(&i.address_erased()))
        {
            return Err(CommonError::Unknown); // CommonError::PayerCannotBeInBatchOfEntitiesApplyingShield
        }

        Ok(())
    }

    async fn batch_fetch_xrd_balances_of_accounts_or_access_controllers(
        &self,
        network_id: NetworkID,
        addresses: IndexSet<AddressOfPayerOfShieldApplication>,
    ) -> Result<IndexMap<AddressOfPayerOfShieldApplication, Decimal>> {
        assert!(addresses.iter().all(|a| a.network_id() == network_id));
        let gateway_client =
            GatewayClient::new(self.http_client.driver.clone(), network_id);

        let balances = gateway_client
            .xrd_balances_of_vault_or_account(network_id, addresses)
            .await?;

        let balances = balances
            .into_iter()
            .map(|(k, v)| (k, v.unwrap_or(Decimal192::zero())))
            .collect::<IndexMap<_, _>>();

        Ok(balances)
    }

    async fn get_xrd_balances(
        &self,
        network_id: NetworkID,
        manifests_with_entities_without_xrd_balances: Vec<
            ShieldApplicationInputWithoutXrdBalance,
        >,
    ) -> Result<Vec<ShieldApplicationInput>> {
        let addresses_to_query = manifests_with_entities_without_xrd_balances
            .iter()
            .flat_map(|i| i.addresses_to_fetch_xrd_balance_for())
            .collect::<IndexSet<AddressOfPayerOfShieldApplication>>();

        let balances = self
            .batch_fetch_xrd_balances_of_accounts_or_access_controllers(
                network_id,
                addresses_to_query,
            )
            .await?;

        manifests_with_entities_without_xrd_balances
                .into_iter()
                .map(|i| {
                    let entity_applying_shield_and_balance_res: Result<XrdBalanceOfEntity<EntityApplyingShield>> = match i.get_entity_applying_shield() {
                        EntityApplyingShield::Securified(e) => {
                            let vault_address = e.xrd_vault_address();
                            let balance = balances.get(&AddressOfVaultOrAccount::Vault(vault_address)).ok_or(CommonError::Unknown).cloned()?; // TODO better error
                            Ok(XrdBalanceOfEntity {
                                entity: EntityApplyingShield::securified(e),
                                balance
                            })
                        },
                        EntityApplyingShield::Unsecurified(e) => {
                            match &e.entity {
                                AccountOrPersona::AccountEntity(a) => {
                                    let balance = balances.get(&AddressOfVaultOrAccount::Account(a.address())).ok_or(CommonError::Unknown).cloned()?; // TODO better error
                            Ok(XrdBalanceOfEntity {
                                entity: EntityApplyingShield::unsecurified_account(UnsecurifiedAccount::with_unsecured_entity_control(a.clone(), e.unsecured_entity_control.clone())),
                                balance
                            })
                                }
                                AccountOrPersona::PersonaEntity(p) => {
                                    // Unsecurified Personas cannot have any XRD... 
                                    // thus we use Decimal192::zero(), which is a safe default
                                    // we can update the types involved in this function
                                    // to make this exeuction path impossible, alas,
                                    // they are already too complex, so seems no worth it.
                                    Ok(XrdBalanceOfEntity {
                                        entity: EntityApplyingShield::unsecurified_persona(UnsecurifiedPersona::with_unsecured_entity_control(p.clone(), e.unsecured_entity_control.clone())),
                                        balance: Decimal192::zero()
                                    })
                                }
                            }
                        },
                    };
                    let entity_applying_shield_and_balance = entity_applying_shield_and_balance_res?;
                    match i.get_payer() {
                        Some(payer) => {
                            let balance = balances.get(&AddressOfVaultOrAccount::Account(payer.address())).ok_or(CommonError::Unknown).cloned()?; // TODO better error
                            Ok(ShieldApplicationInput::new(XrdBalanceOfEntity::<Account> {
                                entity: payer,
                                balance
                            }, entity_applying_shield_and_balance, i.manifest, i.estimated_xrd_fee))
                        }
                        None => {
                            Ok(ShieldApplicationInput::new(None, entity_applying_shield_and_balance, i.manifest, i.estimated_xrd_fee))
                        }
                    }
                })
                .collect::<Result<Vec<ShieldApplicationInput>>>()
    }

    async fn build_transaction_intents(
        &self,
        network_id: NetworkID,
        manifests_with_entities_with_xrd_balance: Vec<
            SecurityShieldApplication,
        >,
    ) -> Result<ApplySecurityShieldPayloadToSign> {
        let gateway_client =
            GatewayClient::new(self.http_client.driver.clone(), network_id);

        let start_epoch_inclusive = gateway_client.current_epoch().await?;
        let end_epoch_exclusive = Epoch::one_week_from(start_epoch_inclusive);

        let mut transaction_id_to_notary_private_key: IndexMap<
            TransactionIntentHash,
            Ed25519PrivateKey,
        > = IndexMap::new();

        let mut build_intent = |manifest: &TransactionManifest,
                                nonce: Nonce,
                                notary_private_key_bytes: Exactly32Bytes|
         -> Result<TransactionIntent> {
            let notary_private_key = Ed25519PrivateKey::from_exactly32_bytes(
                notary_private_key_bytes,
            );
            let notary_public_key = notary_private_key.public_key();
            let header = TransactionHeader::new(
                network_id,
                start_epoch_inclusive,
                end_epoch_exclusive,
                nonce,
                notary_public_key,
                NotaryIsSignatory(true),
                0,
            );

            let intent = TransactionIntent::new(
                header,
                manifest.clone(),
                Message::None,
            )?;

            // So we can return the notary keys to callee so we can notarize later.
            //
            // For securified entities the same notary private key will be present under many TransactionIntentHash
            // map keys (identifier).
            transaction_id_to_notary_private_key
                .insert(intent.transaction_intent_hash(), notary_private_key);

            Ok(intent)
        };

        let with_intents = manifests_with_entities_with_xrd_balance.into_iter().map(|m| {
            // We tactically use the same nonce for all variants of the TransactionIntents
            // for securified entities - ensuring that we cannot accidentally submit
            // two variants of the same application.
            let nonce =  Nonce::random();
            // We can use the same notary private key for all variants since they
            // are in fact the same application
            let notary_private_key_bytes = Exactly32Bytes::generate();
            match m {
                SecurityShieldApplication::ForUnsecurifiedEntity(unsec) => {
                    let intent = build_intent(unsec.manifest(), nonce, notary_private_key_bytes)?;
                    let with_intents = SecurityShieldApplicationForUnsecurifiedEntityWithTransactionIntent::with_intent(unsec, intent);

                    Ok(SecurityShieldApplicationWithTransactionIntents::ForUnsecurifiedEntity(with_intents))
                }
                SecurityShieldApplication::ForSecurifiedEntity(sec) => {
                    let with_intents: SecurityShieldApplicationForSecurifiedEntityWithTransactionIntents = {

                        let initiate_with_recovery_complete_with_primary = build_intent(sec.initiate_with_recovery_complete_with_primary(), nonce, notary_private_key_bytes)?;

                        let initiate_with_recovery_complete_with_confirmation = build_intent(sec.initiate_with_recovery_complete_with_confirmation(), nonce, notary_private_key_bytes)?;

                        let initiate_with_recovery_delayed_completion = build_intent(sec.initiate_with_recovery_delayed_completion(), nonce, notary_private_key_bytes)?;

                        let initiate_with_primary_complete_with_confirmation = build_intent(sec.initiate_with_primary_complete_with_confirmation(), nonce, notary_private_key_bytes)?;

                        let initiate_with_primary_delayed_completion = build_intent(sec.initiate_with_primary_delayed_completion(), nonce, notary_private_key_bytes)?;

                        Ok(SecurityShieldApplicationForSecurifiedEntityWithTransactionIntents::with_intents(sec, initiate_with_recovery_complete_with_primary, initiate_with_recovery_complete_with_confirmation, initiate_with_recovery_delayed_completion, initiate_with_primary_complete_with_confirmation, initiate_with_primary_delayed_completion))
                    }?;
                    Ok(SecurityShieldApplicationWithTransactionIntents::ForSecurifiedEntity(with_intents))
                }
            }
        }).collect::<Result<Vec<SecurityShieldApplicationWithTransactionIntents>>>()?;

        let payload_to_sign = ApplySecurityShieldPayloadToSign {
            applications_with_intents: with_intents,
            notary_keys: transaction_id_to_notary_private_key,
        };

        Ok(payload_to_sign)
    }

    async fn persist_notary_private_keys_to_be_able_to_cancel_transactions(
        &self,
        _transaction_id_to_notary_private_key: &IndexMap<
            TransactionIntentHash,
            Ed25519PrivateKey,
        >,
    ) -> Result<()> {
        // We do not support this yet, but might in the future.
        info!("Skipped persisting notary private keys to be able to cancel transactions");
        Ok(())
    }

    async fn sign_transaction_intents(
        &self,
        payload_to_sign: ApplySecurityShieldPayloadToSign,
    ) -> Result<ApplySecurityShieldSignedPayload> {
        todo!()
    }

    async fn sign_and_enqueue_batch_of_transactions_applying_security_shield(
        &self,
        network_id: NetworkID,
        manifest_and_payer_tuples: IndexSet<ManifestWithPayerByAddress>,
    ) -> Result<IndexSet<TransactionIntentHash>> {
        let manifests_with_entities_without_xrd_balances = manifest_and_payer_tuples
            .into_iter()
            .map(|manifest_with_payer_by_address| {
                let manifest = manifest_with_payer_by_address.manifest;
                let estimated_xrd_fee = manifest_with_payer_by_address.estimated_xrd_fee;
                let address_of_ac_or_entity_applying_shield =
                    extract_address_of_entity_updating_shield(&manifest)?;

                let entity_applying_shield = self.get_entity_applying_shield(
                    address_of_ac_or_entity_applying_shield,
                )?;

                if let Some(payer_address) =
                    manifest_with_payer_by_address.payer
                {
                    let payer = self.account_by_address(payer_address)?;
                    Ok(ShieldApplicationInputWithoutXrdBalance::new(
                        payer,
                        entity_applying_shield,
                        manifest,
                        estimated_xrd_fee
                    ))
                } else {
                    Ok(ShieldApplicationInputWithoutXrdBalance::new(
                        None,
                        entity_applying_shield,
                        manifest,
                        estimated_xrd_fee
                    ))
                }
            })
            .collect::<Result<Vec<ShieldApplicationInputWithoutXrdBalance>>>()?;

        // Assert that payer if specified is not part of the batch of entities applying shield
        self.assert_that_payer_is_not_in_batch_of_entities_applying_shield(
            &manifests_with_entities_without_xrd_balances,
        )?;

        let manifests_with_entities_with_xrd_balance = self
            .get_xrd_balances(
                network_id,
                manifests_with_entities_without_xrd_balances,
            )
            .await?;

        let applications_without_intents =
            manifests_with_entities_with_xrd_balance
                .into_iter()
                .map(|manifest_with_payer| {
                    match &manifest_with_payer.entity_applying_shield {
                        EntityApplyingShield::Unsecurified(entity) => self
                            .shield_application_for_unsecurified_entity(
                                AnyUnsecurifiedShieldApplicationInput::from((
                                    manifest_with_payer.clone(),
                                    entity.clone(),
                                )),
                            )
                            .map(SecurityShieldApplication::unsecurified),
                        EntityApplyingShield::Securified(entity) => self
                            .shield_application_for_securified_entity(
                                AnySecurifiedShieldApplicationInput::from((
                                    manifest_with_payer.clone(),
                                    entity.clone(),
                                )),
                            )
                            .map(SecurityShieldApplication::securified),
                    }
                })
                .collect::<Result<Vec<SecurityShieldApplication>>>()?;

        let payload_to_sign = self
            .build_transaction_intents(network_id, applications_without_intents)
            .await?;

        self.persist_notary_private_keys_to_be_able_to_cancel_transactions(
            &payload_to_sign.notary_keys,
        )
        .await?;

        let signed_transactions =
            self.sign_transaction_intents(payload_to_sign).await?;

        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ApplySecurityShieldPayloadToSign {
    pub applications_with_intents:
        Vec<SecurityShieldApplicationWithTransactionIntents>,
    pub notary_keys: IndexMap<TransactionIntentHash, Ed25519PrivateKey>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ApplySecurityShieldSignedPayload {
    /// Only one transaction per application - for Securified Entities we will have had 5 manifests
    /// and we select "the best" (quick confirm if possible) depending on the outcome of the
    /// signing process
    pub notarized_transactions: Vec<NotarizedTransaction>,
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
    fn unsecurified(
        application: SecurityShieldApplicationForUnsecurifiedEntity,
    ) -> Self {
        Self::ForUnsecurifiedEntity(application)
    }

    fn securified(
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
    fn initiate_with_recovery_complete_with_confirmation(
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
    fn initiate_with_recovery_complete_with_primary(
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
    fn initiate_with_recovery_delayed_completion(
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
    fn initiate_with_primary_complete_with_confirmation(
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
    fn initiate_with_primary_delayed_completion(&self) -> &TransactionManifest {
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
    fn with_modified_manifest(
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
    fn with_modified_manifest(
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
