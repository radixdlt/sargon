use crate::prelude::*;

/// What each role of an access controller requires, e.g. its current rules or
/// those proposed in a recovery.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AccessControllerRuleSet {
    pub primary_role: AccessRequirement,
    pub recovery_role: AccessRequirement,
    pub confirmation_role: AccessRequirement,
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

impl AccessControllerRuleSet {
    /// The non-fungibles any role names as badges.
    pub fn non_fungibles(&self) -> HashSet<NonFungibleGlobalId> {
        [
            &self.primary_role,
            &self.recovery_role,
            &self.confirmation_role,
        ]
        .into_iter()
        .flat_map(AccessRequirement::non_fungibles)
        .collect()
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
