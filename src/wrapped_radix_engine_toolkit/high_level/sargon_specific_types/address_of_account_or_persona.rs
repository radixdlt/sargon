use crate::prelude::*;
use radix_engine::types::GlobalAddress as ScryptoGlobalAddress;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum AddressOfAccountOrPersona {
    Account { address: AccountAddress },
    Persona { address: IdentityAddress },
}

impl From<AccountAddress> for AddressOfAccountOrPersona {
    fn from(address: AccountAddress) -> Self {
        Self::Account { address }
    }
}

impl From<IdentityAddress> for AddressOfAccountOrPersona {
    fn from(address: IdentityAddress) -> Self {
        Self::Persona { address }
    }
}

impl IntoScryptoAddress for AddressOfAccountOrPersona {
    fn scrypto(&self) -> ScryptoGlobalAddress {
        match self {
            AddressOfAccountOrPersona::Account { address } => address.scrypto(),
            AddressOfAccountOrPersona::Persona { address } => address.scrypto(),
        }
    }
    fn network_id(&self) -> NetworkID {
        match self {
            AddressOfAccountOrPersona::Account { address } => {
                address.network_id()
            }
            AddressOfAccountOrPersona::Persona { address } => {
                address.network_id()
            }
        }
    }
}
