use crate::prelude::*;

/// Either a "physical" badge (resource) or some source for recreation of a producer
/// of a virtual badge (signature), e.g. a HD derivation path, from which a private key
/// is derived which produces virtual badges (signatures).
#[derive(
    EnumAsInner,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    uniffi::Enum,
)]
pub enum FactorInstanceBadge {
    Virtual {
        value: FactorInstanceBadgeVirtualSource,
    },
    Physical {
        value: ResourceAddress,
    },
}