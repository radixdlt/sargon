use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct ComponentEntityRoleAssignmentEntryAssignment {
    pub resolution: RoleAssignmentResolution,
    pub explicit_rule: Option<ExplicitRule>,
}

impl ComponentEntityRoleAssignmentEntryAssignment {
    pub fn new(
        resolution: RoleAssignmentResolution,
        explicit_rule: impl Into<Option<ExplicitRule>>,
    ) -> Self {
        Self {
            resolution,
            explicit_rule: explicit_rule.into(),
        }
    }
}

impl HasSampleValues for ComponentEntityRoleAssignmentEntryAssignment {
    fn sample() -> Self {
        Self::sample_explicit_allow_all()
    }

    fn sample_other() -> Self {
        Self::sample_explicit_deny_all()
    }
}

impl ComponentEntityRoleAssignmentEntryAssignment {
    pub fn sample_explicit_allow_all() -> Self {
        Self::new(RoleAssignmentResolution::Explicit, ExplicitRule::AllowAll)
    }

    pub fn sample_explicit_deny_all() -> Self {
        Self::new(RoleAssignmentResolution::Explicit, ExplicitRule::DenyAll)
    }

    pub fn sample_owner_allow_all() -> Self {
        Self::new(RoleAssignmentResolution::Owner, ExplicitRule::AllowAll)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ComponentEntityRoleAssignmentEntryAssignment;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
