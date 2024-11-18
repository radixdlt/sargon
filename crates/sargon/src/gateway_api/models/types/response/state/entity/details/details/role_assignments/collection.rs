use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct ComponentEntityRoleAssignments {
    pub entries: Vec<ComponentEntityRoleAssignmentEntry>,
}

impl ComponentEntityRoleAssignments {
    pub fn new(entries: Vec<ComponentEntityRoleAssignmentEntry>) -> Self {
        Self { entries }
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
        Self::new(vec![
            ComponentEntityRoleAssignmentEntry::sample_depositor_allow_all(),
            ComponentEntityRoleAssignmentEntry::sample_withdrawer_allow_all(),
        ])
    }

    pub fn sample_deny_all() -> Self {
        Self::new(vec![
            ComponentEntityRoleAssignmentEntry::sample_depositor_deny_all(),
            ComponentEntityRoleAssignmentEntry::sample_withdrawer_deny_all(),
        ])
    }
}
