use account_for_display::AccountForDisplay;
use entity_for_display::EntityForDisplay;
use persona_for_display::PersonaForDisplay;

use crate::prelude::*;

pub trait ProfileEntityByAddress {
    fn entity_by_address(
        &self,
        address: AddressOfAccountOrPersona,
    ) -> Result<AccountOrPersona>;

    fn entity_for_display_by_address(
        &self,
        address: AddressOfAccountOrPersona,
    ) -> Result<EntityForDisplay> {
        let entity = self.entity_by_address(address)?;
        Ok(EntityForDisplay::from(entity))
    }
}

pub trait ProfileAccountByAddress {
    fn account_by_address(&self, address: AccountAddress) -> Result<Account>;

    fn account_for_display_by_address(
        &self,
        address: AccountAddress,
    ) -> Result<AccountForDisplay> {
        let account = self.account_by_address(address)?;
        Ok(AccountForDisplay::from(account))
    }
}

pub trait ProfilePersonaByAddress {
    fn persona_by_address(&self, address: IdentityAddress) -> Result<Persona>;

    fn persona_for_display_by_address(
        &self,
        address: IdentityAddress,
    ) -> Result<PersonaForDisplay> {
        let persona = self.persona_by_address(address)?;
        Ok(PersonaForDisplay::from(persona))
    }
}

pub trait GetEntityByAddress:
    ProfileAccountByAddress
    + ProfileEntityByAddress
    + ProfilePersonaByAddress
    + Send
    + Sync
{
}

impl<
        T: ProfileAccountByAddress
            + ProfileEntityByAddress
            + ProfilePersonaByAddress
            + Send
            + Sync,
    > GetEntityByAddress for T
{
}
