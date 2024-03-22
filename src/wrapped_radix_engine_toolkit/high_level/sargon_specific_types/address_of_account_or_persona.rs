use crate::prelude::*;

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, derive_more::Display, uniffi::Enum,
)]
pub enum AddressOfAccountOrPersona {
    Account { address: AccountAddress },
    Persona { address: IdentityAddress },
}

impl AddressOfAccountOrPersona {
    /// First tries to decode the string as an `AccountAddress`, if that we try
    /// as an `IdentityAddress`, if that fails, an error is thrown.
    pub fn new_from_bech32(s: &str) -> Result<Self> {
        AccountAddress::from_str(s)
            .map(|address| Self::Account { address })
            .or(IdentityAddress::from_str(s)
                .map(|address| Self::Persona { address }))
            .map_err(|_| CommonError::InvalidAccountAddress {
                bad_value: s.to_owned(),
            })
    }

    /// Returns a new address, with the same node_id, but using `network_id` as
    /// network.
    pub fn map_to_network(&self, network_id: NetworkID) -> Self {
        match self {
            AddressOfAccountOrPersona::Account { address } => {
                address.map_to_network(network_id).into()
            }
            AddressOfAccountOrPersona::Persona { address } => {
                address.map_to_network(network_id).into()
            }
        }
    }
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

    /// Returns the [`NetworkID`] of this [`AddressOfAccountOrPersona`].
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
        Self::sample_mainnet()
    }

    fn sample_other() -> Self {
        Self::sample_mainnet_other()
    }
}

impl AddressOfAccountOrPersona {
    pub(crate) fn sample_mainnet() -> Self {
        AccountAddress::sample_mainnet().into()
    }

    pub(crate) fn sample_mainnet_other() -> Self {
        IdentityAddress::sample_mainnet().into()
    }

    pub(crate) fn sample_stokenet() -> Self {
        AccountAddress::sample_stokenet().into()
    }

    pub(crate) fn sample_stokenet_other() -> Self {
        IdentityAddress::sample_stokenet().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(SUT::sample_other().network_id(), NetworkID::Mainnet);
    }

    #[test]
    fn map_to_network() {
        let to = NetworkID::Stokenet;
        assert_eq!(
            SUT::sample().map_to_network(to),
            SUT::Account {
                address: AccountAddress::sample_mainnet().map_to_network(to)
            }
        );
        assert_eq!(
            SUT::sample_other().map_to_network(to),
            SUT::Persona {
                address: IdentityAddress::sample_mainnet().map_to_network(to)
            }
        );
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

    #[test]
    fn new_from_bech32_invalid_addr() {
        assert!(SUT::new_from_bech32(&PackageAddress::sample().to_string())
            .is_err());
    }
}
