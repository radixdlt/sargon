use crate::prelude::*;

use radix_engine_toolkit::models::canonical_address_types::CanonicalPoolAddress as RetPoolAddress;

use radix_engine_common::types::EntityType as ScryptoEntityType;

/// Sample to a mainnet PoolAddress with single resource.
#[uniffi::export]
pub fn new_pool_address_sample_single() -> PoolAddress {
    PoolAddress::sample_mainnet_single_pool()
}

/// Sample to a mainnet PoolAddress with two resources.
#[uniffi::export]
pub fn new_pool_address_sample_two() -> PoolAddress {
    PoolAddress::sample_mainnet_bi_pool()
}

/// Sample to a mainnet PoolAddress with three resources.
#[uniffi::export]
pub fn new_pool_address_sample_multi() -> PoolAddress {
    PoolAddress::sample_mainnet_multi_pool()
}

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

impl PoolAddress {
    /// Returns the kind of pool, either 1, 2 or Multi resources.
    pub fn pool_address_kind(&self) -> PoolKind {
        match self.entity_type() {
            ScryptoEntityType::GlobalOneResourcePool => PoolKind::OneResource,
            ScryptoEntityType::GlobalTwoResourcePool => PoolKind::TwoResources,
            ScryptoEntityType::GlobalMultiResourcePool => PoolKind::MultiResources,
            _ => unreachable!("Bug in radix-engine-toolkit's CanonicalPoolAddress implementation, wrong entity type returned")
        }
    }
}

impl HasSampleValues for PoolAddress {
    fn sample() -> Self {
        Self::sample_mainnet_bi_pool()
    }

    fn sample_other() -> Self {
        Self::sample_mainnet_single_pool()
    }
}

impl PoolAddress {
    pub fn sample_mainnet_single_pool() -> Self {
        "pool_rdx1c325zs6dz3un8ykkjavy9fkvvyzarkaehgsl408qup6f95aup3le3w"
            .parse()
            .expect("Sample")
    }
    pub fn sample_mainnet_bi_pool() -> Self {
        "pool_rdx1c5dkfdtdqvczcwzdyvzeuhddyha768p2q28erden533fty8h68ay6m"
            .parse()
            .expect("Sample")
    }
    pub fn sample_mainnet_multi_pool() -> Self {
        "pool_rdx1cc7etecr23e77z9aqvq9rg43ndh9jkt7dzmaygg4t8c36z8qe6k47t"
            .parse()
            .expect("Sample")
    }

    pub fn sample_stokenet_single_pool() -> Self {
        "pool_tdx_2_1c3qzq55xdg6a66kn0qsdnw2zwvvxwljx5m3cp7xcdzeym3kpnzmpcp"
            .parse()
            .expect("Sample")
    }
    pub fn sample_stokenet_bi_pool() -> Self {
        "pool_tdx_2_1c4ml86h8lvfk7jma0jy0vksh8srcxhmtax8nd3aur29qtd2k2wmlzk"
            .parse()
            .expect("Sample")
    }
    pub fn sample_stokenet_multi_pool() -> Self {
        "pool_tdx_2_1ce2v6h4qqwuy7m55luappx2u2puutgfs9punuz8lpc33xhfh32gsw3"
            .parse()
            .expect("Sample")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PoolAddress;

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
    fn hash() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                SUT::sample_mainnet_single_pool(),
                SUT::sample_mainnet_bi_pool(),
                SUT::sample_mainnet_multi_pool(),
                SUT::sample_stokenet_single_pool(),
                SUT::sample_stokenet_bi_pool(),
                SUT::sample_stokenet_multi_pool(),
                // twice => duplicates should be removed
                SUT::sample_mainnet_single_pool(),
                SUT::sample_mainnet_bi_pool(),
                SUT::sample_mainnet_multi_pool(),
                SUT::sample_stokenet_single_pool(),
                SUT::sample_stokenet_bi_pool(),
                SUT::sample_stokenet_multi_pool(),
            ])
            .len(),
            6
        )
    }

    #[test]
    fn display() {
        let s =
            "pool_rdx1c325zs6dz3un8ykkjavy9fkvvyzarkaehgsl408qup6f95aup3le3w";
        let a = SUT::try_from_bech32(s).unwrap();
        assert_eq!(format!("{}", a), s);
    }

    #[test]
    fn debug() {
        let s =
            "pool_rdx1c325zs6dz3un8ykkjavy9fkvvyzarkaehgsl408qup6f95aup3le3w";
        let a = SUT::try_from_bech32(s).unwrap();
        assert_eq!(format!("{:?}", a), s);
    }

    #[test]
    fn manual_perform_uniffi_conversion() {
        type RetAddr = <SUT as FromRetAddress>::RetAddress;
        let sut = SUT::sample();
        let bech32 = sut.to_string();
        let ret = RetAddr::try_from_bech32(&bech32).unwrap();

        let ffi_side =
            <RetAddr as crate::UniffiCustomTypeConverter>::from_custom(ret);
        assert_eq!(ffi_side, bech32);
        let from_ffi_side =
            <RetAddr as crate::UniffiCustomTypeConverter>::into_custom(
                ffi_side,
            )
            .unwrap();
        assert_eq!(ret, from_ffi_side);
    }

    #[test]
    fn json_roundtrip() {
        let a: SUT =
            "pool_rdx1c325zs6dz3un8ykkjavy9fkvvyzarkaehgsl408qup6f95aup3le3w"
                .parse()
                .unwrap();

        assert_json_value_eq_after_roundtrip(
            &a,
            json!("pool_rdx1c325zs6dz3un8ykkjavy9fkvvyzarkaehgsl408qup6f95aup3le3w"),
        );
        assert_json_roundtrip(&a);
        assert_json_value_ne_after_roundtrip(
            &a,
            json!("pool_rdx1c5dkfdtdqvczcwzdyvzeuhddyha768p2q28erden533fty8h68ay6m"),
        );
    }

    #[test]
    fn json_roundtrip_fails_for_invalid() {
        assert_json_value_fails::<SUT>(json!(
            "pool_rdx1c325zs6dz3un8ykkjavy9fkvvyzarkaehgsl408qup6f95aup3le3x"
        ));
        assert_json_value_fails::<SUT>(
            json!("account_rdx1c325zs6dz3un8ykkjavy9fkvvyzarkaehgsl408qup6f95aup3le3w")
        );
        assert_json_value_fails::<SUT>(json!("super invalid"));
    }

    #[test]
    fn network_id_stokenet() {
        let a: SUT =
            "pool_tdx_2_1c3qzq55xdg6a66kn0qsdnw2zwvvxwljx5m3cp7xcdzeym3kpnzmpcp"
                .parse()
                .unwrap();
        assert_eq!(a.network_id(), NetworkID::Stokenet);
    }

    #[test]
    fn network_id_mainnet() {
        let a: SUT =
            "pool_rdx1c325zs6dz3un8ykkjavy9fkvvyzarkaehgsl408qup6f95aup3le3w"
                .parse()
                .unwrap();
        assert_eq!(a.network_id(), NetworkID::Mainnet);
    }

    #[test]
    fn pool_kind() {
        assert_eq!(
            SUT::sample_mainnet_single_pool().pool_address_kind(),
            PoolKind::OneResource
        );
        assert_eq!(
            SUT::sample_mainnet_bi_pool().pool_address_kind(),
            PoolKind::TwoResources
        );
        assert_eq!(
            SUT::sample_mainnet_multi_pool().pool_address_kind(),
            PoolKind::MultiResources
        );
    }
}

#[cfg(test)]
mod uniffi_tests {
    use crate::{new_resource_address, EntityAddress};

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
    fn sample() {
        assert_eq!(
            SUT::sample_mainnet_single_pool(),
            new_pool_address_sample_single()
        );
        assert_eq!(
            SUT::sample_mainnet_bi_pool(),
            new_pool_address_sample_two()
        );
        assert_eq!(
            SUT::sample_mainnet_multi_pool(),
            new_pool_address_sample_multi()
        );
    }
}
