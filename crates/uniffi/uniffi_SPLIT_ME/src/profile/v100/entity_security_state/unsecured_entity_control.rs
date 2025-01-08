use crate::prelude::*;
use sargon::UnsecuredEntityControl as InternalUnsecuredEntityControl;

/// Basic security control of an unsecured entity. When said entity
/// is "securified" it will no longer be controlled by this `UnsecuredEntityControl`
/// but rather by an `AccessControl`. It is a name space holding the
/// single factor instance which was used to create
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct UnsecuredEntityControl {
    // /// The factor instance which was used to create this unsecured entity, which
    // /// also controls this entity and is used for signing transactions.
    pub transaction_signing: HierarchicalDeterministicFactorInstance,

    /// The provisional security structure configuration
    pub provisional_securified_config: Option<ProvisionalSecurifiedConfig>,
}

#[uniffi::export]
pub fn new_unsecured_entity_control_sample() -> UnsecuredEntityControl {
    InternalUnsecuredEntityControl::sample().into()
}

#[uniffi::export]
pub fn new_unsecured_entity_control_sample_other() -> UnsecuredEntityControl {
    InternalUnsecuredEntityControl::sample_other().into()
}
