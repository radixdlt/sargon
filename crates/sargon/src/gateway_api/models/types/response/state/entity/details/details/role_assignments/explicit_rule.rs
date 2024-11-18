use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
#[serde(tag = "type")]
pub enum ExplicitRule {
    AllowAll,
    DenyAll,
    Protected,
}
