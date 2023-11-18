use serde::{Deserialize, Serialize};
use wallet_kit_test_utils::error::Error;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct DerivationPath(String);

impl DerivationPath {
    pub fn new(path: String) -> Result<Self, Error> {
        Ok(Self(path))
    }
}
impl DerivationPath {
    pub fn placeholder() -> Self {
        Self::new("m/44H/1022H/10H/618H/1460H/0H".to_string()).unwrap()
    }
}
