use super::{account_address::AccountAddress, network_id::NetworkID};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, hash::Hash};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Account {
    pub network_id: NetworkID,
    pub display_name: String,
    pub address: AccountAddress,
}

impl Account {
    pub fn with_values(display_name: String, address: AccountAddress) -> Self {
        Self {
            network_id: address.network_id,
            address,
            display_name,
        }
    }
}

impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | {}", self.display_name, self.address)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn json_roundtrip() {
        // let model = assert_eq_after_json_roundtrip(
        //     &model,
        //     r#"
        //     {
        //         "id": "66f07ca2-a9d9-49e5-8152-77aca3d1dd74",
        //         "date": "2023-09-11T16:05:56",
        //         "description": "iPhone"
        //     }
        //     "#,
        // );
    }
}
