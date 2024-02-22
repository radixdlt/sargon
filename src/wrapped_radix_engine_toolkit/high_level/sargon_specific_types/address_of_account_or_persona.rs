use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum AddressOfAccountOrPersona {
    Account { address: AccountAddress },
    Persona { address: IdentityAddress },
}
