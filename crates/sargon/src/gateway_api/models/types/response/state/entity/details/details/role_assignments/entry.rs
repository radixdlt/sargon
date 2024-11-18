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
        Self::sample_depositor_allow_all()
    }

    fn sample_other() -> Self {
        Self::sample_withdrawer_deny_all()
    }
}

impl ComponentEntityRoleAssignmentEntry {
    pub fn sample_depositor_allow_all() -> Self {
        Self::new(
            RoleKey::sample_depositor(),
            ComponentEntityRoleAssignmentEntryAssignment::sample_allow_all(),
        )
    }

    pub fn sample_depositor_deny_all() -> Self {
        Self::new(
            RoleKey::sample_depositor(),
            ComponentEntityRoleAssignmentEntryAssignment::sample_deny_all(),
        )
    }

    pub fn sample_withdrawer_allow_all() -> Self {
        Self::new(
            RoleKey::sample_withdrawer(),
            ComponentEntityRoleAssignmentEntryAssignment::sample_allow_all(),
        )
    }

    pub fn sample_withdrawer_deny_all() -> Self {
        Self::new(
            RoleKey::sample_withdrawer(),
            ComponentEntityRoleAssignmentEntryAssignment::sample_deny_all(),
        )
    }
}
