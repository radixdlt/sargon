use crate::prelude::*;

#[uniffi::export]
pub fn vault_address_is_fungible(address: &VaultAddress) -> bool {
    address.is_fungible()
}

#[uniffi::export]
pub fn vault_address_is_non_fungible(address: &VaultAddress) -> bool {
    address.is_non_fungible()
}

#[uniffi::export]
pub fn new_vault_address_sample_mainnet_fungible() -> VaultAddress {
    VaultAddress::sample_mainnet()
}

#[uniffi::export]
pub fn new_vault_address_sample_mainnet_non_fungible() -> VaultAddress {
    VaultAddress::sample_mainnet_other()
}

#[uniffi::export]
pub fn new_vault_address_sample_stokenet_fungible() -> VaultAddress {
    VaultAddress::sample_stokenet()
}

#[uniffi::export]
pub fn new_vault_address_sample_stokenet_non_fungible() -> VaultAddress {
    VaultAddress::sample_stokenet_other()
}

#[cfg(test)]
mod uniffi_tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = VaultAddress;

    #[test]
    fn new_from_bech32_get_network_id_and_address() {
        let b32 = "internal_vault_rdx1nz8nsf78zrrxq7r2r059vzsnmleawr36470chegf84p06fhze6wq3d";
        let address = new_vault_address(b32.to_owned()).unwrap();
        assert_eq!(vault_address_network_id(&address), NetworkID::Mainnet);
        assert_eq!(vault_address_bech32_address(&address), b32);
    }

    #[test]
    fn new() {
        let s = "internal_vault_rdx1tz474x29nxxd4k2p2reete9xyz4apawv63dphxkr00qt23vyju49fq";
        let a = SUT::try_from_bech32(s).unwrap();
        let b = new_vault_address(s.to_string()).unwrap();
        assert_eq!(b.address(), s);
        assert_eq!(a, b);
    }

    #[test]
    fn is_fungible() {
        assert!(vault_address_is_fungible(&SUT::sample_mainnet_fungible()));
        assert!(!vault_address_is_non_fungible(
            &SUT::sample_mainnet_fungible()
        ));

        assert!(vault_address_is_fungible(&SUT::sample_stokenet_fungible()));
        assert!(!vault_address_is_non_fungible(
            &SUT::sample_stokenet_fungible()
        ));
    }

    #[test]
    fn sample() {
        assert_eq!(
            new_vault_address_sample_mainnet_fungible(),
            SUT::sample_mainnet_fungible()
        );

        assert_eq!(
            new_vault_address_sample_mainnet_non_fungible(),
            SUT::sample_mainnet_non_fungible()
        );

        assert_eq!(
            new_vault_address_sample_stokenet_fungible(),
            SUT::sample_stokenet_fungible()
        );

        assert_eq!(
            new_vault_address_sample_stokenet_non_fungible(),
            SUT::sample_stokenet_non_fungible()
        );
    }
}
