use std::{collections::BTreeMap, hash::Hash};

use serde::{Deserialize, Serialize};

use super::{account_address::AccountAddress, network_id::NetworkID};

pub trait Identifiable {
    type ID: Hash;
    fn id(&self) -> Self::ID;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct IdentifiedArray<I: Hash + Clone + PartialEq + Eq + Ord, E: Clone + PartialEq + Eq>(
    BTreeMap<I, E>,
);
pub type IdentifiedArrayOf<IE> = IdentifiedArray<<IE as Identifiable>::ID, IE>;

/// An Entity, either Account or Persona
pub trait Entity {
    type Address;
    fn address(&self) -> Self::Address;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Account {
    pub network_id: NetworkID,
    pub display_name: String,
    pub address: AccountAddress,
}

impl Entity for Account {
    type Address = AccountAddress;

    fn address(&self) -> Self::Address {
        self.address.clone()
    }
}

impl Identifiable for Account {
    type ID = AccountAddress;
    fn id(&self) -> Self::ID {
        self.address.clone()
    }
}
