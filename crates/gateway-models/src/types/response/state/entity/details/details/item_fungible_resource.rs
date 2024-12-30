use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct StateEntityDetailsResponseFungibleResourceDetails {
    pub role_assignments: ComponentEntityRoleAssignments,
}

impl StateEntityDetailsResponseFungibleResourceDetails {
    pub fn new(role_assignments: ComponentEntityRoleAssignments) -> Self {
        Self { role_assignments }
    }
}
