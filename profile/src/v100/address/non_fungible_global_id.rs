use radix_engine_toolkit::models::scrypto::non_fungible_global_id::SerializableNonFungibleGlobalId as EngineSerializableNonFungibleGlobalId;
// use radix_engine_toolkit_uniffi::common::non_fungible::NonFungibleGlobalId as EngineNonFungibleGlobalId;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt::Display,
    hash::{Hash, Hasher},
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct NonFungibleGlobalId(EngineSerializableNonFungibleGlobalId);

impl NonFungibleGlobalId {
    pub fn as_str(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl Display for NonFungibleGlobalId {
    fn fmt(
        &self,
        f: &mut radix_engine_common::prelude::fmt::Formatter<'_>,
    ) -> radix_engine_common::prelude::fmt::Result {
        write!(f, "{}", self.as_str())
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
