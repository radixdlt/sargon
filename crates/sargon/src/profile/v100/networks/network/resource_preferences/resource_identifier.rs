use crate::prelude::*;

/// An enum representation of an resource for which the user can set up its preferences.
#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    Ord,
    PartialOrd,
    derive_more::Display,
    uniffi::Enum,
)]
#[serde(tag = "kind", content = "value")]
#[serde(rename_all = "camelCase")]
pub enum ResourceIdentifier {
    Fungible(ResourceAddress),
    NonFungible(ResourceAddress),
    PoolUnit(PoolAddress),
}

impl IsNetworkAware for ResourceIdentifier {
    fn network_id(&self) -> NetworkID {
        match self {
            Self::NonFungible(resource_address) => {
                resource_address.network_id()
            }
            Self::Fungible(resource_address) => resource_address.network_id(),
            Self::PoolUnit(pool_address) => pool_address.network_id(),
        }
    }
}

impl Identifiable for ResourceIdentifier {
    type ID = Self;
    fn id(&self) -> Self::ID {
        self.clone()
    }
}

impl From<PoolAddress> for ResourceIdentifier {
    fn from(value: PoolAddress) -> Self {
        Self::PoolUnit(value)
    }
}

impl HasSampleValues for ResourceIdentifier {
    fn sample() -> Self {
        Self::sample_fungible_mainnet()
    }

    fn sample_other() -> Self {
        Self::sample_non_fungible_stokenet()
    }
}

#[allow(unused)]
impl ResourceIdentifier {
    pub(crate) fn sample_fungible_mainnet() -> Self {
        Self::Fungible(ResourceAddress::sample_mainnet())
    }

    pub(crate) fn sample_non_fungible_mainnet() -> Self {
        Self::NonFungible(ResourceAddress::sample_mainnet_other())
    }

    pub(crate) fn sample_non_fungible_stokenet() -> Self {
        Self::NonFungible(ResourceAddress::sample_stokenet())
    }

    pub(crate) fn sample_pool_unit_mainnet() -> Self {
        Self::PoolUnit(PoolAddress::sample())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ResourceIdentifier;

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
    fn from() {
        assert_eq!(
            SUT::sample_pool_unit_mainnet(),
            PoolAddress::sample().into()
        )
    }

    #[test]
    fn test_is_network_aware() {
        assert_eq!(
            SUT::sample_fungible_mainnet().network_id(),
            NetworkID::Mainnet
        );
        assert_eq!(
            SUT::sample_non_fungible_stokenet().network_id(),
            NetworkID::Stokenet
        );
    }

    #[test]
    fn json_roundtrip() {
        assert_eq_after_json_roundtrip(
            &SUT::sample_fungible_mainnet(),
            r#"
            {
                "kind": "fungible",
                "value": "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
            }
            "#,
        );

        assert_eq_after_json_roundtrip(
            &SUT::sample_non_fungible_mainnet(),
            r#"
            {
                "kind": "nonFungible",
                "value": "resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j"
            }
            "#,
        );

        assert_eq_after_json_roundtrip(
            &SUT::sample_non_fungible_stokenet(),
            r#"
            {
                "kind": "nonFungible",
                "value": "resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc"
            }
            "#,
        );

        assert_eq_after_json_roundtrip(
            &SUT::sample_pool_unit_mainnet(),
            r#"
            {
                "kind": "poolUnit",
                "value": "pool_rdx1c5dkfdtdqvczcwzdyvzeuhddyha768p2q28erden533fty8h68ay6m"
            }
            "#,
        );
    }
}
