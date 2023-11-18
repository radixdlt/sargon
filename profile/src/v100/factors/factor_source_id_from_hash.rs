use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct FactorSourceIDFromHash {}

impl FactorSourceIDFromHash {
    pub fn placeholder() -> Self {
        Self {}
    }
}
