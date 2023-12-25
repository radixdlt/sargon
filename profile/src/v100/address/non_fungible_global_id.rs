use crate::{CommonError as Error, NonFungibleLocalId};
use radix_engine_common::data::scrypto::model::NonFungibleLocalId as NativeNonFungibleLocalId;
use radix_engine_toolkit_json::models::scrypto::non_fungible_global_id::{
    SerializableNonFungibleGlobalId as EngineSerializableNonFungibleGlobalId,
    SerializableNonFungibleGlobalIdInternal as EngineSerializableNonFungibleGlobalIdInternal,
};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt::Display,
    hash::{Hash, Hasher},
    str::FromStr,
    sync::Arc,
};

use crate::NetworkID;

use super::resource_address::ResourceAddress;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, uniffi::Object)]
pub struct NonFungibleGlobalId(EngineSerializableNonFungibleGlobalId);

#[uniffi::export]
impl NonFungibleGlobalId {
    pub fn as_str(&self) -> String {
        format!("{}", self)
    }

    #[uniffi::constructor]
    pub fn from_str(string: String) -> Result<Arc<Self>, Error> {
        Self::try_from_str(string.as_str()).map(|id| id.into())
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
    pub fn try_from_str(s: &str) -> Result<Self, Error> {
        EngineSerializableNonFungibleGlobalIdInternal::from_str(s)
            .map(|i| Self(EngineSerializableNonFungibleGlobalId(i)))
            .map_err(|_| Error::InvalidNonFungibleGlobalID)
    }
}

impl TryInto<NonFungibleGlobalId> for &str {
    type Error = crate::CommonError;

    /// Tries to deserializes a bech32 address into an `NonFungibleGlobalId`.
    fn try_into(self) -> Result<NonFungibleGlobalId, Self::Error> {
        NonFungibleGlobalId::try_from_str(self)
    }
}

impl NonFungibleGlobalId {
    /// Returns the non-fungible id.
    pub fn local_id(&self) -> Arc<NonFungibleLocalId> {
        let native_id: NativeNonFungibleLocalId =
            self.0 .0.non_fungible_global_id.local_id().clone();
        let id: crate::NonFungibleLocalId = native_id.into();
        Arc::new(id)
    }
}

#[uniffi::export]
impl NonFungibleGlobalId {
    pub fn network_id(&self) -> NetworkID {
        NetworkID::from_repr(self.0 .0.network_id).expect("Valid NetworkID")
    }

    /// Returns the resource address.
    pub fn resource_address(&self) -> Arc<ResourceAddress> {
        let parts: Vec<String> = self
            .to_canonical_string()
            .split(":")
            .map(|p| p.to_string())
            .collect();
        ResourceAddress::new(parts[0].to_string(), self.network_id()).into()
    }

    /// Returns the canonical string representation of a NonFungibleGlobalID: "<resource>:<local>"
    ///
    /// For example:
    ///
    /// `resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<value>`
    pub fn to_canonical_string(&self) -> String {
        format!("{}", self.0 .0)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{
        assert_json_roundtrip, assert_json_value_eq_after_roundtrip,
        assert_json_value_ne_after_roundtrip, NonFungibleLocalId,
    };
    use serde_json::json;

    use super::NonFungibleGlobalId;

    #[test]
    fn test_deserialize() {
        let str = "resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<value>";
        let id: NonFungibleGlobalId = str.try_into().unwrap();
        match id.local_id().as_ref() {
            NonFungibleLocalId::Str { value } => assert_eq!(value, "value"),
            _ => panic!("wrong"),
        }

        assert_eq!(id.to_canonical_string(), str);
    }

    #[test]
    fn test_address() {
        let str = "resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<value>";
        let id: NonFungibleGlobalId = str.try_into().unwrap();
        assert_eq!(
            id.resource_address().address(),
            "resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha"
        );
    }

    #[test]
    fn test_network_id() {
        let str = "resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<value>";
        let id: NonFungibleGlobalId = str.try_into().unwrap();
        assert_eq!(id.as_str(), str);
    }

    #[test]
    fn test_as_str() {
        let str = "resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<value>";
        let id: NonFungibleGlobalId = str.try_into().unwrap();
        assert_eq!(id.as_str(), str);
    }

    #[test]
    fn test_format() {
        let str = "resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<value>";
        let id: NonFungibleGlobalId = str.try_into().unwrap();
        assert_eq!(format!("{}", id), str);
    }

    #[test]
    fn json_roundtrip() {
        let id: NonFungibleGlobalId =
            "resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<value>"
                .try_into()
                .unwrap();

        assert_json_value_eq_after_roundtrip(
            &id,
            json!("resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<value>"),
        );
        assert_json_roundtrip(&id);
        assert_json_value_ne_after_roundtrip(
            &id,
            json!("resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<WRONG>"),
        );
    }

    #[test]
    fn compare() {
        let a: NonFungibleGlobalId =
            "resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<1>"
                .try_into()
                .unwrap();
        let b: NonFungibleGlobalId =
            "resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<2>"
                .try_into()
                .unwrap();
        assert!(a < b);
        assert!(b > a);
    }

    #[test]
    fn hash() {
        let a: NonFungibleGlobalId =
            "resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<1>"
                .try_into()
                .unwrap();
        let b: NonFungibleGlobalId =
            "resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<2>"
                .try_into()
                .unwrap();
        let mut set = HashSet::<NonFungibleGlobalId>::new();
        set.insert(a.clone());
        assert_eq!(set.len(), 1);
        set.insert(a);
        assert_eq!(set.len(), 1);
        set.insert(b);
        assert_eq!(set.len(), 2);
    }
}
