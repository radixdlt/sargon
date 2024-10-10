use crate::prelude::*;
use sargon::HierarchicalDeterministicFactorInstance as InternalHierarchicalDeterministicFactorInstance;

/// A virtual hierarchical deterministic `FactorInstance`
#[derive(Clone, Debug, PartialEq, Eq, Hash,  uniffi::Record)]
pub struct HierarchicalDeterministicFactorInstance {
    pub factor_source_id: FactorSourceIDFromHash,
    pub public_key: HierarchicalDeterministicPublicKey,
}

impl From<InternalHierarchicalDeterministicFactorInstance> for HierarchicalDeterministicFactorInstance {
    fn from(factor_instance: InternalHierarchicalDeterministicFactorInstance) -> Self {
        Self {
            factor_source_id: factor_instance.factor_source_id.into(),
            public_key: factor_instance.public_key.into(),
        }
    }
}

impl Into<InternalHierarchicalDeterministicFactorInstance> for HierarchicalDeterministicFactorInstance {
    fn into(self) -> InternalHierarchicalDeterministicFactorInstance {
        InternalHierarchicalDeterministicFactorInstance {
            factor_source_id: self.factor_source_id.into(),
            public_key: self.public_key.into(),
        }
    }
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

