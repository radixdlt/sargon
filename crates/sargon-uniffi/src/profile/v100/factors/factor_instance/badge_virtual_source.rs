use crate::prelude::*;
use sargon::FactorInstanceBadgeVirtualSource as InternalFactorInstanceBadgeVirtualSource;

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum FactorInstanceBadgeVirtualSource {
    HierarchicalDeterministic {
        value: HierarchicalDeterministicPublicKey,
    },
}

impl From<InternalFactorInstanceBadgeVirtualSource>
    for FactorInstanceBadgeVirtualSource
{
    fn from(value: InternalFactorInstanceBadgeVirtualSource) -> Self {
        match value {
            InternalFactorInstanceBadgeVirtualSource::HierarchicalDeterministic { value } => {
                FactorInstanceBadgeVirtualSource::HierarchicalDeterministic {
                    value: value.into(),
                }
            }
        }
    }
}

impl Into<InternalFactorInstanceBadgeVirtualSource>
    for FactorInstanceBadgeVirtualSource
{
    fn into(self) -> InternalFactorInstanceBadgeVirtualSource {
        match self {
            FactorInstanceBadgeVirtualSource::HierarchicalDeterministic { value } => {
                InternalFactorInstanceBadgeVirtualSource::HierarchicalDeterministic {
                    value: value.into(),
                }
            }
        }
    }
}
