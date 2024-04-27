use crate::prelude::*;

/// Sample to a mainnet PoolAddress with single resource.
#[uniffi::export]
pub fn new_pool_address_sample_mainnet_single() -> PoolAddress {
    PoolAddress::sample_mainnet()
}

/// Sample to a mainnet PoolAddress with two resources.
#[uniffi::export]
pub fn new_pool_address_sample_mainnet_two() -> PoolAddress {
    PoolAddress::sample_mainnet_other()
}

/// Sample to a mainnet PoolAddress with three resources.
#[uniffi::export]
pub fn new_pool_address_sample_mainnet_multi() -> PoolAddress {
    PoolAddress::sample_mainnet_multi_pool()
}

/// Sample to a stokenet PoolAddress with single resource.
#[uniffi::export]
pub fn new_pool_address_sample_stokenet_single() -> PoolAddress {
    PoolAddress::sample_stokenet()
}

/// Sample to a stokenet PoolAddress with two resources.
#[uniffi::export]
pub fn new_pool_address_sample_stokenet_two() -> PoolAddress {
    PoolAddress::sample_stokenet_other()
}

/// Sample to a stokenet PoolAddress with three resources.
#[uniffi::export]
pub fn new_pool_address_sample_stokenet_multi() -> PoolAddress {
    PoolAddress::sample_stokenet_multi_pool()
}

#[cfg(test)]
mod uniffi_tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PoolAddress;

    #[test]
    fn new_from_bech32_get_network_id_and_address() {
        let b32 =
            "pool_rdx1c325zs6dz3un8ykkjavy9fkvvyzarkaehgsl408qup6f95aup3le3w";
        let address = new_pool_address(b32.to_owned()).unwrap();
        assert_eq!(pool_address_network_id(&address), NetworkID::Mainnet);
        assert_eq!(pool_address_bech32_address(&address), b32);
    }

    #[test]
    fn new() {
        let s =
            "pool_rdx1c325zs6dz3un8ykkjavy9fkvvyzarkaehgsl408qup6f95aup3le3w";
        let a = SUT::try_from_bech32(s).unwrap();
        let b = new_pool_address(s.to_string()).unwrap();
        assert_eq!(b.address(), s);
        assert_eq!(a, b);
    }

    #[test]
    fn pool_kind() {
        assert_eq!(
            pool_address_kind(&SUT::sample_mainnet_single_pool()),
            PoolKind::OneResource
        );
        assert_eq!(
            pool_address_kind(&SUT::sample_mainnet_bi_pool()),
            PoolKind::TwoResources
        );
        assert_eq!(
            pool_address_kind(&SUT::sample_mainnet_multi_pool()),
            PoolKind::MultiResources
        );
    }

    #[test]
    fn hash_of_sample() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_pool_address_sample_mainnet_single(),
                new_pool_address_sample_mainnet_two(),
                new_pool_address_sample_mainnet_multi(),
                new_pool_address_sample_stokenet_single(),
                new_pool_address_sample_stokenet_two(),
                new_pool_address_sample_stokenet_multi(),
                // duplicates should be removed
                new_pool_address_sample_mainnet_single(),
                new_pool_address_sample_mainnet_two(),
                new_pool_address_sample_mainnet_multi(),
                new_pool_address_sample_stokenet_single(),
                new_pool_address_sample_stokenet_two(),
                new_pool_address_sample_stokenet_multi(),
            ])
            .len(),
            6
        );
    }
}
