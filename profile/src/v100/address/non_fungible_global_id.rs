use chrono::format::format;
use radix_engine_common::data::scrypto::model::NonFungibleLocalId;
use radix_engine_toolkit::models::scrypto::non_fungible_global_id::SerializableNonFungibleGlobalId as EngineSerializableNonFungibleGlobalId;

use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt::Display,
    hash::{Hash, Hasher},
};
use wallet_kit_common::network_id::NetworkID;

use super::{entity_address::EntityAddress, resource_address::ResourceAddress};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct NonFungibleGlobalId(EngineSerializableNonFungibleGlobalId);

impl NonFungibleGlobalId {
    pub fn as_str(&self) -> String {
        format!("{}", self)
    }
}

impl Display for NonFungibleGlobalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0 .0)
    }
}

impl Ord for NonFungibleGlobalId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(&other.as_str())
    }
}

impl PartialOrd for NonFungibleGlobalId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for NonFungibleGlobalId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl NonFungibleGlobalId {
    pub fn network_id(&self) -> NetworkID {
        NetworkID::from_repr(self.0 .0.network_id).expect("Valid NetworkID")
    }

    /// Returns the resource address.
    pub fn resource_address(&self) -> ResourceAddress {
        let address: String = self
            .to_canonical_string()
            .clone()
            .split(":")
            .into_iter()
            .collect::<Vec<&str>>()
            .first()
            .unwrap()
            .to_string();
        ResourceAddress {
            address,
            network_id: self.network_id(),
        }
    }

    /// Returns the canonical string representation of a NonFungibleGlobalID: "<resource>:<local>"
    ///
    /// For example:
    ///
    /// `resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<value>`
    pub fn to_canonical_string(&self) -> String {
        format!("{}", self.0 .0)
    }

    /// Returns the non-fungible id.
    pub fn local_id(&self) -> &NonFungibleLocalId {
        self.0 .0.non_fungible_global_id.local_id()
    }
}

#[cfg(test)]
mod tests {
    use radix_engine_common::data::scrypto::model::NonFungibleLocalId;
    use serde_json::json;
    use wallet_kit_common::network_id::NetworkID;

    use super::NonFungibleGlobalId;

    #[test]
    fn test_deserialize() {
        let str = "resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<value>";
        let id: NonFungibleGlobalId = serde_json::from_value(json!(str)).unwrap();
        match id.local_id() {
            NonFungibleLocalId::String(v) => assert_eq!(v.value(), "value"),
            _ => panic!("wrong"),
        }
        assert_eq!(
            id.resource_address().address,
            "resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha"
        );
        assert_eq!(id.network_id(), NetworkID::Simulator);
        assert_eq!(id.to_canonical_string(), str);
    }
}
