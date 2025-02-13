use crate::prelude::*;

/// An entity applying a shield - divided into Securified and Unsecurified entities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityApplyingShield {
    Unsecurified(AnyUnsecurifiedEntity),
    Securified(AnySecurifiedEntity),
}

impl EntityApplyingShield {
    pub fn unsecurified_account(entity: UnsecurifiedAccount) -> Self {
        EntityApplyingShield::Unsecurified(AnyUnsecurifiedEntity::from(entity))
    }

    pub fn unsecurified_persona(entity: UnsecurifiedPersona) -> Self {
        EntityApplyingShield::Unsecurified(AnyUnsecurifiedEntity::from(entity))
    }

    pub fn securified(entity: AnySecurifiedEntity) -> Self {
        EntityApplyingShield::Securified(entity)
    }

    pub fn securified_account(account: SecurifiedAccount) -> Self {
        EntityApplyingShield::Securified(AnySecurifiedEntity::from(account))
    }

    pub fn securified_persona(persona: SecurifiedPersona) -> Self {
        EntityApplyingShield::Securified(AnySecurifiedEntity::from(persona))
    }
}

impl EntityApplyingShield {
    pub fn entity(&self) -> AccountOrPersona {
        match self {
            EntityApplyingShield::Unsecurified(unsec) => unsec.entity.clone(),
            EntityApplyingShield::Securified(sec) => sec.entity.clone(),
        }
    }

    pub fn address(&self) -> AddressOfAccountOrPersona {
        self.entity().address()
    }

    pub fn into_account(self) -> Option<Account> {
        match self {
            Self::Unsecurified(unsec) => match unsec.entity {
                AccountOrPersona::AccountEntity(account) => Some(account),
                AccountOrPersona::PersonaEntity(_) => None,
            },
            Self::Securified(sec) => match sec.entity {
                AccountOrPersona::AccountEntity(account) => Some(account),
                AccountOrPersona::PersonaEntity(_) => None,
            },
        }
    }
}
