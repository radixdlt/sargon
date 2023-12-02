use hierarchical_deterministic::derivation::hierarchical_deterministic_private_key::HierarchicalDeterministicPrivateKey;

use crate::v100::factors::factor_source_id::FactorSourceID;

use super::factor_instance::FactorInstance;

pub struct PrivateHierarchicalDeterministicFactorInstance {
    pub private_key: HierarchicalDeterministicPrivateKey,
    pub factor_source_id: FactorSourceID,
}

impl From<PrivateHierarchicalDeterministicFactorInstance> for HierarchicalDeterministicPrivateKey {
    fn from(value: PrivateHierarchicalDeterministicFactorInstance) -> Self {
        value.private_key
    }
}

impl From<PrivateHierarchicalDeterministicFactorInstance> for FactorInstance {
    fn from(value: PrivateHierarchicalDeterministicFactorInstance) -> Self {
        FactorInstance::with_hierarchical_deterministic_public_key(
            value.factor_source_id,
            value.private_key.public_key(),
        )
    }
}

impl PrivateHierarchicalDeterministicFactorInstance {
    pub fn new(
        private_key: HierarchicalDeterministicPrivateKey,
        factor_source_id: FactorSourceID,
    ) -> Self {
        Self {
            private_key,
            factor_source_id,
        }
    }
}
