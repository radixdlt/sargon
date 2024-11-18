use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct ComponentEntityRoleAssignmentEntryAssignment {
    pub explicit_rule: Option<ExplicitRule>,
}

impl ComponentEntityRoleAssignmentEntryAssignment {
    pub fn new(explicit_rule: impl Into<Option<ExplicitRule>>) -> Self {
        Self {
            explicit_rule: explicit_rule.into(),
        }
    }
}

impl HasSampleValues for ComponentEntityRoleAssignmentEntryAssignment {
    fn sample() -> Self {
        Self::sample_allow_all()
    }

    fn sample_other() -> Self {
        Self::sample_deny_all()
    }
}

impl ComponentEntityRoleAssignmentEntryAssignment {
    pub fn sample_allow_all() -> Self {
        Self::new(ExplicitRule::AllowAll)
    }

    pub fn sample_deny_all() -> Self {
        Self::new(ExplicitRule::DenyAll)
    }
}
