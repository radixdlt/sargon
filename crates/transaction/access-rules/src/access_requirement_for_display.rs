use crate::prelude::*;

/// A requirement as shown to the user: simplified, with each badge resolved
/// to what it is, e.g. one of the user's factors.
///
/// Its shape follows from the engine's rules, so hosts only draw:
/// * `Anyone` and `NoOne` for a whole role, never nested;
/// * `AnyOf` and `AllOf` with at least two requirements, alternating: an
///   `AnyOf` never directly holds an `AnyOf`, nor an `AllOf` an `AllOf`;
/// * `CountOf` with only badges, and a count above one and below their
///   number.
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

    /// At least one of `requirements` must pass.
    AnyOf {
        requirements: Vec<AccessRequirementForDisplay>,
    },

    /// All of `requirements` must pass.
    AllOf {
        requirements: Vec<AccessRequirementForDisplay>,
    },

    /// At least `count` of `badges` must be proven.
    CountOf {
        count: u32,
        badges: Vec<BadgeForDisplay>,
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
    /// [`AccessRequirement::simplified`], with each badge shown as what it
    /// is. Every signature badge is an unknown key until
    /// [`AccessRequirementForDisplay::with_own_factor_sources`] marks the
    /// user's keys.
    pub fn for_display(self) -> AccessRequirementForDisplay {
        self.simplified().into_display()
    }

    /// `self` must be simplified.
    fn into_display(self) -> AccessRequirementForDisplay {
        let display_all = |requirements: Vec<Self>| {
            requirements.into_iter().map(Self::into_display).collect()
        };
        match self {
            Self::Anyone => AccessRequirementForDisplay::Anyone,
            Self::NoOne => AccessRequirementForDisplay::NoOne,
            Self::Badge { badge } => AccessRequirementForDisplay::Badge {
                badge: badge.into(),
            },
            Self::AmountOf { amount, resource } => {
                AccessRequirementForDisplay::AmountOf { amount, resource }
            }
            Self::AtLeast {
                count: 1,
                requirements,
            } => AccessRequirementForDisplay::AnyOf {
                requirements: display_all(requirements),
            },
            Self::AtLeast {
                count,
                requirements,
            } if count as usize == requirements.len() => {
                AccessRequirementForDisplay::AllOf {
                    requirements: display_all(requirements),
                }
            }
            Self::AtLeast {
                count,
                requirements,
            } => match Self::only_badges(&requirements) {
                Some(badges) => AccessRequirementForDisplay::CountOf {
                    count,
                    badges: badges.into_iter().map(Into::into).collect(),
                },
                None => {
                    Self::any_of_subsets(count, requirements).into_display()
                }
            },
        }
    }

    fn only_badges(
        requirements: &[Self],
    ) -> Option<Vec<ResourceOrNonFungible>> {
        requirements
            .iter()
            .map(|requirement| match requirement {
                Self::Badge { badge } => Some(badge.clone()),
                _ => None,
            })
            .collect()
    }

    /// "At least `count` of `requirements`" as "any of" its subsets of
    /// `count` requirements, "all of" each: the same meaning, without a count
    /// over anything but badges.
    ///
    /// The engine's `CountOf` only takes badges, so rules converted from the
    /// engine never need this. It keeps a requirement built otherwise exact.
    fn any_of_subsets(count: u32, requirements: Vec<Self>) -> Self {
        Self::AtLeast {
            count: 1,
            requirements: requirements
                .into_iter()
                .combinations(count as usize)
                .map(|subset| Self::AtLeast {
                    count,
                    requirements: subset,
                })
                .collect(),
        }
        .simplified()
    }
}

impl AccessRequirementForDisplay {
    /// Shows the user's keys among the unknown keys as their factor sources.
    /// `own_factor_source` returns the user's factor source whose key a
    /// signature badge is, if any.
    pub fn with_own_factor_sources(
        self,
        own_factor_source: &impl Fn(&NonFungibleGlobalId) -> Option<FactorSource>,
    ) -> Self {
        let mark_all = |requirements: Vec<Self>| {
            requirements
                .into_iter()
                .map(|requirement| {
                    requirement.with_own_factor_sources(own_factor_source)
                })
                .collect()
        };
        match self {
            Self::Badge { badge } => Self::Badge {
                badge: badge.with_own_factor_source(own_factor_source),
            },
            Self::AnyOf { requirements } => Self::AnyOf {
                requirements: mark_all(requirements),
            },
            Self::AllOf { requirements } => Self::AllOf {
                requirements: mark_all(requirements),
            },
            Self::CountOf { count, badges } => Self::CountOf {
                count,
                badges: badges
                    .into_iter()
                    .map(|badge| {
                        badge.with_own_factor_source(own_factor_source)
                    })
                    .collect(),
            },
            Self::Anyone | Self::NoOne | Self::AmountOf { .. } => self,
        }
    }
}

impl AccessRequirementForDisplay {
    /// Whether a group lists two entries the same holder passes together:
    /// the same badge or the same factor of the user twice, or a resource
    /// next to one of its own non-fungibles or amounts.
    ///
    /// The engine counts every entry, so such a group passes with fewer
    /// holders than it lists: "any 2 of these 3" is passed alone by the key
    /// listed twice. A rule that names each holder once never has them, so
    /// one that does must not be shown as if it were shared.
    pub fn has_overlapping_entries(&self) -> bool {
        match self {
            Self::Anyone
            | Self::NoOne
            | Self::Badge { .. }
            | Self::AmountOf { .. } => false,
            Self::AnyOf { requirements } | Self::AllOf { requirements } => {
                any_overlap(requirements.iter().filter_map(Self::passed_with))
                    || requirements.iter().any(Self::has_overlapping_entries)
            }
            Self::CountOf { badges, .. } => {
                any_overlap(badges.iter().map(BadgeForDisplay::passed_with))
            }
        }
    }

    /// What passes this entry of a group, unless it is a group itself.
    fn passed_with(&self) -> Option<PassedWith<'_>> {
        match self {
            Self::Badge { badge } => Some(badge.passed_with()),
            Self::AmountOf { resource, .. } => {
                Some(PassedWith::Resource(resource))
            }
            _ => None,
        }
    }
}

/// What passes a single entry of a group.
enum PassedWith<'a> {
    /// A signature with this factor of the user, whichever of its keys.
    OwnFactor(FactorSourceID),
    NonFungible(&'a NonFungibleGlobalId),
    /// Any proof of the resource, or one of an amount of it.
    Resource(&'a ResourceAddress),
}

/// Whether two of `entries` are passed with the same thing: the same factor
/// or non-fungible, or a resource and itself or one of its non-fungibles.
fn any_overlap<'a>(entries: impl Iterator<Item = PassedWith<'a>>) -> bool {
    let mut factors = HashSet::new();
    let mut non_fungibles = HashSet::new();
    let mut resources = HashSet::new();
    let mut resources_of_non_fungibles = HashSet::new();
    for entry in entries {
        let overlaps = match entry {
            PassedWith::OwnFactor(factor) => !factors.insert(factor),
            PassedWith::NonFungible(id) => {
                resources_of_non_fungibles.insert(id.resource_address);
                !non_fungibles.insert(id)
                    || resources.contains(&id.resource_address)
            }
            PassedWith::Resource(resource) => {
                !resources.insert(*resource)
                    || resources_of_non_fungibles.contains(resource)
            }
        };
        if overlaps {
            return true;
        }
    }
    false
}

impl From<ResourceOrNonFungible> for BadgeForDisplay {
    fn from(badge: ResourceOrNonFungible) -> Self {
        match badge {
            ResourceOrNonFungible::Resource { value } => {
                Self::Resource { resource: value }
            }
            ResourceOrNonFungible::NonFungible { value }
                if is_signature_badge(&value) =>
            {
                Self::UnknownKey { badge: value }
            }
            ResourceOrNonFungible::NonFungible { value } => {
                Self::NonFungible { badge: value }
            }
        }
    }
}

impl BadgeForDisplay {
    fn with_own_factor_source(
        self,
        own_factor_source: &impl Fn(&NonFungibleGlobalId) -> Option<FactorSource>,
    ) -> Self {
        match self {
            Self::UnknownKey { badge } => match own_factor_source(&badge) {
                Some(factor_source) => Self::OwnFactor { factor_source },
                None => Self::UnknownKey { badge },
            },
            badge => badge,
        }
    }

    fn passed_with(&self) -> PassedWith<'_> {
        match self {
            Self::OwnFactor { factor_source } => {
                PassedWith::OwnFactor(factor_source.factor_source_id())
            }
            Self::UnknownKey { badge } | Self::NonFungible { badge } => {
                PassedWith::NonFungible(badge)
            }
            Self::Resource { resource } => PassedWith::Resource(resource),
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
        Self::AnyOf {
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

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccessRequirementForDisplay;

    fn badge(id: &NonFungibleGlobalId) -> AccessRequirement {
        AccessRequirement::Badge {
            badge: ResourceOrNonFungible::NonFungible { value: id.clone() },
        }
    }

    fn at_least(
        count: u32,
        requirements: impl Into<Vec<AccessRequirement>>,
    ) -> AccessRequirement {
        AccessRequirement::AtLeast {
            count,
            requirements: requirements.into(),
        }
    }

    /// Three different non-signature badges, shown as themselves.
    fn nfts() -> [NonFungibleGlobalId; 3] {
        let resource = NonFungibleGlobalId::sample().resource_address;
        [1, 2, 3].map(|id| {
            NonFungibleGlobalId::new_unchecked(
                resource,
                NonFungibleLocalId::integer(id),
            )
        })
    }

    fn shown(id: &NonFungibleGlobalId) -> BadgeForDisplay {
        BadgeForDisplay::NonFungible { badge: id.clone() }
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

        let requirement = at_least(
            1,
            [
                badge(&own_key),
                badge(&unknown_key),
                badge(&nft),
                AccessRequirement::Badge {
                    badge: ResourceOrNonFungible::Resource { value: resource },
                },
                amount,
            ],
        );

        let as_badge = |badge| SUT::Badge { badge };
        assert_eq!(
            requirement
                .for_display()
                .with_own_factor_sources(&own_factor_source),
            SUT::AnyOf {
                requirements: vec![
                    as_badge(BadgeForDisplay::OwnFactor {
                        factor_source: FactorSource::sample()
                    }),
                    as_badge(BadgeForDisplay::UnknownKey {
                        badge: unknown_key
                    }),
                    as_badge(shown(&nft)),
                    as_badge(BadgeForDisplay::Resource { resource }),
                    SUT::AmountOf {
                        amount: Decimal192::one(),
                        resource
                    },
                ],
            }
        );
    }

    #[test]
    fn groups_are_shown_by_their_count() {
        let [a, b, c] = nfts();
        let badges = [badge(&a), badge(&b), badge(&c)];

        assert_eq!(
            at_least(2, badges.clone()).for_display(),
            SUT::CountOf {
                count: 2,
                badges: vec![shown(&a), shown(&b), shown(&c)],
            }
        );
        // The Sargon shield shape: "2 of 3", or an override.
        assert_eq!(
            at_least(1, [at_least(2, badges.clone()), badge(&a)]).for_display(),
            SUT::AnyOf {
                requirements: vec![
                    SUT::CountOf {
                        count: 2,
                        badges: vec![shown(&a), shown(&b), shown(&c)],
                    },
                    SUT::Badge { badge: shown(&a) },
                ],
            }
        );
        assert_eq!(
            at_least(3, badges).for_display(),
            SUT::AllOf {
                requirements: vec![
                    SUT::Badge { badge: shown(&a) },
                    SUT::Badge { badge: shown(&b) },
                    SUT::Badge { badge: shown(&c) },
                ],
            }
        );
    }

    /// "2 of a, a, b" reads as shared, but `a` alone passes it.
    #[test]
    fn a_badge_counted_twice_is_an_overlap() {
        let [a, b, c] = nfts();
        let alone = AuthZone {
            implicit_non_fungibles: [a.clone()].into(),
            proofs: vec![],
        };

        let counted_twice = at_least(2, [badge(&a), badge(&a), badge(&b)]);
        assert!(counted_twice.is_satisfied_by(&alone));
        assert_eq!(
            counted_twice.clone().for_display(),
            SUT::CountOf {
                count: 2,
                badges: vec![shown(&a), shown(&a), shown(&b)],
            }
        );
        assert!(counted_twice.for_display().has_overlapping_entries());

        // "All of a, a": `a` alone.
        let all_of_one = at_least(2, [badge(&a), badge(&a)]);
        assert!(all_of_one.is_satisfied_by(&alone));
        assert!(all_of_one.for_display().has_overlapping_entries());

        // At any depth.
        assert!(at_least(
            1,
            [badge(&c), at_least(2, [badge(&a), badge(&a), badge(&b)])]
        )
        .for_display()
        .has_overlapping_entries());
    }

    #[test]
    fn each_holder_named_once_is_no_overlap() {
        let [a, b, c] = nfts();
        let badges = [badge(&a), badge(&b), badge(&c)];

        assert!(!badge(&a).for_display().has_overlapping_entries());
        assert!(!at_least(2, badges.clone())
            .for_display()
            .has_overlapping_entries());
        assert!(!at_least(3, badges.clone())
            .for_display()
            .has_overlapping_entries());
        // The same badge in two groups is shown as what it is: an
        // alternative to the group.
        assert!(!at_least(1, [at_least(2, badges), badge(&a)])
            .for_display()
            .has_overlapping_entries());
    }

    /// A proof of a non-fungible is also a proof of its resource, and of an
    /// amount of it.
    #[test]
    fn a_resource_overlaps_its_own_non_fungibles_and_amounts() {
        let [a, b, _] = nfts();
        let resource = a.resource_address;
        let any_of_resource = AccessRequirement::Badge {
            badge: ResourceOrNonFungible::Resource { value: resource },
        };
        let amount_of_resource = AccessRequirement::AmountOf {
            amount: Decimal192::one(),
            resource,
        };

        assert!(at_least(2, [badge(&a), any_of_resource.clone(), badge(&b)])
            .for_display()
            .has_overlapping_entries());
        assert!(at_least(2, [amount_of_resource, badge(&a)])
            .for_display()
            .has_overlapping_entries());
        assert!(at_least(2, [any_of_resource.clone(), any_of_resource])
            .for_display()
            .has_overlapping_entries());
    }

    /// Two keys of one factor are signed for together.
    #[test]
    fn a_factor_of_the_user_listed_twice_is_an_overlap() {
        let own_key = NonFungibleGlobalId::from(
            HierarchicalDeterministicPublicKey::sample(),
        );
        let other_own_key = NonFungibleGlobalId::from(
            HierarchicalDeterministicPublicKey::sample_other(),
        );
        let [a, _, _] = nfts();
        let requirement =
            at_least(2, [badge(&own_key), badge(&other_own_key), badge(&a)])
                .for_display();

        assert!(!requirement.has_overlapping_entries());
        assert!(requirement
            .with_own_factor_sources(&|_| Some(FactorSource::sample()))
            .has_overlapping_entries());
    }

    /// Only for requirements not converted from the engine.
    #[test]
    fn count_over_groups_is_shown_as_any_of_its_subsets() {
        let [a, b, c] = nfts();
        let groups = [
            at_least(1, [badge(&a), badge(&b)]),
            at_least(2, [badge(&b), badge(&c)]),
            badge(&c),
        ];
        let requirement = at_least(2, groups.clone());

        let rewritten = AccessRequirement::any_of_subsets(2, groups.into());
        for subset in 0..1usize << 3 {
            let auth_zone = AuthZone {
                implicit_non_fungibles: nfts()
                    .into_iter()
                    .enumerate()
                    .filter(|(index, _)| subset & (1 << index) != 0)
                    .map(|(_, id)| id)
                    .collect(),
                proofs: vec![],
            };
            assert_eq!(
                rewritten.is_satisfied_by(&auth_zone),
                requirement.is_satisfied_by(&auth_zone)
            );
        }

        let any_of = |requirements| SUT::AnyOf { requirements };
        let all_of = |requirements| SUT::AllOf { requirements };
        let one = |id| SUT::Badge { badge: shown(id) };
        assert_eq!(
            requirement.for_display(),
            any_of(vec![
                all_of(vec![any_of(vec![one(&a), one(&b)]), one(&b), one(&c)]),
                all_of(vec![any_of(vec![one(&a), one(&b)]), one(&c)]),
                all_of(vec![one(&b), one(&c), one(&c)]),
            ])
        );
    }
}
