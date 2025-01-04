use crate::prelude::*;
use sargon::FactorInstanceBadgeVirtualSource as InternalFactorInstanceBadgeVirtualSource;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum FactorInstanceBadgeVirtualSource {
    HierarchicalDeterministic {
        value: HierarchicalDeterministicPublicKey,
    },
}

delegate_debug_into!(
    FactorInstanceBadgeVirtualSource,
    InternalFactorInstanceBadgeVirtualSource
);
