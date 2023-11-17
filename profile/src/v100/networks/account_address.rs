use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{error::Error, utils::string_utils::suffix_string};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccountAddress {
    pub address: String,
}

impl AccountAddress {
    pub fn from_string(s: &str) -> Result<Self, Error> {
        Err(Error::InvalidAccountAddress(s.to_string()))
    }
}

impl Display for AccountAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.address)
    }
}

impl AccountAddress {
    pub fn short(&self) -> String {
        let suffix = suffix_string(6, &self.address);
        format!("{}...{}", &self.address[0..4], suffix)
    }
}

#[cfg(test)]
mod tests {
    use crate::{error::Error, v100::networks::account_address::AccountAddress};

    #[test]
    fn invalid() {
        assert_eq!(
            AccountAddress::from_string("x"),
            Err(Error::InvalidAccountAddress("x".to_string()))
        )
    }

    #[test]
    fn short() {
        let sut = AccountAddress {
            address: "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
                .to_string(),
        };
        assert_eq!(sut.short(), "acco...please");
    }
}
