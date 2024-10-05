use crate::prelude::*;

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, uniffi::Enum,
)]
pub enum FactorInstanceBadgeVirtualSource {
    HierarchicalDeterministic {
        value: HierarchicalDeterministicPublicKey,
    },
}