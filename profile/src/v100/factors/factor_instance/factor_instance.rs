use hierarchical_deterministic::derivation::hierarchical_deterministic_public_key::HierarchicalDeterministicPublicKey;

use crate::v100::factors::{
    factor_source_id::FactorSourceID,
    hierarchical_deterministic_factor_instance::HierarchicalDeterministicFactorInstance,
};

use super::factor_instance_badge::FactorInstanceBadge;

pub struct FactorInstance {
    /// The ID of the `FactorSource` that was used to produce this
    /// factor instance. We will lookup the `FactorSource` in the
    /// `Profile` and can present user with instruction to re-access
    /// this factor source in order control the `badge`.
    pub factor_source_id: FactorSourceID,

    /// Either a "physical" badge (NFT) or some source for recreation of a producer
    /// of a virtual badge (signature), e.g. a HD derivation path, from which a private key
    /// is derived which produces virtual badges (signatures).
    pub badge: FactorInstanceBadge,
}

impl FactorInstance {
    pub fn new(factor_source_id: FactorSourceID, badge: FactorInstanceBadge) -> Self {
        Self {
            factor_source_id,
            badge,
        }
    }
}
