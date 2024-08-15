// use crate::prelude::*;

// #[uniffi::export]
// pub fn new_asset_preferences_sample() -> AssetPreferences {
//     AssetPreferences::sample()
// }

// #[uniffi::export]
// pub fn new_asset_preferences_sample_other() -> AssetPreferences {
//     AssetPreferences::sample_other()
// }

// #[uniffi::export]
// pub fn asset_preferences_get_hidden_resources(
//     asset_preferences: AssetPreferences,
// ) -> HiddenResources {
//     asset_preferences.get_hidden_resources()
// }

// #[uniffi::export]
// pub fn asset_preferences_hide_resource(
//     asset_preferences: &AssetPreferences,
//     kind: AssetAddress,
// ) -> AssetPreferences {
//     let mut asset_preferences = asset_preferences.clone();
//     asset_preferences.hide_resource(kind);
//     asset_preferences
// }

// #[uniffi::export]
// pub fn asset_preferences_unhide_resource(
//     asset_preferences: &AssetPreferences,
//     kind: AssetAddress,
// ) -> AssetPreferences {
//     let mut asset_preferences = asset_preferences.clone();
//     asset_preferences.unhide_resource(kind);
//     asset_preferences
// }

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[allow(clippy::upper_case_acronyms)]
//     type SUT = AssetPreferences;

//     #[test]
//     fn equality_samples() {
//         assert_eq!(SUT::sample(), new_asset_preferences_sample());
//         assert_eq!(
//             SUT::sample_other(),
//             new_asset_preferences_sample_other()
//         );
//     }

//     #[test]
//     fn hidden_resources() {
//         // Test with empty AssetPreferences
//         let sut = SUT::new();
//         let mut result = asset_preferences_get_hidden_resources(sut.clone());
//         assert!(result.fungible.is_empty());

//         // Test after hiding a fungible resource
//         let fungible = ResourceAddress::sample();
//         let after_hiding = asset_preferences_hide_resource(
//             &sut,
//             AssetAddress::Fungible(fungible),
//         );

//         result =
//             asset_preferences_get_hidden_resources(after_hiding.clone());
//         assert_eq!(result.fungible, vec![fungible]);

//         // Test after unhiding the fungible resource
//         let after_unhiding = asset_preferences_unhide_resource(
//             &after_hiding,
//             AssetAddress::Fungible(fungible),
//         );
//         result = asset_preferences_get_hidden_resources(after_unhiding);
//         assert!(result.fungible.is_empty());
//     }
// }
