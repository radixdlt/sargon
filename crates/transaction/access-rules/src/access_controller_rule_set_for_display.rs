use crate::prelude::*;

/// What each role of an access controller requires, as shown to the user.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AccessControllerRuleSetForDisplay {
    pub primary_role: AccessRequirementForDisplay,
    pub recovery_role: AccessRequirementForDisplay,
    pub confirmation_role: AccessRequirementForDisplay,
}

impl AccessControllerRuleSet {
    /// Each role for display, see [`AccessRequirement::for_display`].
    pub fn for_display(self) -> AccessControllerRuleSetForDisplay {
        AccessControllerRuleSetForDisplay {
            primary_role: self.primary_role.for_display(),
            recovery_role: self.recovery_role.for_display(),
            confirmation_role: self.confirmation_role.for_display(),
        }
    }
}

impl AccessControllerRuleSetForDisplay {
    /// Each role with the user's keys marked, see
    /// [`AccessRequirementForDisplay::with_own_factor_sources`].
    pub fn with_own_factor_sources(
        self,
        own_factor_source: &impl Fn(&NonFungibleGlobalId) -> Option<FactorSource>,
    ) -> Self {
        Self {
            primary_role: self
                .primary_role
                .with_own_factor_sources(own_factor_source),
            recovery_role: self
                .recovery_role
                .with_own_factor_sources(own_factor_source),
            confirmation_role: self
                .confirmation_role
                .with_own_factor_sources(own_factor_source),
        }
    }
}

impl AccessControllerRuleSetForDisplay {
    /// Whether any role counts a holder more than once, see
    /// [`AccessRequirementForDisplay::has_overlapping_entries`].
    pub fn has_overlapping_entries(&self) -> bool {
        [
            &self.primary_role,
            &self.recovery_role,
            &self.confirmation_role,
        ]
        .into_iter()
        .any(AccessRequirementForDisplay::has_overlapping_entries)
    }
}

impl HasSampleValues for AccessControllerRuleSetForDisplay {
    fn sample() -> Self {
        Self {
            primary_role: AccessRequirementForDisplay::sample(),
            recovery_role: AccessRequirementForDisplay::sample_other(),
            confirmation_role: AccessRequirementForDisplay::sample_other(),
        }
    }

    fn sample_other() -> Self {
        Self {
            primary_role: AccessRequirementForDisplay::sample_other(),
            recovery_role: AccessRequirementForDisplay::sample(),
            confirmation_role: AccessRequirementForDisplay::NoOne,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hierarchical_deterministic::prelude::HierarchicalDeterministicPublicKey;

    /// From engine rules to what is shown, with a different requirement per
    /// role, so that a swapped role is caught at any step.
    #[test]
    fn each_role_is_converted_simplified_and_resolved() {
        let own_key = NonFungibleGlobalId::from(
            HierarchicalDeterministicPublicKey::sample(),
        );
        let own_factor_source = |id: &NonFungibleGlobalId| {
            (*id == own_key).then(FactorSource::sample)
        };
        let protected = |requirement| {
            ScryptoAccessRule::Protected(
                ScryptoCompositeRequirement::BasicRequirement(requirement),
            )
        };
        let rule_set = ScryptoRuleSet {
            primary_role: protected(ScryptoBasicRequirement::CountOf(
                0,
                vec![],
            )),
            recovery_role: ScryptoAccessRule::DenyAll,
            confirmation_role: protected(ScryptoBasicRequirement::AnyOf(vec![
                ResourceOrNonFungible::NonFungible {
                    value: own_key.clone(),
                }
                .into(),
            ])),
        };

        assert_eq!(
            AccessControllerRuleSet::from((rule_set, NetworkID::Mainnet))
                .for_display()
                .with_own_factor_sources(&own_factor_source),
            AccessControllerRuleSetForDisplay {
                primary_role: AccessRequirementForDisplay::Anyone,
                recovery_role: AccessRequirementForDisplay::NoOne,
                confirmation_role: AccessRequirementForDisplay::Badge {
                    badge: BadgeForDisplay::OwnFactor {
                        factor_source: FactorSource::sample()
                    }
                },
            }
        );
    }
}
