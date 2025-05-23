use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct ComponentEntityRoleAssignmentEntry {
    pub role_key: RoleKey,
    pub assignment: ComponentEntityRoleAssignmentEntryAssignment,
}

impl ComponentEntityRoleAssignmentEntry {
    pub fn new(
        role_key: RoleKey,
        assignment: ComponentEntityRoleAssignmentEntryAssignment,
    ) -> Self {
        Self {
            role_key,
            assignment,
        }
    }
}

impl HasSampleValues for ComponentEntityRoleAssignmentEntry {
    fn sample() -> Self {
        Self::sample_depositor_explicit_allow_all()
    }

    fn sample_other() -> Self {
        Self::sample_withdrawer_explicit_deny_all()
    }
}

impl ComponentEntityRoleAssignmentEntry {
    pub fn sample_depositor_explicit_allow_all() -> Self {
        Self::new(
            RoleKey::main_depositor(),
            ComponentEntityRoleAssignmentEntryAssignment::sample_explicit_allow_all(),
        )
    }

    pub fn sample_depositor_explicit_deny_all() -> Self {
        Self::new(
            RoleKey::main_depositor(),
            ComponentEntityRoleAssignmentEntryAssignment::sample_explicit_deny_all(),
        )
    }

    pub fn sample_withdrawer_explicit_allow_all() -> Self {
        Self::new(
            RoleKey::main_withdrawer(),
            ComponentEntityRoleAssignmentEntryAssignment::sample_explicit_allow_all(),
        )
    }

    pub fn sample_withdrawer_explicit_deny_all() -> Self {
        Self::new(
            RoleKey::main_withdrawer(),
            ComponentEntityRoleAssignmentEntryAssignment::sample_explicit_deny_all(),
        )
    }

    pub fn sample_depositor_owner_allow_all() -> Self {
        Self::new(
            RoleKey::main_depositor(),
            ComponentEntityRoleAssignmentEntryAssignment::sample_owner_allow_all(),
        )
    }

    pub fn sample_withdrawer_owner_allow_all() -> Self {
        Self::new(
            RoleKey::main_withdrawer(),
            ComponentEntityRoleAssignmentEntryAssignment::sample_owner_allow_all(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ComponentEntityRoleAssignmentEntry;

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
