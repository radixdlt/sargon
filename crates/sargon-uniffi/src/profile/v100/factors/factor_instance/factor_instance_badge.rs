use crate::prelude::*;
use sargon::FactorInstanceBadge as InternalFactorInstanceBadge;

/// Either a "physical" badge (resource) or some source for recreation of a producer
/// of a virtual badge (signature), e.g. a HD derivation path, from which a private key
/// is derived which produces virtual badges (signatures).
#[derive(EnumAsInner, Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum FactorInstanceBadge {
    Virtual {
        value: FactorInstanceBadgeVirtualSource,
    },
    Physical {
        value: ResourceAddress,
    },
}

impl From<InternalFactorInstanceBadge> for FactorInstanceBadge {
    fn from(value: InternalFactorInstanceBadge) -> Self {
        match value {
            InternalFactorInstanceBadge::Virtual { value } => {
                FactorInstanceBadge::Virtual {
                    value: value.into(),
                }
            }
            InternalFactorInstanceBadge::Physical { value } => {
                FactorInstanceBadge::Physical {
                    value: value.into(),
                }
            }
        }
    }
}

impl Into<InternalFactorInstanceBadge> for FactorInstanceBadge {
    fn into(self) -> InternalFactorInstanceBadge {
        match self {
            FactorInstanceBadge::Virtual { value } => {
                InternalFactorInstanceBadge::Virtual {
                    value: value.into(),
                }
            }
            FactorInstanceBadge::Physical { value } => {
                InternalFactorInstanceBadge::Physical {
                    value: value.into(),
                }
            }
        }
    }
}
