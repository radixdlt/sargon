use crate::prelude::*;

/// A requirement as shown to the user: simplified, with each badge resolved
/// to what it is, e.g. one of the user's factors.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AccessRequirementForDisplay {
    /// Passes without any proof.
    Anyone,

    /// Never passes.
    NoOne,

    /// A proof of this badge.
    Badge { badge: BadgeForDisplay },

    /// A proof of at least `amount` of `resource`.
    AmountOf {
        amount: Decimal192,
        resource: ResourceAddress,
    },

    /// At least `count` of `requirements` must pass.
    AtLeast {
        count: u32,
        requirements: Vec<AccessRequirementForDisplay>,
    },
}

/// What a badge is, as far as the wallet knows.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BadgeForDisplay {
    /// A signature with one of the user's factors.
    OwnFactor { factor_source: FactorSource },

    /// A signature with a key the wallet doesn't know, e.g. a co-signer's or a
    /// dApp's.
    UnknownKey { badge: NonFungibleGlobalId },

    /// A non-fungible that isn't a signature, e.g. an NFT badge.
    NonFungible { badge: NonFungibleGlobalId },

    /// Any amount of a resource.
    Resource { resource: ResourceAddress },
}

impl AccessRequirement {
    /// The requirement to show to the user: simplified, see
    /// [`AccessRequirement::simplified`], with each badge resolved.
    /// `own_factor_source` returns the user's factor source whose key a
    /// signature badge is, if any.
    pub fn for_display(
        self,
        own_factor_source: &impl Fn(&NonFungibleGlobalId) -> Option<FactorSource>,
    ) -> AccessRequirementForDisplay {
        self.simplified().into_display(own_factor_source)
    }

    fn into_display(
        self,
        own_factor_source: &impl Fn(&NonFungibleGlobalId) -> Option<FactorSource>,
    ) -> AccessRequirementForDisplay {
        match self {
            Self::Anyone => AccessRequirementForDisplay::Anyone,
            Self::NoOne => AccessRequirementForDisplay::NoOne,
            Self::Badge { badge } => AccessRequirementForDisplay::Badge {
                badge: BadgeForDisplay::resolve(badge, own_factor_source),
            },
            Self::AmountOf { amount, resource } => {
                AccessRequirementForDisplay::AmountOf { amount, resource }
            }
            Self::AtLeast {
                count,
                requirements,
            } => AccessRequirementForDisplay::AtLeast {
                count,
                requirements: requirements
                    .into_iter()
                    .map(|requirement| {
                        requirement.into_display(own_factor_source)
                    })
                    .collect(),
            },
        }
    }
}

impl BadgeForDisplay {
    fn resolve(
        badge: ResourceOrNonFungible,
        own_factor_source: &impl Fn(&NonFungibleGlobalId) -> Option<FactorSource>,
    ) -> Self {
        match badge {
            ResourceOrNonFungible::Resource { value } => {
                Self::Resource { resource: value }
            }
            ResourceOrNonFungible::NonFungible { value }
                if !is_signature_badge(&value) =>
            {
                Self::NonFungible { badge: value }
            }
            ResourceOrNonFungible::NonFungible { value } => {
                match own_factor_source(&value) {
                    Some(factor_source) => Self::OwnFactor { factor_source },
                    None => Self::UnknownKey { badge: value },
                }
            }
        }
    }
}

impl HasSampleValues for AccessRequirementForDisplay {
    /// One of the user's factors.
    fn sample() -> Self {
        Self::Badge {
            badge: BadgeForDisplay::sample(),
        }
    }

    /// One of the user's factors or an unknown key.
    fn sample_other() -> Self {
        Self::AtLeast {
            count: 1,
            requirements: vec![
                Self::sample(),
                Self::Badge {
                    badge: BadgeForDisplay::sample_other(),
                },
            ],
        }
    }
}

impl HasSampleValues for BadgeForDisplay {
    fn sample() -> Self {
        Self::OwnFactor {
            factor_source: FactorSource::sample(),
        }
    }

    fn sample_other() -> Self {
        Self::UnknownKey {
            badge: NonFungibleGlobalId::sample_other(),
        }
    }
}

/// Whether the badge is the implicit badge of a key's signature.
fn is_signature_badge(badge: &NonFungibleGlobalId) -> bool {
    let network_id = badge.resource_address.network_id();
    [SECP256K1_SIGNATURE_RESOURCE, ED25519_SIGNATURE_RESOURCE]
        .into_iter()
        .any(|resource| {
            badge.resource_address
                == ResourceAddress::from((resource, network_id))
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use hierarchical_deterministic::prelude::HierarchicalDeterministicPublicKey;

    fn badge(id: &NonFungibleGlobalId) -> AccessRequirement {
        AccessRequirement::Badge {
            badge: ResourceOrNonFungible::NonFungible { value: id.clone() },
        }
    }

    #[test]
    fn badges_are_shown_by_what_they_are() {
        let own_key = NonFungibleGlobalId::from(
            HierarchicalDeterministicPublicKey::sample(),
        );
        let unknown_key = NonFungibleGlobalId::from(
            HierarchicalDeterministicPublicKey::sample_other(),
        );
        let nft = NonFungibleGlobalId::sample();
        let resource = ResourceAddress::sample();
        let amount = AccessRequirement::AmountOf {
            amount: Decimal192::one(),
            resource,
        };
        // Also claims the NFT, which must not be shown as a factor since it
        // isn't a signature.
        let own_factor_source = |id: &NonFungibleGlobalId| {
            (*id == own_key || *id == nft).then(FactorSource::sample)
        };

        let requirement = AccessRequirement::AtLeast {
            count: 1,
            requirements: vec![
                badge(&own_key),
                badge(&unknown_key),
                badge(&nft),
                AccessRequirement::Badge {
                    badge: ResourceOrNonFungible::Resource { value: resource },
                },
                amount,
            ],
        };

        let shown = |badge| AccessRequirementForDisplay::Badge { badge };
        assert_eq!(
            requirement.for_display(&own_factor_source),
            AccessRequirementForDisplay::AtLeast {
                count: 1,
                requirements: vec![
                    shown(BadgeForDisplay::OwnFactor {
                        factor_source: FactorSource::sample()
                    }),
                    shown(BadgeForDisplay::UnknownKey { badge: unknown_key }),
                    shown(BadgeForDisplay::NonFungible { badge: nft }),
                    shown(BadgeForDisplay::Resource { resource }),
                    AccessRequirementForDisplay::AmountOf {
                        amount: Decimal192::one(),
                        resource
                    },
                ],
            }
        );
    }
}
