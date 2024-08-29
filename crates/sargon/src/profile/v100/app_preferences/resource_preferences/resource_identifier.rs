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
        Self::sample_fungible()
    }

    fn sample_other() -> Self {
        Self::sample_non_fungible()
    }
}

#[allow(unused)]
impl ResourceIdentifier {
    pub(crate) fn sample_fungible() -> Self {
        Self::Fungible(ResourceAddress::sample())
    }

    pub(crate) fn sample_non_fungible() -> Self {
        Self::NonFungible(ResourceAddress::sample_other())
    }

    pub(crate) fn sample_pool_unit() -> Self {
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
        assert_eq!(SUT::sample_pool_unit(), PoolAddress::sample().into())
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
                "value": "resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j"
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
