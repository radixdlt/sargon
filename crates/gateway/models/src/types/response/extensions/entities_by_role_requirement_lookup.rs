use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct EntitiesByRoleRequirementLookupResponse {
    pub items: Vec<EntitiesByRoleRequirementLookupItem>,
}

impl EntitiesByRoleRequirementLookupResponse {
    pub fn new(items: Vec<EntitiesByRoleRequirementLookupItem>) -> Self {
        Self { items }
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct EntitiesByRoleRequirementLookupItem {
    pub total_count: u64,
    pub requirement: RoleRequirement,
    pub entities: Vec<EntityByRoleRequirement>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct EntityByRoleRequirement {
    pub entity_address: Option<Address>,
    pub first_seen_state_version: u64,
}
