use serde::{Deserialize, Serialize};

use super::badge_virtual_source::FactorInstanceBadgeVirtualSource;
use enum_as_inner::EnumAsInner;
/// Either a "physical" badge (NFT) or some source for recreation of a producer
/// of a virtual badge (signature), e.g. a HD derivation path, from which a private key
/// is derived which produces virtual badges (signatures).
#[derive(Serialize, Deserialize, EnumAsInner, Clone, Debug, PartialEq, Eq)]
pub enum FactorInstanceBadge {
    Virtual(FactorInstanceBadgeVirtualSource),
}
