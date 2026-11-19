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
