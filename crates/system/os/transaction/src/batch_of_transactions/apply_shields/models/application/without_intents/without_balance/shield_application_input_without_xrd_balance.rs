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

    pub fn get_payer(&self) -> Option<Account> {
        match self {
            Self::Unsecurified(u) => u.get_payer(),
            Self::Securified(s) => s.get_payer(),
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

    pub fn get_payer(&self) -> Option<Account> {
        match self {
            Self::Account(a) => a.maybe_paying_account.clone(),
            Self::Persona(p) => Some(p.paying_account.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationInputForUnsecurifiedAccountWithoutXrdBalance {
    pub reviewed_manifest: TransactionManifest,
    pub estimated_xrd_fee: Decimal,
    pub entity_input: UnsecurifiedAccount,
    pub maybe_paying_account: Option<Account>,
}
impl ApplicationInputForUnsecurifiedAccountWithoutXrdBalance {
    pub fn addresses_to_fetch_xrd_balance_for(
        &self,
    ) -> IndexSet<AddressOfPayerOfShieldApplication> {
        let mut addresses =
            IndexSet::from_iter([self.entity_input.entity.address.into()]);
        if let Some(paying_account) = &self.maybe_paying_account {
            addresses.insert(paying_account.address.into());
        }
        addresses
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationInputForUnsecurifiedPersonaWithoutXrdBalance {
    pub reviewed_manifest: TransactionManifest,
    pub estimated_xrd_fee: Decimal,
    pub entity_input: UnsecurifiedPersona,
    pub paying_account: Account,
}
impl ApplicationInputForUnsecurifiedPersonaWithoutXrdBalance {
    pub fn addresses_to_fetch_xrd_balance_for(
        &self,
    ) -> IndexSet<AddressOfPayerOfShieldApplication> {
        IndexSet::from_iter([self.paying_account.address.into()])
    }
}

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

    pub fn get_payer(&self) -> Option<Account> {
        match self {
            Self::Account(a) => a.maybe_paying_account.clone(),
            Self::Persona(p) => p.maybe_paying_account.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationInputForSecurifiedSpecificEntityWithoutXrdBalance<T>
where
    T: IsBaseEntity + std::hash::Hash + Eq + Clone,
{
    pub reviewed_manifest: TransactionManifest,
    pub estimated_xrd_fee: Decimal,
    pub entity_input: AbstractSecurifiedEntity<T>,
    pub maybe_paying_account: Option<Account>,
}

impl<T: IsBaseEntity + std::hash::Hash + Eq + Clone>
    ApplicationInputForSecurifiedSpecificEntityWithoutXrdBalance<T>
{
    pub fn addresses_to_fetch_xrd_balance_for(
        &self,
    ) -> IndexSet<AddressOfPayerOfShieldApplication> {
        let mut addresses = IndexSet::from_iter([self
            .entity_input
            .securified_entity_control()
            .xrd_vault_address()
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
        if let Some(paying_account) = &self.maybe_paying_account {
            addresses.insert(paying_account.address.into());
        }
        addresses
    }
}

pub type ApplicationInputForSecurifiedAccountWithoutXrdBalance =
    ApplicationInputForSecurifiedSpecificEntityWithoutXrdBalance<Account>;

pub type ApplicationInputForSecurifiedPersonaWithoutXrdBalance =
    ApplicationInputForSecurifiedSpecificEntityWithoutXrdBalance<Persona>;
