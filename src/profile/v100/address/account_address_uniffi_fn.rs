use crate::prelude::*;

#[uniffi::export]
pub fn new_account_address_from(
    public_key: PublicKey,
    network_id: NetworkID,
) -> AccountAddress {
    AccountAddress::new(public_key, network_id)
}

#[uniffi::export]
pub fn new_account_address_sample_mainnet() -> AccountAddress {
    AccountAddress::sample_mainnet()
}

#[uniffi::export]
pub fn new_account_address_sample_mainnet_other() -> AccountAddress {
    AccountAddress::sample_mainnet_other()
}

#[uniffi::export]
pub fn new_account_address_sample_stokenet() -> AccountAddress {
    AccountAddress::sample_stokenet()
}

#[uniffi::export]
pub fn new_account_address_sample_stokenet_other() -> AccountAddress {
    AccountAddress::sample_stokenet_other()
}

/// Returns `false` for all addresses created with `Ed25519PublicKey`s, i.e.
/// for all accounts created by the Babylon Radix Wallets.
/// Returns `true` for all addresses created with `Secp256k1PublicKey`s, i.e.
/// imported from the Olympia Wallet.
#[uniffi::export]
pub fn account_address_is_legacy(address: &AccountAddress) -> bool {
    address.is_legacy_address()
}

#[cfg(test)]
mod uniffi_tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccountAddress;

    #[test]
    fn new_from_bech32_get_network_id_and_address() {
        let b32 = "account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264";
        let address = new_account_address(b32.to_owned()).unwrap();
        assert_eq!(account_address_network_id(&address), NetworkID::Mainnet);
        assert_eq!(account_address_bech32_address(&address), b32);
    }

    #[test]
    fn address_format_default() {
        let sut: SUT = SUT::try_from_bech32(
            "account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264",
        )
        .unwrap();
        assert_eq!(
            account_address_formatted(&sut, AddressFormat::Default),
            "acco...aer264"
        );
    }

    #[test]
    fn new_from_key() {
        let public_key: PublicKey = Ed25519PublicKey::from_str(
            "3e9b96a2a863f1be4658ea66aa0584d2a8847d4c0f658b20e62e3594d994d73d",
        )
        .unwrap()
        .into();

        let bech32 = "account_rdx129qdd2yp9vs8jkkn2uwn6sw0ejwmcwr3r4c3usr2hp0nau67m2kzdm";
        assert_eq!(
            SUT::new(public_key, NetworkID::Mainnet),
            new_account_address_from(public_key, NetworkID::Mainnet)
        );
        let from_bech32 = new_account_address(bech32.to_string()).unwrap();
        assert_eq!(SUT::try_from_bech32(bech32).unwrap(), from_bech32.clone());
        assert_eq!(from_bech32.address(), bech32)
    }

    #[test]
    fn is_legacy_address() {
        assert!(account_address_is_legacy(&SUT::from_str("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease").unwrap()));
        assert!(!account_address_is_legacy(&SUT::sample_stokenet()));
    }

    #[test]
    fn sample() {
        assert_eq!(new_account_address_sample_mainnet(), SUT::sample_mainnet());

        assert_eq!(
            new_account_address_sample_mainnet_other(),
            SUT::sample_mainnet_other()
        );

        assert_eq!(
            new_account_address_sample_stokenet(),
            SUT::sample_stokenet()
        );

        assert_eq!(
            new_account_address_sample_stokenet_other(),
            SUT::sample_stokenet_other()
        );
    }
}
