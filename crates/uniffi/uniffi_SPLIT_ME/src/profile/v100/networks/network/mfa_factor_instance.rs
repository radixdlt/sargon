use crate::prelude::*;
use sargon::MFAFactorInstance as InternalMFAFactorInstance;

decl_vec_samples_for!(MFAFactorInstances, MFAFactorInstance);

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct MFAFactorInstance {
    /// The `FactorInstance` used for MFA
    pub factor_instance: FactorInstance,
}

#[uniffi::export]
pub fn new_mfa_factor_instance_sample() -> MFAFactorInstance {
    InternalMFAFactorInstance::sample().into()
}

#[uniffi::export]
pub fn new_mfa_factor_instance_sample_other() -> MFAFactorInstance {
    InternalMFAFactorInstance::sample_other().into()
}
