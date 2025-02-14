use crate::prelude::*;

pub trait ProfileEntityByAddress {
    fn entity_by_address(
        &self,
        address: AddressOfAccountOrPersona,
    ) -> Result<AccountOrPersona>;
}

pub trait ProfileAccountByAddress {
    fn account_by_address(&self, address: AccountAddress) -> Result<Account>;
}

pub trait ProfilePersonaByAddress {
    fn persona_by_address(&self, address: IdentityAddress) -> Result<Persona>;
}

pub trait GetEntityByAddress:
    ProfileAccountByAddress + ProfileEntityByAddress + ProfilePersonaByAddress
{
}

impl<
        T: ProfileAccountByAddress
            + ProfileEntityByAddress
            + ProfilePersonaByAddress,
    > GetEntityByAddress for T
{
}
