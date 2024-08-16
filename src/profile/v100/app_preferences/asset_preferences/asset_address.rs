use crate::prelude::*;

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
pub enum AssetAddress {
    Fungible(ResourceAddress),
    NonFungible(NonFungibleGlobalId),
    PoolUnit(PoolAddress),
}

impl Identifiable for AssetAddress {
    type ID = Self;
    fn id(&self) -> Self::ID {
        self.clone()
    }
}

impl From<ResourceAddress> for AssetAddress {
    fn from(value: ResourceAddress) -> Self {
        Self::Fungible(value)
    }
}

impl From<NonFungibleGlobalId> for AssetAddress {
    fn from(value: NonFungibleGlobalId) -> Self {
        Self::NonFungible(value)
    }
}

impl From<PoolAddress> for AssetAddress {
    fn from(value: PoolAddress) -> Self {
        Self::PoolUnit(value)
    }
}

impl HasSampleValues for AssetAddress {
    fn sample() -> Self {
        Self::sample_fungible()
    }

    fn sample_other() -> Self {
        Self::sample_non_fungible()
    }
}

#[allow(unused)]
impl AssetAddress {
    pub(crate) fn sample_fungible() -> Self {
        Self::Fungible(ResourceAddress::sample())
    }

    pub(crate) fn sample_non_fungible() -> Self {
        Self::NonFungible(NonFungibleGlobalId::sample_other())
    }

    pub(crate) fn sample_pool_unit() -> Self {
        Self::PoolUnit(PoolAddress::sample())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AssetAddress;

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
    fn json_roundtrip() {
        assert_eq_after_json_roundtrip(
            &SUT::sample_fungible(),
            r#"
            {
                "kind": "fungible",
                "value": "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
            }
            "#,
        );

        assert_eq_after_json_roundtrip(
            &SUT::sample_non_fungible(),
            r#"
            {
                "kind": "nonFungible",
                "value": "resource_rdx1n2ekdd2m0jsxjt9wasmu3p49twy2yfalpaa6wf08md46sk8dfmldnd:<foobar>"
            }
            "#,
        );

        assert_eq_after_json_roundtrip(
            &SUT::sample_pool_unit(),
            r#"
            {
                "kind": "poolUnit",
                "value": "pool_rdx1c5dkfdtdqvczcwzdyvzeuhddyha768p2q28erden533fty8h68ay6m"
            }
            "#,
        );
    }
}
