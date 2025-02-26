use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShieldApplicationInputWithoutXrdBalance {
    Unsecurified(ApplicationInputForUnsecurifiedEntityWithoutXrdBalance),
    Securified(ApplicationInputForSecurifiedEntityWithoutXrdBalance),
}
impl ShieldApplicationInputWithoutXrdBalance {
    pub fn addresses_to_fetch_xrd_balance_for(
        &self,
    ) -> IndexSet<AddressOfPayerOfShieldApplication> {
        match self {
            Self::Unsecurified(u) => u.addresses_to_fetch_xrd_balance_for(),
            Self::Securified(s) => s.addresses_to_fetch_xrd_balance_for(),
        }
    }
    pub fn address_of_entity_applying_shield(
        &self,
    ) -> AddressOfAccountOrPersona {
        match self {
            Self::Unsecurified(u) => u.address_of_entity_applying_shield(),
            Self::Securified(s) => s.address_of_entity_applying_shield(),
        }
    }

    pub fn payer_is_entity(&self) -> bool {
        self.address_of_entity_applying_shield()
            == self.get_payer().address.into()
    }
    pub fn get_payer(&self) -> Account {
        match self {
            Self::Unsecurified(u) => u.get_payer(),
            Self::Securified(s) => s.get_payer(),
        }
    }
    pub fn fee_tip_percentage(&self) -> Option<u16> {
        match self {
            Self::Unsecurified(u) => u.fee_tip_percentage(),
            Self::Securified(s) => s.fee_tip_percentage(),
        }
    }
}

// ========================
// UNSECURIFIED
// ========================
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationInputForUnsecurifiedEntityWithoutXrdBalance {
    Account(ApplicationInputForUnsecurifiedAccountWithoutXrdBalance),
    Persona(ApplicationInputForUnsecurifiedPersonaWithoutXrdBalance),
}
impl ApplicationInputForUnsecurifiedEntityWithoutXrdBalance {
    pub fn fee_tip_percentage(&self) -> Option<u16> {
        match self {
            Self::Account(a) => a.fee_tip_percentage(),
            Self::Persona(p) => p.fee_tip_percentage(),
        }
    }

    pub fn addresses_to_fetch_xrd_balance_for(
        &self,
    ) -> IndexSet<AddressOfPayerOfShieldApplication> {
        match self {
            Self::Account(a) => a.addresses_to_fetch_xrd_balance_for(),
            Self::Persona(p) => p.addresses_to_fetch_xrd_balance_for(),
        }
    }

    pub fn address_of_entity_applying_shield(
        &self,
    ) -> AddressOfAccountOrPersona {
        match self {
            Self::Account(a) => a.entity_input.address(),
            Self::Persona(p) => p.entity_input.address(),
        }
    }

    pub fn get_payer(&self) -> Account {
        match self {
            Self::Account(a) => a.paying_account.clone(),
            Self::Persona(p) => p.paying_account.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbstractApplicationInputForUnsecurifiedEntityWithoutXrdBalance<
    E: IsEntity,
> {
    pub reviewed_manifest: TransactionManifest,
    pub estimated_xrd_fee: Decimal,
    pub entity_input: AbstractUnsecurifiedEntity<E>,
    pub paying_account: Account,
    fee_tip_percentage: Option<u16>,
}
pub type ApplicationInputForUnsecurifiedAccountWithoutXrdBalance =
    AbstractApplicationInputForUnsecurifiedEntityWithoutXrdBalance<Account>;

impl<E: IsEntity>
    AbstractApplicationInputForUnsecurifiedEntityWithoutXrdBalance<E>
{
    pub fn new(
        reviewed_manifest: TransactionManifest,
        estimated_xrd_fee: Decimal,
        entity_input: AbstractUnsecurifiedEntity<E>,
        paying_account: Account,
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

    pub fn addresses_to_fetch_xrd_balance_for(
        &self,
    ) -> IndexSet<AddressOfPayerOfShieldApplication> {
        match self.entity_input.address() {
            AddressOfAccountOrPersona::Account(a) => IndexSet::from_iter([
                a.into(),
                self.paying_account.address.into(),
            ]),
            AddressOfAccountOrPersona::Identity(_) => {
                IndexSet::from_iter([self.paying_account.address.into()])
            }
        }
    }
}

pub type ApplicationInputForUnsecurifiedPersonaWithoutXrdBalance =
    AbstractApplicationInputForUnsecurifiedEntityWithoutXrdBalance<Persona>;

// ========================
// SECURIFIED
// ========================
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationInputForSecurifiedEntityWithoutXrdBalance {
    Account(ApplicationInputForSecurifiedAccountWithoutXrdBalance),
    Persona(ApplicationInputForSecurifiedPersonaWithoutXrdBalance),
}

impl ApplicationInputForSecurifiedEntityWithoutXrdBalance {
    pub fn addresses_to_fetch_xrd_balance_for(
        &self,
    ) -> IndexSet<AddressOfPayerOfShieldApplication> {
        match self {
            Self::Account(a) => a.addresses_to_fetch_xrd_balance_for(),
            Self::Persona(p) => p.addresses_to_fetch_xrd_balance_for(),
        }
    }

    pub fn address_of_entity_applying_shield(
        &self,
    ) -> AddressOfAccountOrPersona {
        match self {
            Self::Account(a) => a.entity_input.address(),
            Self::Persona(p) => p.entity_input.address(),
        }
    }

    pub fn get_payer(&self) -> Account {
        match self {
            Self::Account(a) => a.paying_account.clone(),
            Self::Persona(p) => p.paying_account.clone(),
        }
    }

    pub fn fee_tip_percentage(&self) -> Option<u16> {
        match self {
            Self::Account(a) => a.fee_tip_percentage(),
            Self::Persona(p) => p.fee_tip_percentage(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationInputForSecurifiedSpecificEntityWithoutXrdBalance<T>
where
    T: IsEntity,
{
    pub reviewed_manifest: TransactionManifest,
    pub estimated_xrd_fee: Decimal,
    pub entity_input: AbstractSecurifiedEntity<T>,
    pub paying_account: Account,
    fee_tip_percentage: Option<u16>,
}

impl<T: IsEntity>
    ApplicationInputForSecurifiedSpecificEntityWithoutXrdBalance<T>
{
    pub fn new(
        reviewed_manifest: TransactionManifest,
        estimated_xrd_fee: Decimal,
        entity_input: AbstractSecurifiedEntity<T>,
        paying_account: Account,
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

    pub fn addresses_to_fetch_xrd_balance_for(
        &self,
    ) -> IndexSet<AddressOfPayerOfShieldApplication> {
        let mut addresses = IndexSet::from_iter([self
            .entity_input
            .securified_entity_control()
            .access_controller_address()
            .into()]);
        match Into::<AddressOfAccountOrPersona>::into(
            self.entity_input.entity.address(),
        ) {
            AddressOfAccountOrPersona::Account(a) => {
                addresses.insert(a.into());
            }
            AddressOfAccountOrPersona::Identity(_) => {
                // nothing to do
            }
        }
        // N.B. We dont check the XRD balance of the payer since we should not use it to pay for others.
        addresses.insert(self.paying_account.address.into());

        addresses
    }
}

pub type ApplicationInputForSecurifiedAccountWithoutXrdBalance =
    ApplicationInputForSecurifiedSpecificEntityWithoutXrdBalance<Account>;

pub type ApplicationInputForSecurifiedPersonaWithoutXrdBalance =
    ApplicationInputForSecurifiedSpecificEntityWithoutXrdBalance<Persona>;
