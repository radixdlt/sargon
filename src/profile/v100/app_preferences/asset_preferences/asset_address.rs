use crate::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Ord,
    PartialOrd,
    derive_more::Display,
    uniffi::Enum,
)]
pub enum AssetAddress {
    Fungible(ResourceAddress),
    NonFungible(NonFungibleGlobalId),
    PoolUnit(PoolAddress),
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

impl<'de> Deserialize<'de> for AssetAddress {
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        #[derive(Deserialize, Serialize)]
        struct Wrapper {
            kind: String,
            value: String,
        }
        let wrapper = Wrapper::deserialize(deserializer)?;
        
        let asset_address = match wrapper.kind.as_str() {
            "fungible" => {
                match wrapper.value.parse::<ResourceAddress>() {
                    Ok(res) => AssetAddress::Fungible(res),
                    Err(e) => Err(serde::de::Error::custom(e))?,
                }
            },
            "nonFungible" => {
                match wrapper.value.parse::<NonFungibleGlobalId>() {
                    Ok(res) => AssetAddress::NonFungible(res),
                    Err(e) => Err(serde::de::Error::custom(e))?,
                }
            },
            "poolUnit" => {
                match wrapper.value.parse::<PoolAddress>() {
                    Ok(res) => AssetAddress::PoolUnit(res),
                    Err(e) => Err(serde::de::Error::custom(e))?,
                }
            },
            _ => unreachable!()
        };
        
        Ok(asset_address)
    }
}

impl Serialize for AssetAddress {
    #[cfg(not(tarpaulin_include))] // false negative
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("AssetAddress", 2)?;
        state.serialize_field("kind", &self.kind())?;
        state.serialize_field("value", &self.value())?;
        state.end()
    }
}

impl AssetAddress {
    fn kind(&self) -> String {
        match self {
            AssetAddress::Fungible(_) => "fungible".to_owned(),
            AssetAddress::NonFungible(_) => "nonFungible".to_owned(),
            AssetAddress::PoolUnit(_) => "poolUnit".to_owned(),
        }
    }

    fn value(&self) -> String {
        match self {
            AssetAddress::Fungible(value) => value.address(),
            AssetAddress::NonFungible(value) => value.to_string(),
            AssetAddress::PoolUnit(value) => value.to_string(),
        }
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
