use crate::prelude::*;

#[uniffi::export]
pub fn new_non_fungible_resource_address_sample_mainnet(
) -> NonFungibleResourceAddress {
    NonFungibleResourceAddress::sample_mainnet()
}

#[uniffi::export]
pub fn new_non_fungible_resource_address_sample_mainnet_other(
) -> NonFungibleResourceAddress {
    NonFungibleResourceAddress::sample_mainnet_other()
}

#[uniffi::export]
pub fn new_non_fungible_resource_address_sample_stokenet(
) -> NonFungibleResourceAddress {
    NonFungibleResourceAddress::sample_stokenet()
}

#[uniffi::export]
pub fn new_non_fungible_resource_address_sample_stokenet_other(
) -> NonFungibleResourceAddress {
    NonFungibleResourceAddress::sample_stokenet_other()
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NonFungibleResourceAddress;

    #[test]
    fn from_bech32() {
        assert_eq!(new_non_fungible_resource_address("resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa".to_owned()).unwrap(), SUT::sample());
        assert_eq!(new_non_fungible_resource_address("resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd".to_owned()).unwrap(), SUT::sample_other());
    }

    #[test]
    fn to_bech32() {
        assert_eq!(non_fungible_resource_address_bech32_address(&SUT::sample()), "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa");
        assert_eq!(non_fungible_resource_address_bech32_address(&SUT::sample_other()), "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd");
    }

    #[test]
    fn network_id() {
        assert_eq!(
            non_fungible_resource_address_network_id(&SUT::sample_mainnet()),
            NetworkID::Mainnet
        );
        assert_eq!(
            non_fungible_resource_address_network_id(&SUT::sample_stokenet()),
            NetworkID::Stokenet
        );
    }

    #[test]
    fn hash_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_non_fungible_resource_address_sample_mainnet(),
                new_non_fungible_resource_address_sample_mainnet_other(),
                new_non_fungible_resource_address_sample_stokenet(),
                new_non_fungible_resource_address_sample_stokenet_other(),
                // duplicates should be removed
                new_non_fungible_resource_address_sample_mainnet(),
                new_non_fungible_resource_address_sample_mainnet_other(),
                new_non_fungible_resource_address_sample_stokenet(),
                new_non_fungible_resource_address_sample_stokenet_other(),
            ])
            .len(),
            4
        );
    }
}
