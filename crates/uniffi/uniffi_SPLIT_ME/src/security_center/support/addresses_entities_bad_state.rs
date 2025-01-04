use crate::prelude::*;
use sargon::AddressesOfEntitiesInBadState as InternalAddressesOfEntitiesInBadState;

/// A struct that represents the addresses of entities in a bad state.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct AddressesOfEntitiesInBadState {
    pub accounts: Vec<AccountAddress>,
    pub hidden_accounts: Vec<AccountAddress>,
    pub personas: Vec<IdentityAddress>,
    pub hidden_personas: Vec<IdentityAddress>,
}
