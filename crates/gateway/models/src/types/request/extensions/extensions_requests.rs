use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntitiesByRoleRequirementLookupRequest {
    pub requirements: Vec<RoleRequirement>,
}

impl EntitiesByRoleRequirementLookupRequest {
    pub fn new(requirements: Vec<RoleRequirement>) -> Self {
        Self { requirements }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleRequirement {
    pub resource_address: ResourceAddress,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_fungible_id: Option<NonFungibleLocalId>,
}

impl RoleRequirement {
    pub fn new(
        resource_address: ResourceAddress,
        non_fungible_id: Option<NonFungibleLocalId>,
    ) -> Self {
        Self {
            resource_address,
            non_fungible_id,
        }
    }
}

impl From<FactorInstanceBadge> for RoleRequirement {
    fn from(value: FactorInstanceBadge) -> Self {
        let scrypto_resource_or_non_fungible =
            ScryptoResourceOrNonFungible::from(value.clone());
        RoleRequirement::from((
            scrypto_resource_or_non_fungible,
            value.network_id(),
        ))
    }
}

impl From<(ScryptoResourceOrNonFungible, NetworkID)> for RoleRequirement {
    fn from(value: (ScryptoResourceOrNonFungible, NetworkID)) -> Self {
        match value.0 {
            ScryptoResourceOrNonFungible::NonFungible(id) => {
                let global_id = NonFungibleGlobalId::from((id, value.1));

                RoleRequirement::new(
                    global_id.resource_address,
                    Some(global_id.non_fungible_local_id),
                )
            }
            ScryptoResourceOrNonFungible::Resource(address) => {
                RoleRequirement::new(
                    ResourceAddress::from((address, value.1)),
                    None,
                )
            }
        }
    }
}
