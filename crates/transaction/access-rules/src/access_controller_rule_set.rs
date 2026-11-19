use crate::prelude::*;

/// What each role of an access controller requires, e.g. its current rules or
/// those proposed in a recovery.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AccessControllerRuleSet {
    pub primary_role: AccessRequirement,
    pub recovery_role: AccessRequirement,
    pub confirmation_role: AccessRequirement,
}

impl AccessControllerRuleSet {
    /// Each role simplified, see [`AccessRequirement::simplified`].
    pub fn simplified(self) -> Self {
        Self {
            primary_role: self.primary_role.simplified(),
            recovery_role: self.recovery_role.simplified(),
            confirmation_role: self.confirmation_role.simplified(),
        }
    }
}

impl From<(ScryptoRuleSet, NetworkID)> for AccessControllerRuleSet {
    fn from((rule_set, network_id): (ScryptoRuleSet, NetworkID)) -> Self {
        Self {
            primary_role: (rule_set.primary_role, network_id).into(),
            recovery_role: (rule_set.recovery_role, network_id).into(),
            confirmation_role: (rule_set.confirmation_role, network_id).into(),
        }
    }
}

impl HasSampleValues for AccessControllerRuleSet {
    fn sample() -> Self {
        Self {
            primary_role: AccessRequirement::sample(),
            recovery_role: AccessRequirement::sample_other(),
            confirmation_role: AccessRequirement::sample_other(),
        }
    }

    fn sample_other() -> Self {
        Self {
            primary_role: AccessRequirement::sample_other(),
            recovery_role: AccessRequirement::sample(),
            confirmation_role: AccessRequirement::sample(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccessControllerRuleSet;

    /// A different rule per role, so that a swapped role is caught.
    #[test]
    fn each_role_keeps_its_rule() {
        let badge = ResourceOrNonFungible::sample();
        let rule_set = ScryptoRuleSet {
            primary_role: ScryptoAccessRule::Protected(
                ScryptoCompositeRequirement::BasicRequirement(
                    ScryptoBasicRequirement::CountOf(0, vec![]),
                ),
            ),
            recovery_role: ScryptoAccessRule::DenyAll,
            confirmation_role: ScryptoAccessRule::Protected(
                ScryptoCompositeRequirement::BasicRequirement(
                    ScryptoBasicRequirement::AnyOf(vec![badge.clone().into()]),
                ),
            ),
        };

        assert_eq!(
            SUT::from((rule_set, NetworkID::Mainnet)).simplified(),
            SUT {
                primary_role: AccessRequirement::Anyone,
                recovery_role: AccessRequirement::NoOne,
                confirmation_role: AccessRequirement::Badge { badge },
            }
        );
    }
}
