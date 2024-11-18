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

    pub fn sample_owner_deny_all() -> Self {
        Self::new(RoleAssignmentResolution::Owner, ExplicitRule::DenyAll)
    }
}
