use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum ProfileChange {
    AddedAccount { address: AccountAddress },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum Event {
    ProfileChanged { change: ProfileChange },
}
