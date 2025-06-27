use crate::prelude::*;

decl_address!(
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
    /// Implementation wise we wrap [Radix Engine Toolkit's `CanonicalPoolAddress`][ret].
    ///
    /// [entt]: https://github.com/radixdlt/radixdlt-scrypto/blob/fc196e21aacc19c0a3dbb13f3cd313dccf4327ca/radix-engine-common/src/types/entity_type.rs
    /// [ret]: https://github.com/radixdlt/radix-engine-toolkit/blob/34fcc3d5953f4fe131d63d4ee2c41259a087e7a5/crates/radix-engine-toolkit/src/models/canonical_address_types.rs#L256-L261
    pool => [
        ScryptoEntityType::GlobalOneResourcePool,
        ScryptoEntityType::GlobalTwoResourcePool,
        ScryptoEntityType::GlobalMultiResourcePool
    ]
);

impl PoolAddress {
    pub fn sample_mainnet() -> Self {
        Self::sample_mainnet_single_pool()
    }

    pub fn sample_mainnet_other() -> Self {
        Self::sample_mainnet_bi_pool()
    }

    pub fn sample_stokenet() -> Self {
        Self::sample_stokenet_single_pool()
    }

    pub fn sample_stokenet_other() -> Self {
        Self::sample_stokenet_bi_pool()
    }
}

/// The kind of the Pool, either One, Two or Multi resources.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

impl HasSampleValues for PoolKind {
    fn sample() -> Self {
        Self::OneResource
    }

    fn sample_other() -> Self {
        Self::TwoResources
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
