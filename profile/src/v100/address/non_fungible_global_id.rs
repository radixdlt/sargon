use crate::{CommonError as Error, EntityAddress, NonFungibleLocalId};
use radix_engine_common::address::AddressBech32Decoder;
use radix_engine_toolkit_json::models::scrypto::non_fungible_global_id::{
    SerializableNonFungibleGlobalId as EngineSerializableNonFungibleGlobalId,
    SerializableNonFungibleGlobalIdInternal as EngineSerializableNonFungibleGlobalIdInternal,
};
use serde::{de, Deserializer, Serialize, Serializer};
use std::{
    cmp::Ordering,
    fmt::Display,
    hash::{Hash, Hasher},
    str::FromStr,
};
use transaction::prelude::NonFungibleGlobalId as EngineNonFungibleGlobalId;

use crate::NetworkID;

use super::resource_address::ResourceAddress;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct NonFungibleGlobalId {
    pub resource_address: ResourceAddress,
    pub non_fungible_local_id: NonFungibleLocalId,
}

impl From<ResourceAddress> for radix_engine_common::types::ResourceAddress {
    fn from(value: ResourceAddress) -> Self {
        radix_engine_common::types::ResourceAddress::try_from_bech32(
            &AddressBech32Decoder::new(&value.network_id.network_definition()),
            value.address.clone().as_str(),
        )
        .unwrap()
    }
}

impl Serialize for NonFungibleGlobalId {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_canonical_string().as_str())
    }
}

impl<'de> serde::Deserialize<'de> for NonFungibleGlobalId {
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<NonFungibleGlobalId, D::Error> {
        let s = String::deserialize(deserializer)?;
        EngineSerializableNonFungibleGlobalIdInternal::from_str(s.as_str())
            .map(|internal| Self::from_internal_engine(internal))
            .map_err(de::Error::custom)
    }
}

impl NonFungibleGlobalId {
    fn from_internal_engine(internal: EngineSerializableNonFungibleGlobalIdInternal) -> Self {
        let (engine_resource_address, engine_local_id) =
            internal.non_fungible_global_id.into_parts();

        let resource_address_bech32 = ResourceAddress::address_from_node_id(
            engine_resource_address.into_node_id(),
            internal.network_id,
        );

        let non_fungible_local_id: NonFungibleLocalId = engine_local_id.into();
        Self {
            resource_address: ResourceAddress {
                address: resource_address_bech32,
                network_id: NetworkID::from_repr(internal.network_id).unwrap(),
            },
            non_fungible_local_id,
        }
    }

    // fn from_engine(engine: EngineSerializableNonFungibleGlobalId) -> Self {
    //     Self::from_internal_engine(engine.0)
    // }

    fn engine_global_id(&self) -> EngineNonFungibleGlobalId {
        EngineNonFungibleGlobalId::new(
            self.resource_address.clone().into(),
            self.non_fungible_local_id.clone().try_into().unwrap(),
        )
    }

    fn network_id(&self) -> NetworkID {
        self.resource_address.network_id.clone()
    }

    fn engine(&self) -> EngineSerializableNonFungibleGlobalId {
        EngineSerializableNonFungibleGlobalId::new(
            self.engine_global_id().into(),
            self.network_id().discriminant(),
        )
    }
}

impl Display for NonFungibleGlobalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.engine().0)
    }
}

impl Ord for NonFungibleGlobalId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

impl PartialOrd for NonFungibleGlobalId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for NonFungibleGlobalId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_canonical_string().hash(state);
    }
}

impl NonFungibleGlobalId {
    pub fn try_from_str(s: &str) -> Result<Self, Error> {
        EngineSerializableNonFungibleGlobalIdInternal::from_str(s)
            .map(|i| Self::from_internal_engine(i))
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
    /// Returns the canonical string representation of a NonFungibleGlobalID: "<resource>:<local>"
    ///
    /// For example:
    ///
    /// `resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<value>`
    pub fn to_canonical_string(&self) -> String {
        format!("{}", self.engine().0)
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
        let str = "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd:#2244#";
        let id: NonFungibleGlobalId = str.try_into().unwrap();
        match id.clone().non_fungible_local_id {
            NonFungibleLocalId::Integer { value } => assert_eq!(value, 2244),
            _ => panic!("wrong"),
        }

        assert_eq!(id.to_canonical_string(), str);
    }

    #[test]
    fn test_address() {
        let str = "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd:#2244#";
        let id: NonFungibleGlobalId = str.try_into().unwrap();
        assert_eq!(
            id.resource_address.address,
            "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd"
        );
    }

    #[test]
    fn test_network_id() {
        let str = "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd:#2244#";
        let id: NonFungibleGlobalId = str.try_into().unwrap();
        assert_eq!(id.to_string(), str);
    }

    #[test]
    fn test_as_str() {
        let str = "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd:#2244#";
        let id: NonFungibleGlobalId = str.try_into().unwrap();
        assert_eq!(id.to_string(), str);
    }

    #[test]
    fn test_format() {
        let str = "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd:#2244#";
        let id: NonFungibleGlobalId = str.try_into().unwrap();
        assert_eq!(format!("{}", id), str);
    }

    #[test]
    fn json_roundtrip() {
        let id: NonFungibleGlobalId =
            "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd:#2244#"
                .try_into()
                .unwrap();

        assert_json_value_eq_after_roundtrip(
            &id,
            json!("resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd:#2244#"),
        );
        assert_json_roundtrip(&id);
        assert_json_value_ne_after_roundtrip(
            &id,
            json!("resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd:#9999#"),
        );
    }

    #[test]
    fn json_roundtrip_str() {
        let id = NonFungibleGlobalId::try_from_str(
            "resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<foobar>",
        )
        .unwrap();

        assert_json_value_eq_after_roundtrip(
            &id,
            json!("resource_sim1ngktvyeenvvqetnqwysevcx5fyvl6hqe36y3rkhdfdn6uzvt5366ha:<foobar>"),
        );
    }

    #[test]
    fn compare() {
        let a: NonFungibleGlobalId =
            "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd:#3333#"
                .try_into()
                .unwrap();
        let b: NonFungibleGlobalId =
            "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd:#8888#"
                .try_into()
                .unwrap();
        assert!(a < b);
        assert!(b > a);
    }

    #[test]
    fn hash() {
        let a: NonFungibleGlobalId =
            "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd:#1#"
                .try_into()
                .unwrap();
        let b: NonFungibleGlobalId =
            "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd:#2#"
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
