use crate::prelude::*;
use sargon::UnsecuredEntityControl as InternalUnsecuredEntityControl;

/// Basic security control of an unsecured entity. When said entity
/// is "securified" it will no longer be controlled by this `UnsecuredEntityControl`
/// but rather by an `AccessControl`. It is a name space holding the
/// single factor instance which was used to create
#[derive(
    Clone,  PartialEq, Eq, Hash,  uniffi::Record,
)]
pub struct UnsecuredEntityControl {
    // /// The factor instance which was used to create this unsecured entity, which
    // /// also controls this entity and is used for signing transactions.
    pub transaction_signing: HierarchicalDeterministicFactorInstance,

    /// The factor instance which can be used for ROLA.
    pub authentication_signing: Option<HierarchicalDeterministicFactorInstance>,
}

impl From<InternalUnsecuredEntityControl> for UnsecuredEntityControl {
    fn from(value: InternalUnsecuredEntityControl) -> Self {
        Self {
            transaction_signing: value.transaction_signing.into(),
            authentication_signing: value.authentication_signing.map(Into::into),
        }
    }
}

impl Into<InternalUnsecuredEntityControl> for UnsecuredEntityControl {
    fn into(self) -> InternalUnsecuredEntityControl {
        InternalUnsecuredEntityControl {
            transaction_signing: self.transaction_signing.into(),
            authentication_signing: self.authentication_signing.map(Into::into),
        }
    }
}

#[uniffi::export]
pub fn new_unsecured_entity_control_sample() -> UnsecuredEntityControl {
    InternalUnsecuredEntityControl::sample()
}

#[uniffi::export]
pub fn new_unsecured_entity_control_sample_other() -> UnsecuredEntityControl {
    InternalUnsecuredEntityControl::sample_other()
}

