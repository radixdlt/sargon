use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct StateEntityDetailsResponseComponentDetails {
    pub role_assignments: Option<ComponentEntityRoleAssignments>,
    // Store raw JSON as a string to preserve Eq; transparently (de)serialize as JSON
    #[serde(default, with = "stringified_json")]
    pub state: Option<String>,
}

impl StateEntityDetailsResponseComponentDetails {
    pub fn new(
        role_assignments: impl Into<Option<ComponentEntityRoleAssignments>>,
    ) -> Self {
        Self {
            role_assignments: role_assignments.into(),
            state: None,
        }
    }

    // Try to decode the stored JSON state into a concrete type later
    pub fn try_decode_state<T: serde::de::DeserializeOwned>(
        &self,
    ) -> Result<Option<T>, serde_json::Error> {
        match &self.state {
            Some(json) => serde_json::from_str(json).map(Some),
            None => Ok(None),
        }
    }
}

// Serialize/deserialize Option<String> as arbitrary JSON, keeping the raw text
mod stringified_json {
    use serde::ser::Error as _; // bring `custom` into scope for Serializer::Error
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use serde_json::Value;

    pub fn serialize<S>(v: &Option<String>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match v {
            Some(raw) => {
                let val: Value =
                    serde_json::from_str(raw).map_err(S::Error::custom)?;
                val.serialize(s)
            }
            None => s.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(d: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<Value>::deserialize(d)?;
        Ok(opt.map(|v| v.to_string()))
    }
}
