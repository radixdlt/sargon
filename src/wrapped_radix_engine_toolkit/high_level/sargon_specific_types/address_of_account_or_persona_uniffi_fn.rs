use crate::prelude::*;

/// First tries to decode the string as an `AccountAddress`, if that we try
/// as an `IdentityAddress`, if that fails, an error is thrown.
#[uniffi::export]
pub fn new_address_of_account_or_persona_from_bech32(
    string: String,
) -> Result<AddressOfAccountOrPersona> {
    AddressOfAccountOrPersona::new_from_bech32(&string)
}

#[uniffi::export]
pub fn new_address_of_account_or_persona_sample_mainnet(
) -> AddressOfAccountOrPersona {
    AddressOfAccountOrPersona::sample_mainnet()
}

#[uniffi::export]
pub fn new_address_of_account_or_persona_sample_mainnet_other(
) -> AddressOfAccountOrPersona {
    AddressOfAccountOrPersona::sample_mainnet_other()
}

#[uniffi::export]
pub fn new_address_of_account_or_persona_sample_stokenet(
) -> AddressOfAccountOrPersona {
    AddressOfAccountOrPersona::sample_stokenet()
}

#[uniffi::export]
pub fn new_address_of_account_or_persona_sample_stokenet_other(
) -> AddressOfAccountOrPersona {
    AddressOfAccountOrPersona::sample_stokenet_other()
}

/// Returns the bech32 encoded address of the `AddressOfAccountOrPersona`
#[uniffi::export]
pub fn address_of_account_or_persona_to_string(
    address: &AddressOfAccountOrPersona,
) -> String {
    address.to_string()
}

/// Returns the [`NetworkID`] of this [`AddressOfAccountOrPersona`].
#[uniffi::export]
pub fn address_of_account_or_persona_network_id(
    address: &AddressOfAccountOrPersona,
) -> NetworkID {
    address.network_id()
}

/// Returns a new address, with the same node_id, but using `network_id` as
/// network.
#[uniffi::export]
pub fn address_of_account_or_persona_map_to_network(
    address: &AddressOfAccountOrPersona,
    network_id: NetworkID,
) -> AddressOfAccountOrPersona {
    address.map_to_network(network_id)
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AddressOfAccountOrPersona;

    #[test]
    fn bech32_roundtrip() {
        let sut = SUT::sample();
        assert_eq!(
            new_address_of_account_or_persona_from_bech32(
                address_of_account_or_persona_to_string(&sut)
            )
            .unwrap(),
            sut
        );
    }

    #[test]
    fn network_id() {
        assert_eq!(
            address_of_account_or_persona_network_id(&SUT::sample()),
            NetworkID::Mainnet
        );
        assert_eq!(
            address_of_account_or_persona_network_id(&SUT::sample_other()),
            NetworkID::Mainnet
        );
    }

    #[test]
    fn hash_of_sample_values() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_address_of_account_or_persona_sample_mainnet(),
                new_address_of_account_or_persona_sample_mainnet_other(),
                new_address_of_account_or_persona_sample_stokenet(),
                new_address_of_account_or_persona_sample_stokenet_other(),
                // duplicates should be removed
                new_address_of_account_or_persona_sample_mainnet(),
                new_address_of_account_or_persona_sample_mainnet_other(),
                new_address_of_account_or_persona_sample_stokenet(),
                new_address_of_account_or_persona_sample_stokenet_other(),
            ])
            .len(),
            4
        );
    }

    #[test]
    fn map_to_network() {
        let to = NetworkID::Stokenet;
        assert_eq!(
            address_of_account_or_persona_map_to_network(&SUT::sample(), to),
            SUT::Account {
                address: AccountAddress::sample_mainnet().map_to_network(to)
            }
        );
    }
}
