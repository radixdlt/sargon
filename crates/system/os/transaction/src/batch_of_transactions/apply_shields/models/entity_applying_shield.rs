use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityApplyingShield {
    Unsecurified(AnyUnsecurifiedEntity),
    Securified(AnySecurifiedEntity),
}

impl EntityApplyingShield {
    pub fn securified(entity: AnySecurifiedEntity) -> Self {
        Self::Securified(entity)
    }

    pub fn unsecurified_account(account: UnsecurifiedAccount) -> Self {
        Self::Unsecurified(AnyUnsecurifiedEntity::from(account))
    }

    pub fn unsecurified_persona(persona: UnsecurifiedPersona) -> Self {
        Self::Unsecurified(AnyUnsecurifiedEntity::from(persona))
    }
}

impl HasEntityAddress for EntityApplyingShield {
    fn address_erased(&self) -> AddressOfAccountOrPersona {
        match self {
            EntityApplyingShield::Securified(e) => e.address_erased(),
            EntityApplyingShield::Unsecurified(e) => e.address_erased(),
        }
    }
}
