use crate::prelude::*;

/// Maps each engine operator to its "at least n of" form, one to one. The
/// result is not simplified, see [`AccessRequirement::simplified`].
impl From<(ScryptoAccessRule, NetworkID)> for AccessRequirement {
    fn from((rule, network_id): (ScryptoAccessRule, NetworkID)) -> Self {
        match rule {
            ScryptoAccessRule::AllowAll => Self::Anyone,
            ScryptoAccessRule::DenyAll => Self::NoOne,
            ScryptoAccessRule::Protected(requirement) => {
                Self::from_composite(requirement, network_id)
            }
        }
    }
}

impl AccessRequirement {
    fn from_composite(
        requirement: ScryptoCompositeRequirement,
        network_id: NetworkID,
    ) -> Self {
        let nested = |requirements: Vec<ScryptoCompositeRequirement>| {
            requirements
                .into_iter()
                .map(|requirement| {
                    Self::from_composite(requirement, network_id)
                })
                .collect::<Vec<_>>()
        };
        match requirement {
            ScryptoCompositeRequirement::BasicRequirement(basic) => {
                Self::from_basic(basic, network_id)
            }
            ScryptoCompositeRequirement::AnyOf(requirements) => {
                Self::any_of(nested(requirements))
            }
            ScryptoCompositeRequirement::AllOf(requirements) => {
                Self::all_of(nested(requirements))
            }
        }
    }

    fn from_basic(
        requirement: ScryptoBasicRequirement,
        network_id: NetworkID,
    ) -> Self {
        let badge = |badge: ScryptoResourceOrNonFungible| Self::Badge {
            badge: (badge, network_id).into(),
        };
        let badges = |badges: Vec<ScryptoResourceOrNonFungible>| {
            badges.into_iter().map(badge).collect::<Vec<_>>()
        };
        match requirement {
            ScryptoBasicRequirement::Require(required) => badge(required),
            ScryptoBasicRequirement::AmountOf(amount, resource) => {
                Self::AmountOf {
                    amount: amount.into(),
                    resource: (resource, network_id).into(),
                }
            }
            ScryptoBasicRequirement::CountOf(count, required) => {
                Self::AtLeast {
                    count: count.into(),
                    requirements: badges(required),
                }
            }
            ScryptoBasicRequirement::AllOf(required) => {
                Self::all_of(badges(required))
            }
            ScryptoBasicRequirement::AnyOf(required) => {
                Self::any_of(badges(required))
            }
        }
    }

    fn any_of(requirements: Vec<Self>) -> Self {
        Self::AtLeast {
            count: 1,
            requirements,
        }
    }

    fn all_of(requirements: Vec<Self>) -> Self {
        Self::AtLeast {
            count: u32::try_from(requirements.len())
                .expect("Requirements of a rule never exceed u32::MAX"),
            requirements,
        }
    }
}
