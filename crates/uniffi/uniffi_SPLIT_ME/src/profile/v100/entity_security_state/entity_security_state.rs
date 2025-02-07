use crate::prelude::*;
use sargon::EntitySecurityState as InternalEntitySecurityState;

/// Describes the state an entity - Account or Persona - is in, in regards to how
/// the user controls it, i.e. if it is controlled by a single factor (private key)
///  or an `AccessController` with a potential Multi-Factor setup.
#[derive(Clone, PartialEq, Eq, Hash, uniffi::Enum)]
#[allow(clippy::large_enum_variant)]
pub enum EntitySecurityState {
    /// The entity is controlled by a single factor (private key)
    Unsecured { value: UnsecuredEntityControl },

    /// The entity is controlled by multi-factor
    Securified { value: SecuredEntityControl },
}

impl EntitySecurityState {
    pub fn into_internal(&self) -> InternalEntitySecurityState {
        self.clone().into()
    }
}
impl From<InternalEntitySecurityState> for EntitySecurityState {
    fn from(value: InternalEntitySecurityState) -> Self {
        match value {
            InternalEntitySecurityState::Unsecured { value } => {
                EntitySecurityState::Unsecured {
                    value: value.into(),
                }
            }
            InternalEntitySecurityState::Securified { value } => {
                EntitySecurityState::Securified {
                    value: value.into(),
                }
            }
        }
    }
}

impl From<EntitySecurityState> for InternalEntitySecurityState {
    fn from(val: EntitySecurityState) -> Self {
        match val {
            EntitySecurityState::Unsecured { value } => {
                InternalEntitySecurityState::Unsecured {
                    value: value.into_internal(),
                }
            }
            EntitySecurityState::Securified { value } => {
                InternalEntitySecurityState::Securified {
                    value: value.into_internal(),
                }
            }
        }
    }
}

#[uniffi::export]
pub fn new_entity_security_state_sample() -> EntitySecurityState {
    InternalEntitySecurityState::sample().into()
}

#[uniffi::export]
pub fn new_entity_security_state_sample_other() -> EntitySecurityState {
    InternalEntitySecurityState::sample_other().into()
}
