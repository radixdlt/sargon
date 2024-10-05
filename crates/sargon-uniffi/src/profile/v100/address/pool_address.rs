use crate::prelude::*;

decl_ret_wrapped_address!(
    /// Addresses identifying an OnLedger (OnNetwork) Liquidity Pool (LP) of tokens that users can contribute
    /// Liquidity too, e.g.:
    /// `"pool_rdx1c325zs6dz3un8ykkjavy9fkvvyzarkaehgsl408qup6f95aup3le3w"`
    ///
    /// Typically users contribute to Liquidity Pools by using a Dapp and the Radix Wallet.
    ///
    /// There are fundamentally three different sub-types ([Scrypto's `EntityType`][entt]) of PoolAddresses:
    /// * GlobalOneResourcePool
    /// * GlobalTwoResourcePool
    /// * GlobalMultiResourcePool
    ///
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalPoolAddress`][ret], and
    /// give it UniFFI support, as a `uniffi::Record` (we also own Serde).
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L256-L261
    pool
);

/// The kind of the Pool, either One, Two or Multi resources.
#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum PoolKind {
    /// A Pool to which user can contribute liquidity of a single
    /// resource kind.
    OneResource,

    /// A Pool to which user can contribute liquidity of two different
    /// resources
    TwoResources,

    /// A Pool to which user can contribute liquidity of many different
    /// resources
    MultiResources,
}

/// Returns the kind of pool, either 1, 2 or Multi resources.
#[uniffi::export]
pub fn pool_address_kind(address: &PoolAddress) -> PoolKind {
    address.pool_address_kind()
}

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
