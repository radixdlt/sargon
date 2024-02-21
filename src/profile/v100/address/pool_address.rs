use crate::prelude::*;

use radix_engine_toolkit::models::canonical_address_types::CanonicalPoolAddress as RetPoolAddress;

use radix_engine_common::types::EntityType as ScryptoEntityType;

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
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    derive_more::FromStr,
    derive_more::Display,
    SerializeDisplay,
    DeserializeFromStr,
    uniffi::Record,
)]
#[display("{secret_magic}")]
pub struct PoolAddress {
    /// @Kotlin / Swift developer: Do NOT use this property/field. Instead use all the provided methods on this address type.
    /// (which are in fact vendored as freestanding global functions,
    /// due to limitations in UniFII as of Feb 2024, but you should
    /// create extension methods on this address type in FFI land, translating
    /// these functions into methods.)
    pub(crate) secret_magic: RetPoolAddress,
}

/// Placeholder to a mainnet PoolAddress with single resource.
#[uniffi::export]
pub fn new_pool_address_placeholder_single() -> PoolAddress {
    PoolAddress::placeholder_mainnet_single_pool()
}

/// Placeholder to a mainnet PoolAddress with two resources.
#[uniffi::export]
pub fn new_pool_address_placeholder_two() -> PoolAddress {
    PoolAddress::placeholder_mainnet_bi_pool()
}

/// Placeholder to a mainnet PoolAddress with three resources.
#[uniffi::export]
pub fn new_pool_address_placeholder_multi() -> PoolAddress {
    PoolAddress::placeholder_mainnet_multi_pool()
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
            _ => panic!("Bug in radix-engine-toolkit's CanonicalPoolAddress implementation, wrong entity type returned")
        }
    }
}

impl HasPlaceholder for PoolAddress {
    fn placeholder() -> Self {
        Self::placeholder_mainnet_bi_pool()
    }

    fn placeholder_other() -> Self {
        Self::placeholder_mainnet_single_pool()
    }
}

impl PoolAddress {
    pub fn placeholder_mainnet_single_pool() -> Self {
        "pool_rdx1c325zs6dz3un8ykkjavy9fkvvyzarkaehgsl408qup6f95aup3le3w"
            .parse()
            .expect("Placeholder")
    }
    pub fn placeholder_mainnet_bi_pool() -> Self {
        "pool_rdx1c5dkfdtdqvczcwzdyvzeuhddyha768p2q28erden533fty8h68ay6m"
            .parse()
            .expect("Placeholder")
    }
    pub fn placeholder_mainnet_multi_pool() -> Self {
        "pool_rdx1cc7etecr23e77z9aqvq9rg43ndh9jkt7dzmaygg4t8c36z8qe6k47t"
            .parse()
            .expect("Placeholder")
    }

    pub fn placeholder_stokenet_single_pool() -> Self {
        "pool_tdx_2_1c3qzq55xdg6a66kn0qsdnw2zwvvxwljx5m3cp7xcdzeym3kpnzmpcp"
            .parse()
            .expect("Placeholder")
    }
    pub fn placeholder_stokenet_bi_pool() -> Self {
        "pool_tdx_2_1c4ml86h8lvfk7jma0jy0vksh8srcxhmtax8nd3aur29qtd2k2wmlzk"
            .parse()
            .expect("Placeholder")
    }
    pub fn placeholder_stokenet_multi_pool() -> Self {
        "pool_tdx_2_1ce2v6h4qqwuy7m55luappx2u2puutgfs9punuz8lpc33xhfh32gsw3"
            .parse()
            .expect("Placeholder")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PoolAddress;

    #[test]
    fn equality() {
        assert_eq!(SUT::placeholder(), SUT::placeholder());
        assert_eq!(SUT::placeholder_other(), SUT::placeholder_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::placeholder(), SUT::placeholder_other());
    }

    #[test]
    fn hash() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                SUT::placeholder_mainnet_single_pool(),
                SUT::placeholder_mainnet_bi_pool(),
                SUT::placeholder_mainnet_multi_pool(),
                SUT::placeholder_stokenet_single_pool(),
                SUT::placeholder_stokenet_bi_pool(),
                SUT::placeholder_stokenet_multi_pool(),
                // twice => duplicates should be removed
                SUT::placeholder_mainnet_single_pool(),
                SUT::placeholder_mainnet_bi_pool(),
                SUT::placeholder_mainnet_multi_pool(),
                SUT::placeholder_stokenet_single_pool(),
                SUT::placeholder_stokenet_bi_pool(),
                SUT::placeholder_stokenet_multi_pool(),
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
        assert_eq!(format!("{a}"), s);
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
            SUT::placeholder_mainnet_single_pool().pool_address_kind(),
            PoolKind::OneResource
        );
        assert_eq!(
            SUT::placeholder_mainnet_bi_pool().pool_address_kind(),
            PoolKind::TwoResources
        );
        assert_eq!(
            SUT::placeholder_mainnet_multi_pool().pool_address_kind(),
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
            pool_address_kind(&SUT::placeholder_mainnet_single_pool()),
            PoolKind::OneResource
        );
        assert_eq!(
            pool_address_kind(&SUT::placeholder_mainnet_bi_pool()),
            PoolKind::TwoResources
        );
        assert_eq!(
            pool_address_kind(&SUT::placeholder_mainnet_multi_pool()),
            PoolKind::MultiResources
        );
    }

    #[test]
    fn placeholder() {
        assert_eq!(
            SUT::placeholder_mainnet_single_pool(),
            new_pool_address_placeholder_single()
        );
        assert_eq!(
            SUT::placeholder_mainnet_bi_pool(),
            new_pool_address_placeholder_two()
        );
        assert_eq!(
            SUT::placeholder_mainnet_multi_pool(),
            new_pool_address_placeholder_multi()
        );
    }
}
