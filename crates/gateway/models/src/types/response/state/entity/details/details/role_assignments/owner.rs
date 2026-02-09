use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct ComponentEntityRoleAssignmentOwner {
    pub rule: ExplicitRule,
}

impl ComponentEntityRoleAssignmentOwner {
    pub fn new(rule: ExplicitRule) -> Self {
        Self { rule }
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
        Self::new(ExplicitRule::Protected {
            access_rule: CompositeRequirement::AnyOf {
                access_rules: vec![],
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ComponentEntityRoleAssignmentOwner;

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
