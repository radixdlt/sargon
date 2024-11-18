use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct ComponentEntityRoleAssignmentOwner {
    pub explicit_rule: ExplicitRule,
}

impl ComponentEntityRoleAssignmentOwner {
    pub fn new(explicit_rule: ExplicitRule) -> Self {
        Self { explicit_rule }
    }
}

impl HasSampleValues for ComponentEntityRoleAssignmentOwner {
    fn sample() -> Self {
        Self::sample_allow_all()
    }

    fn sample_other() -> Self {
        Self::sample_deny_all()
    }
}

impl ComponentEntityRoleAssignmentOwner {
    pub fn sample_allow_all() -> Self {
        Self::new(ExplicitRule::AllowAll)
    }

    pub fn sample_deny_all() -> Self {
        Self::new(ExplicitRule::DenyAll)
    }

    pub fn sample_protected() -> Self {
        Self::new(ExplicitRule::Protected)
    }
}
