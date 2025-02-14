use crate::prelude::*;
use sargon::SecuredEntityControl as InternalSecuredEntityControl;

/// Advanced security control of an entity which has been "securified",
/// meaning an MFA security structure (`SecurityStructureOfFactorSources`)
/// which user has created has been applied to it.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct SecuredEntityControl {
    /// Virtual Entity Creation (Factor)Instance
    ///
    /// Optional since if we recovered this SecuredEntityControl part of
    /// account recovery scan we might not know the veci
    pub veci: Option<HierarchicalDeterministicFactorInstance>,

    /// The address of the access controller which controls this entity.
    ///
    /// Looking up the public key (hashes) set in the key-value store at
    /// this address reveals the true factors (public keys) used to protect
    /// this entity. It will be the same as the ones in `security_structure`
    /// if we have not changed them locally, which we should not do unless
    /// we are sure the Ledger corresponds to the values in `security_structure`.
    pub access_controller_address: AccessControllerAddress,

    /// The believed-to-be-current security structure of FactorInstances which
    /// secures this entity.
    pub security_structure: SecurityStructureOfFactorInstances,

    /// A provisional new security structure configuration which user
    /// is about to change to
    pub provisional_securified_config: Option<ProvisionalSecurifiedConfig>,
}

delegate_debug_into!(SecuredEntityControl, InternalSecuredEntityControl);

#[uniffi::export]
pub fn new_secured_entity_control_sample() -> SecuredEntityControl {
    InternalSecuredEntityControl::sample().into()
}

#[uniffi::export]
pub fn new_secured_entity_control_sample_other() -> SecuredEntityControl {
    InternalSecuredEntityControl::sample_other().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secured_entity_control_samples() {
        assert_eq!(
            new_secured_entity_control_sample(),
            new_secured_entity_control_sample()
        );
        assert_eq!(
            new_secured_entity_control_sample_other(),
            new_secured_entity_control_sample_other()
        );
        assert_ne!(
            new_secured_entity_control_sample(),
            new_secured_entity_control_sample_other()
        );
    }
}
