use crate::prelude::*;

#[uniffi::export]
pub fn asset_preferences_get_hidden_assets(
    asset_preferences: AssetPreferences,
) -> AssetAddresses {
    asset_preferences.get_hidden_assets()
}

#[uniffi::export]
pub fn asset_preferences_hide_asset(
    asset_preferences: AssetPreferences,
    asset: AssetAddress,
) -> AssetPreferences {
    let mut asset_preferences = asset_preferences.clone();
    asset_preferences.hide_asset(asset);
    asset_preferences
}

#[uniffi::export]
pub fn asset_preferences_unhide_asset(
    asset_preferences: AssetPreferences,
    asset: AssetAddress,
) -> AssetPreferences {
    let mut asset_preferences = asset_preferences.clone();
    asset_preferences.unhide_asset(asset);
    asset_preferences
}

#[cfg(test)]
mod tests {

    use std::clone;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AssetPreferences;

    #[test]
    fn equality_samples() {
        assert_eq!(SUT::sample(), new_asset_preferences_sample());
        assert_eq!(SUT::sample_other(), new_asset_preferences_sample_other());
    }

    #[test]
    fn hidden_assets() {
        // Test with empty AssetPreferences
        let mut sut = SUT::new();
        let mut result = asset_preferences_get_hidden_assets(sut.clone());
        assert!(result.is_empty());

        // Test after hiding one asset
        let asset_one = AssetAddress::sample();
        sut = asset_preferences_hide_asset(sut, asset_one.clone());

        result = asset_preferences_get_hidden_assets(sut.clone());
        assert_eq!(AssetAddresses::from_iter([asset_one.clone()]), result);

        // Test after unhiding the fungible asset
        sut = asset_preferences_unhide_asset(sut, asset_one.clone());
        result = asset_preferences_get_hidden_assets(sut);
        assert!(result.is_empty());
    }
}
