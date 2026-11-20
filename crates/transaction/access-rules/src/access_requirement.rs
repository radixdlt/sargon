use crate::prelude::*;

/// What is required to pass an access rule.
///
/// Every operator of the engine's `AccessRule` is a form of "at least `count`
/// of these": `AnyOf` is at least one, `AllOf` is all of them and `CountOf(n)`
/// is at least `n`. So a single recursive type describes any rule.
///
/// Converting an engine rule into this type keeps its meaning, but not its
/// structure. The reverse is not always possible: the engine's `CountOf` only
/// takes badges, so `AtLeast` over nested requirements can only be written
/// when `count` is one (`AnyOf`) or all of them (`AllOf`).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AccessRequirement {
    /// Passes without any proof, e.g. `AllowAll`.
    Anyone,

    /// Never passes, e.g. `DenyAll`.
    NoOne,

    /// A proof of this badge: a non-fungible, or any amount of a resource.
    Badge { badge: ResourceOrNonFungible },

    /// A proof of at least `amount` of `resource`.
    AmountOf {
        amount: Decimal192,
        resource: ResourceAddress,
    },

    /// At least `count` of `requirements` must pass.
    AtLeast {
        count: u32,
        requirements: Vec<AccessRequirement>,
    },
}

impl AccessRequirement {
    /// The non-fungibles this requirement names as badges, e.g. the
    /// signature badges of the keys that can sign for it.
    pub fn non_fungibles(&self) -> HashSet<NonFungibleGlobalId> {
        match self {
            Self::Anyone | Self::NoOne | Self::AmountOf { .. } => {
                HashSet::new()
            }
            Self::Badge {
                badge: ResourceOrNonFungible::NonFungible { value },
            } => HashSet::from([value.clone()]),
            Self::Badge {
                badge: ResourceOrNonFungible::Resource { .. },
            } => HashSet::new(),
            Self::AtLeast { requirements, .. } => {
                requirements.iter().flat_map(Self::non_fungibles).collect()
            }
        }
    }
}

impl HasSampleValues for AccessRequirement {
    /// A single badge.
    fn sample() -> Self {
        Self::Badge {
            badge: ResourceOrNonFungible::sample(),
        }
    }

    /// One of two badges.
    fn sample_other() -> Self {
        Self::AtLeast {
            count: 1,
            requirements: vec![
                Self::sample(),
                Self::Badge {
                    badge: ResourceOrNonFungible::sample_other(),
                },
            ],
        }
    }
}

#[cfg(test)]
mod non_fungibles_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccessRequirement;

    /// Badges at any depth are listed; resources and amounts aren't, as no
    /// key signs for them.
    #[test]
    fn non_fungible_badges_at_any_depth_are_listed() {
        let nft = |n: u64| {
            NonFungibleGlobalId::new_unchecked(
                ResourceAddress::sample_stokenet_nft_abandon(),
                NonFungibleLocalId::integer(n),
            )
        };
        let badge = |n| SUT::Badge {
            badge: ResourceOrNonFungible::NonFungible { value: nft(n) },
        };
        let sut = SUT::AtLeast {
            count: 1,
            requirements: vec![
                badge(1),
                SUT::AtLeast {
                    count: 2,
                    requirements: vec![badge(2), badge(1)],
                },
                SUT::Badge {
                    badge: ResourceOrNonFungible::Resource {
                        value: ResourceAddress::sample(),
                    },
                },
                SUT::AmountOf {
                    amount: Decimal192::one(),
                    resource: ResourceAddress::sample(),
                },
                SUT::Anyone,
            ],
        };

        assert_eq!(sut.non_fungibles(), HashSet::from([nft(1), nft(2)]));
    }
}
