use crate::prelude::*;
use sargon::EntitySecurityState as InternalEntitySecurityState;

/// Describes the state an entity - Account or Persona - is in, in regards to how
/// the user controls it, i.e. if it is controlled by a single factor (private key)
///  or an `AccessController` with a potential Multi-Factor setup.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
pub enum EntitySecurityState {
    /// The account is controlled by a single factor (private key)
    Unsecured { value: UnsecuredEntityControl },
}

#[uniffi::export]
pub fn new_entity_security_state_sample() -> EntitySecurityState {
    InternalEntitySecurityState::sample().into()
}

#[uniffi::export]
pub fn new_entity_security_state_sample_other() -> EntitySecurityState {
    InternalEntitySecurityState::sample_other().into()
}
