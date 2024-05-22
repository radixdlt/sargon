use crate::prelude::*;

#[uniffi::export]
pub fn resource_address_is_fungible(address: &ResourceAddress) -> bool {
    address.is_fungible()
}

#[uniffi::export]
pub fn resource_address_is_non_fungible(address: &ResourceAddress) -> bool {
    address.is_non_fungible()
}

#[uniffi::export]
pub fn new_resource_address_sample_mainnet_xrd() -> ResourceAddress {
    ResourceAddress::sample_mainnet_xrd()
}

#[uniffi::export]
pub fn new_resource_address_sample_mainnet_candy() -> ResourceAddress {
    ResourceAddress::sample_mainnet_candy()
}

#[uniffi::export]
pub fn new_resource_address_sample_mainnet_nft_gc_membership() -> ResourceAddress
{
    ResourceAddress::sample_mainnet_nft_gc_membership()
}

#[uniffi::export]
pub fn new_resource_address_sample_stokenet_xrd() -> ResourceAddress {
    ResourceAddress::sample_stokenet_xrd()
}

#[uniffi::export]
pub fn new_resource_address_sample_stokenet_gum() -> ResourceAddress {
    ResourceAddress::sample_stokenet_gum()
}

#[uniffi::export]
pub fn new_resource_address_sample_stokenet_gc_tokens() -> ResourceAddress {
    ResourceAddress::sample_stokenet_gc_tokens()
}

#[uniffi::export]
pub fn new_resource_address_sample_stokenet_candy() -> ResourceAddress {
    ResourceAddress::sample_stokenet_candy()
}

#[cfg(test)]
mod uniffi_tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ResourceAddress;

    #[test]
    fn new_from_bech32_get_network_id_and_address() {
        let b32 = "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd";
        let address = new_resource_address(b32.to_owned()).unwrap();
        assert_eq!(SUT::try_from_bech32(b32).unwrap(), address);
        assert_eq!(resource_address_network_id(&address), NetworkID::Mainnet);
        assert_eq!(resource_address_bech32_address(&address), b32);
    }

    #[test]
    fn is_fungible() {
        assert!(!resource_address_is_fungible(
            &SUT::sample_mainnet_nft_gc_membership()
        ));
        assert!(resource_address_is_non_fungible(
            &SUT::sample_mainnet_nft_gc_membership()
        ));

        assert!(resource_address_is_fungible(&SUT::sample_mainnet_xrd()));
        assert!(!resource_address_is_non_fungible(&SUT::sample_mainnet_xrd()));
    }

    #[test]
    fn sample() {
        assert_eq!(
            new_resource_address_sample_mainnet_xrd(),
            SUT::sample_mainnet_xrd()
        );
        assert_eq!(
            new_resource_address_sample_mainnet_candy(),
            SUT::sample_mainnet_candy()
        );
        assert_eq!(
            new_resource_address_sample_mainnet_nft_gc_membership(),
            SUT::sample_mainnet_nft_gc_membership()
        );
        assert_eq!(
            new_resource_address_sample_stokenet_xrd(),
            SUT::sample_stokenet_xrd()
        );
        assert_eq!(
            new_resource_address_sample_stokenet_gum(),
            SUT::sample_stokenet_gum()
        );
        assert_eq!(
            new_resource_address_sample_stokenet_gc_tokens(),
            SUT::sample_stokenet_gc_tokens()
        );
        assert_eq!(
            new_resource_address_sample_stokenet_candy(),
            SUT::sample_stokenet_candy()
        );
    }
}
