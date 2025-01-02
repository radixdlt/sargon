use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct StateEntityDetailsResponseComponentDetails {
    pub role_assignments: Option<ComponentEntityRoleAssignments>,
}

impl StateEntityDetailsResponseComponentDetails {
    pub fn new(
        role_assignments: impl Into<Option<ComponentEntityRoleAssignments>>,
    ) -> Self {
        Self {
            role_assignments: role_assignments.into(),
        }
    }
}
