use crate::prelude::*;
use core::hash::Hash;
use std::hash::Hasher;

/// User off-ledger preferences regarding assets.
#[derive(
    Debug,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Clone,
    derive_more::Display,
    uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
#[display("assets: {:#?}", self.assets)]
pub struct AssetPreferences {
    #[serde(default)]
    pub assets: HashMap<AssetAddress, AssetPreference>,
}

impl Hash for AssetPreferences {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut pairs: Vec<_> = self.assets.iter().collect();
        pairs.sort_by_key(|i| i.0);

        Hash::hash(&pairs, state);
    }
}

impl AssetPreferences {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
        }
    }
}

impl HasSampleValues for AssetPreferences {
    fn sample() -> Self {
        Self {
            assets: [(AssetAddress::sample(), AssetPreference::sample())]
                .into(),
        }
    }

    fn sample_other() -> Self {
        Self {
            assets: [(
                AssetAddress::sample_other(),
                AssetPreference::sample_other(),
            )]
            .into(),
        }
    }
}

impl AssetPreferences {
    pub fn get_hidden_assets(&self) -> Vec<AssetAddress> {
        self.assets
            .iter()
            .filter(|(_, preference)| {
                preference.visibility == AssetVisibility::Hidden
            })
            .map(|(address, _)| address.clone())
            .sorted()
            .collect()
    }

    pub fn hide_asset(&mut self, asset: AssetAddress) {
        self.assets
            .entry(asset)
            .or_default()
            .set_visibility(AssetVisibility::Hidden);
    }

    pub fn unhide_asset(&mut self, asset: AssetAddress) {
        self.assets
            .entry(asset)
            .or_default()
            .set_visibility(AssetVisibility::Visible);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AssetPreferences;

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
    fn hidden_resources() {
        use crate::AssetAddress::*;
        let mut sut = SUT::new();

        // Test with no assets hidden
        let mut result = sut.get_hidden_assets();
        assert!(result.is_empty());
        
        // Test with some assets hidden
        let fungible_one = AssetAddress::Fungible(ResourceAddress::sample_other());
        let fungible_two = AssetAddress::Fungible(ResourceAddress::sample());
        sut.hide_asset(fungible_one.clone());
        sut.hide_asset(fungible_two.clone());

        result = sut.get_hidden_assets();
        assert_eq!(vec![fungible_one.clone(), fungible_two.clone()], result);

        // Test hiding some non-fungible and pool unit, and unhiding one of the fungibles
        let non_fungible = AssetAddress::NonFungible(NonFungibleGlobalId::sample());
        let pool_unit = AssetAddress::PoolUnit(PoolAddress::sample());
        sut.unhide_asset(fungible_one);
        sut.hide_asset(non_fungible.clone());
        sut.hide_asset(pool_unit.clone());

        result = sut.get_hidden_assets();
        assert_eq!(vec![fungible_two, non_fungible, pool_unit], result);
    }

    #[test]
    fn json_roundtrip() {
        let sut = SUT::sample();
        let str = serde_json::to_string_pretty(&sut).unwrap_err();
        println!("{}", str);
        // assert_eq_after_json_roundtrip(
        //     &sut,
        //     r#"
        //     {
        //         "fungible": {
        //             "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd": [
        //                 "deletedByUser"
        //             ]
        //         },
        //         "nonFungible": {
        //             "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa:<Member_237>": [
        //                 "deletedByUser"
        //             ]
        //         },
        //         "poolUnit": {
        //             "pool_rdx1c5dkfdtdqvczcwzdyvzeuhddyha768p2q28erden533fty8h68ay6m": [
        //                 "deletedByUser"
        //             ]
        //         }
        //     }
        //     "#,
        // )
    }
}
