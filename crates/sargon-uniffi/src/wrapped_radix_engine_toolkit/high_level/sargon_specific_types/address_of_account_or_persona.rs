use crate::prelude::*;

address_union!(
    /// A tagged union of addresses of either an Account or a Persona (IdentityAddress)
    enum AddressOfAccountOrPersona: account, identity
);
