use enum_as_inner::EnumAsInner;
use hierarchical_deterministic::derivation::hierarchical_deterministic_public_key::HierarchicalDeterministicPublicKey;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, EnumAsInner, Clone, Debug, PartialEq, Eq)]
pub enum FactorInstanceBadgeVirtualSource {
    HierarchicalDeterministic(HierarchicalDeterministicPublicKey),
}
