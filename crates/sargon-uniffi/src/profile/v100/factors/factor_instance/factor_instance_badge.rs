use crate::prelude::*;
use sargon::FactorInstanceBadge as InternalFactorInstanceBadge;

/// Either a "physical" badge (resource) or some source for recreation of a producer
/// of a virtual badge (signature), e.g. a HD derivation path, from which a private key
/// is derived which produces virtual badges (signatures).
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum FactorInstanceBadge {
    Virtual {
        value: FactorInstanceBadgeVirtualSource,
    },
    Physical {
        value: ResourceAddress,
    },
}

delegate_debug_into!(FactorInstanceBadge, InternalFactorInstanceBadge);