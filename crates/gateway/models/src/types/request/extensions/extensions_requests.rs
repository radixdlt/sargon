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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_address: Option<ResourceAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_fungible_id: Option<NonFungibleGlobalId>,
}

impl RoleRequirement {
    pub fn with_resource_address(resource_address: ResourceAddress) -> Self {
        Self {
            resource_address: Some(resource_address),
            non_fungible_id: None,
        }
    }

    pub fn with_non_fungible_id(non_fungible_id: NonFungibleGlobalId) -> Self {
        Self {
            resource_address: None,
            non_fungible_id: Some(non_fungible_id),
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
                RoleRequirement::with_non_fungible_id(
                    NonFungibleGlobalId::from((id, value.1)),
                )
            }
            ScryptoResourceOrNonFungible::Resource(address) => {
                RoleRequirement::with_resource_address(ResourceAddress::from((
                    address, value.1,
                )))
            }
        }
    }
}
