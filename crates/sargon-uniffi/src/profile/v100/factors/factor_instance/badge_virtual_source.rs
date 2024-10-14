use crate::prelude::*;
use sargon::FactorInstanceBadgeVirtualSource as InternalFactorInstanceBadgeVirtualSource;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
pub enum FactorInstanceBadgeVirtualSource {
    HierarchicalDeterministic {
        value: HierarchicalDeterministicPublicKey,
    },
}
