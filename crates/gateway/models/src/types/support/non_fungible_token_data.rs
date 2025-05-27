use crate::prelude::*;

/// A helper struct to group all the resources of a given account.
#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct NonFungibleTokenData {
    pub id: NonFungibleGlobalId,
    pub data: Option<ScryptoSborValue>,
}

impl NonFungibleTokenData {
    pub fn new(id: NonFungibleGlobalId, data: Option<ScryptoSborValue>) -> Self {
        Self {
            id,
            data
        }
    }
}