use hierarchical_deterministic::derivation::hierarchical_deterministic_public_key::HierarchicalDeterministicPublicKey;

pub enum FactorInstanceBadgeVirtualSource {
    HierarchicalDeterministic(HierarchicalDeterministicPublicKey),
}
