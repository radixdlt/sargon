use crate::prelude::*;
use sargon::EntitySecurityState as InternalEntitySecurityState;

/// Describes the state an entity - Account or Persona - is in, in regards to how
/// the user controls it, i.e. if it is controlled by a single factor (private key)
///  or an `AccessController` with a potential Multi-Factor setup.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    uniffi::Enum,
)]
pub enum EntitySecurityState {
    /// The account is controlled by a single factor (private key)
    Unsecured {
        value: UnsecuredEntityControl,
    },
}

impl From<InternalEntitySecurityState> for EntitySecurityState {
    fn from(value: InternalEntitySecurityState) -> Self {
        match value {
            InternalEntitySecurityState::Unsecured { value } => Self::Unsecured {
                value: value.into(),
            },
        }
    }
}

impl Into<InternalEntitySecurityState> for EntitySecurityState {
    fn into(self) -> InternalEntitySecurityState {
        match self {
            Self::Unsecured { value } => InternalEntitySecurityState::Unsecured {
                value: value.into(),
            },
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

