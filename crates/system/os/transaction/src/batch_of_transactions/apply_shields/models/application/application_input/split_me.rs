use crate::prelude::*;

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
