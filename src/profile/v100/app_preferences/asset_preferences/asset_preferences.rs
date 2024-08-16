use crate::prelude::*;
use core::hash::Hash;
use std::{hash::Hasher, ops::Index};

decl_identified_vec_of!(
    /// User off-ledger preferences regarding assets.
    AssetPreference
);

impl HasSampleValues for AssetPreferences {
    fn sample() -> Self {
        Self::from_iter([
            AssetPreference::sample(),
            AssetPreference::sample_other(),
        ])
    }

    fn sample_other() -> Self {
        Self::from_iter([AssetPreference::sample_other()])
    }
}

impl AssetPreferences {
    pub fn get_hidden_assets(&self) -> AssetAddresses {
        self.iter()
            .filter(|x| x.visibility == AssetVisibility::Hidden)
            .map(|x| x.asset_address)
            .collect()
    }

    pub fn hide_asset(&mut self, asset: AssetAddress) {
        if !self
            .update_with(asset.id(), |x| x.visibility = AssetVisibility::Hidden)
        {
            let item = AssetPreference::new(asset, AssetVisibility::Hidden);
            self.append(item);
        }
    }

    pub fn unhide_asset(&mut self, asset: AssetAddress) {
        if !self.update_with(asset.id(), |x| {
            x.visibility = AssetVisibility::Visible
        }) {
            let item = AssetPreference::new(asset, AssetVisibility::Visible);
            self.append(item);
        }
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

        // Test unhiding an asset that wasn't present
        let pool_unit = AssetAddress::PoolUnit(PoolAddress::sample());
        sut.unhide_asset(pool_unit.clone());
        result = sut.get_hidden_assets();
        assert!(result.is_empty());

        // Test with some assets hidden
        let fungible_one =
            AssetAddress::Fungible(ResourceAddress::sample_other());
        let fungible_two = AssetAddress::Fungible(ResourceAddress::sample());
        sut.hide_asset(fungible_one.clone());
        sut.hide_asset(fungible_two.clone());

        result = sut.get_hidden_assets();
        assert_eq!(
            AssetAddresses::from_iter([
                fungible_one.clone(),
                fungible_two.clone()
            ]),
            result
        );

        // Test hiding some non-fungible and pool unit, and unhiding one of the fungibles
        let non_fungible =
            AssetAddress::NonFungible(NonFungibleGlobalId::sample());
        sut.unhide_asset(fungible_one);
        sut.hide_asset(non_fungible.clone());
        sut.hide_asset(pool_unit.clone());

        result = sut.get_hidden_assets();
        assert_eq!(
            AssetAddresses::from_iter([fungible_two, non_fungible, pool_unit]),
            result
        );
    }

    #[test]
    fn json_roundtrip() {
        let sut = SUT::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            [
                {
                    "asset_address": {
                        "kind": "fungible",
                        "value": "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
                    },
                    "visibility": "hidden"
                },
                {
                    "asset_address": {
                        "kind": "nonFungible",
                        "value": "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa:<Member_237>"
                    },
                    "visibility": "visible"
                }
            ]
            "#,
        );
    }
}
