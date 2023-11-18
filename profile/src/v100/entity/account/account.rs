use nutype::nutype;
use serde::{Deserialize, Serialize};
use std::{
    cell::{Cell, RefCell},
    fmt::Display,
};

use crate::v100::{entity::entity_flags::EntityFlags, networks::network::network_id::NetworkID};

use super::{account_address::AccountAddress, appearance_id::AppearanceID};

/// A network unique account with a unique public address and a set of cryptographic
/// factors used to control it.
///
/// Used to own and control assets on the radix network. Uniquely identified by an
/// account address, e.g.
///
/// `account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease`
///
/// But most commonly users see the address on its abbreviated form:
///
/// `acco...please`
///
/// Accounts have a display name and an appearance id.
///
/// An account can be either controlled by a "Babylon" DeviceFactorSource or a
/// Legacy one imported from Olympia, or a Ledger hardware wallet, which too might
/// have been imported from Olympia.
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    /// The ID of the network this account can be used with.
    pub network_id: NetworkID,

    /// An off-ledger display name or description chosen by the user when she
    /// created this account.
    display_name: RefCell<DisplayName>,

    /// A globally unique identifier of this account, being a human readable
    /// address of an account. Always starts with `"account_"``, for example:
    ///
    /// `account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease`
    ///
    /// Most commonly the user will see this address in its abbreviated
    /// form which is:
    ///
    /// `acco...please`
    ///
    /// No two addresses will ever be the same even for the same factor source
    /// but on different networks, since the public keys controlling the
    /// accounts depend on the network id.
    pub address: AccountAddress,

    /// The visual cue user learns to associated this account with, typically
    /// a beautiful colorful gradient.
    appearance_id: Cell<AppearanceID>,

    /// An order set of `EntityFlag`s used to describe certain Off-ledger
    /// user state about Accounts or Personas, such as if an entity is
    /// marked as hidden or not.
    flags: RefCell<EntityFlags>,
}

#[nutype(
    sanitize(trim)
    validate(not_empty, max_len = 20)
)]
#[derive(Serialize, Deserialize, Clone, Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplayName(String);

impl Default for DisplayName {
    fn default() -> Self {
        Self::new("Unnamed").expect("Default display name")
    }
}

impl Account {
    /// Instantiates an account with a display name, address and appearance id.
    pub fn with_values(
        address: AccountAddress,
        display_name: DisplayName,
        appearance_id: AppearanceID,
    ) -> Self {
        Self {
            network_id: address.network_id,
            address,
            display_name: RefCell::new(display_name),
            appearance_id: Cell::new(appearance_id),
            flags: RefCell::new(EntityFlags::default()),
        }
    }
}

// Getters
impl Account {
    pub fn get_display_name(&self) -> String {
        self.display_name.borrow().clone().to_string()
    }

    pub fn get_flags(&self) -> EntityFlags {
        self.flags.borrow().clone()
    }

    pub fn get_appearance_id(&self) -> AppearanceID {
        self.appearance_id.get().clone()
    }
}

// Setters
impl Account {
    pub fn set_display_name(&self, new: DisplayName) {
        *self.display_name.borrow_mut() = new;
    }

    pub fn set_flags(&self, new: EntityFlags) {
        *self.flags.borrow_mut() = new;
    }

    pub fn set_appearance_id(&mut self, new: AppearanceID) {
        *self.appearance_id.get_mut() = new;
    }
}

impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | {}", self.get_display_name(), self.address)
    }
}

#[cfg(test)]
mod tests {
    use crate::v100::entity::account::account_address::AccountAddress;

    use super::Account;

    #[test]
    fn new_with_address_only() {
        let address: AccountAddress =
            "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease"
                .try_into()
                .unwrap();
        let account = Account {
            address: address.clone(),
            ..Default::default()
        };
        assert_eq!(account.address, address);
    }

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
