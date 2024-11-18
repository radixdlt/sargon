use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct ComponentEntityRoleAssignments {
    pub owner: ComponentEntityRoleAssignmentOwner,
    pub entries: Vec<ComponentEntityRoleAssignmentEntry>,
}

impl ComponentEntityRoleAssignments {
    pub fn new(
        owner: ComponentEntityRoleAssignmentOwner,
        entries: Vec<ComponentEntityRoleAssignmentEntry>,
    ) -> Self {
        Self { owner, entries }
    }
}

impl HasSampleValues for ComponentEntityRoleAssignments {
    fn sample() -> Self {
        Self::sample_allow_all()
    }

    fn sample_other() -> Self {
        Self::sample_deny_all()
    }
}

impl ComponentEntityRoleAssignments {
    pub fn sample_allow_all() -> Self {
        Self::new(
            ComponentEntityRoleAssignmentOwner::sample_protected(),
            vec![
                ComponentEntityRoleAssignmentEntry::sample_depositor_explicit_allow_all(
                ),
                ComponentEntityRoleAssignmentEntry::sample_withdrawer_explicit_allow_all(
                ),
            ],
        )
    }

    pub fn sample_deny_all() -> Self {
        Self::new(
            ComponentEntityRoleAssignmentOwner::sample_protected(),
            vec![
                ComponentEntityRoleAssignmentEntry::sample_depositor_explicit_deny_all(),
                ComponentEntityRoleAssignmentEntry::sample_withdrawer_explicit_deny_all(
                ),
            ],
        )
    }
}
