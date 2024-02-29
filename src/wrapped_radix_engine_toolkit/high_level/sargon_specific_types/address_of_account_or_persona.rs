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

impl HasSampleValues for AddressOfAccountOrPersona {
    fn sample() -> Self {
        AccountAddress::sample().into()
    }

    fn sample_other() -> Self {
        IdentityAddress::sample().into()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AddressOfAccountOrPersona;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn network_id() {
        assert_eq!(SUT::sample().network_id(), NetworkID::Mainnet);
    }

    #[test]
    fn into_scrypto_global_address() {
        assert_eq!(
            SUT::sample().scrypto().as_node_id(),
            &AccountAddress::sample().node_id()
        );
        assert_eq!(
            SUT::sample_other().scrypto().as_node_id(),
            &IdentityAddress::sample().node_id()
        );
    }
}
