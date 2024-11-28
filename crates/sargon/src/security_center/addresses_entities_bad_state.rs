use crate::prelude::*;

/// A struct that represents the addresses of entities in a bad state.
#[derive(Debug, Clone, PartialEq)]
pub struct AddressesOfEntitiesInBadState {
    pub accounts: Accounts,
    pub hidden_accounts: Accounts,
    pub personas: Personas,
    pub hidden_personas: Personas,
}

impl AddressesOfEntitiesInBadState {
    pub fn new(
        accounts: Accounts,
        hidden_accounts: Accounts,
        personas: Personas,
        hidden_personas: Personas,
    ) -> Self {
        Self {
            accounts,
            hidden_accounts,
            personas,
            hidden_personas,
        }
    }
}

impl HasSampleValues for AddressesOfEntitiesInBadState {
    fn sample() -> Self {
        Self {
            accounts: Accounts::sample(),
            hidden_accounts: Accounts::new(),
            personas: Personas::sample(),
            hidden_personas: Personas::new(),
        }
    }

    fn sample_other() -> Self {
        Self {
            accounts: Accounts::new(),
            hidden_accounts: Accounts::sample_other(),
            personas: Personas::new(),
            hidden_personas: Personas::sample_other(),
        }
    }
}
