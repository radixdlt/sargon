use crate::prelude::*;
use sargon::HierarchicalDeterministicFactorInstance as InternalHierarchicalDeterministicFactorInstance;

/// A virtual hierarchical deterministic `FactorInstance`
#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Record)]
pub struct HierarchicalDeterministicFactorInstance {
    pub factor_source_id: FactorSourceIDFromHash,
    pub public_key: HierarchicalDeterministicPublicKey,
}

#[uniffi::export]
pub fn new_hierarchical_deterministic_factor_instance_sample(
) -> HierarchicalDeterministicFactorInstance {
    InternalHierarchicalDeterministicFactorInstance::sample().into()
}

#[uniffi::export]
pub fn new_hierarchical_deterministic_factor_instance_sample_other(
) -> HierarchicalDeterministicFactorInstance {
    InternalHierarchicalDeterministicFactorInstance::sample_other().into()
}
