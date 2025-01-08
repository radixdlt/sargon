use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct StateEntityDetailsResponseNonFungibleResourceDetails {
    pub role_assignments: ComponentEntityRoleAssignments,
}

impl StateEntityDetailsResponseNonFungibleResourceDetails {
    pub fn new(role_assignments: ComponentEntityRoleAssignments) -> Self {
        Self { role_assignments }
    }
}
