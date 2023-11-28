use super::badge_virtual_source::FactorInstanceBadgeVirtualSource;

/// Either a "physical" badge (NFT) or some source for recreation of a producer
/// of a virtual badge (signature), e.g. a HD derivation path, from which a private key
/// is derived which produces virtual badges (signatures).
pub enum FactorInstanceBadge {
    Virtual(FactorInstanceBadgeVirtualSource),
}
