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
}
