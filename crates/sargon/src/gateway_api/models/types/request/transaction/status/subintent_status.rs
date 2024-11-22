use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubintentStatusRequest {
    /// Bech32m-encoded hash.
    pub(crate) subintent_hash: String,
}

impl SubintentStatusRequest {
    pub fn new(subintent_hash: String) -> Self {
        Self { subintent_hash }
    }
}
