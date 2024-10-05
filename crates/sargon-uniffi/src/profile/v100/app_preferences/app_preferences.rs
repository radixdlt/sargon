use crate::prelude::*;

/// Collection of all settings, preferences and configuration related to how the wallet
/// behaves and looks.
///
/// Current and other saved Gateways, security settings,
/// App Display settings and preferences for transaction.
#[derive(
    Debug,
    Default,
    PartialEq,
    Eq,
    Clone,
    Hash,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{}", self.description())]
pub struct AppPreferences {
    /// Display settings in the wallet app, such as appearances, currency etc.
    pub display: AppDisplay,

    /// The gateway of the active network and collection of other saved gateways.
    pub gateways: SavedGateways,

    /// Controls e.g. if Profile Snapshot gets synced to iCloud/Google backup or not.
    pub security: Security,

    /// Default config related to making of transactions
    pub transaction: TransactionPreferences,
}

#[uniffi::export]
pub fn new_app_preferences_sample() -> AppPreferences {
    AppPreferences::sample()
}

#[uniffi::export]
pub fn new_app_preferences_sample_other() -> AppPreferences {
    AppPreferences::sample_other()
}

#[uniffi::export]
pub fn new_app_preferences_default() -> AppPreferences {
    AppPreferences::default()
}

#[uniffi::export]
pub fn app_preferences_has_gateway_with_url(
    app_preferences: AppPreferences,
    url: &FfiUrl,
) -> bool {
    app_preferences.has_gateway_with_url(url.url.clone())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AppPreferences;

    #[test]
    fn equality_samples() {
        assert_eq!(SUT::sample(), new_app_preferences_sample());
        assert_eq!(SUT::sample_other(), new_app_preferences_sample_other());
    }

    #[test]
    fn test_default() {
        assert_eq!(new_app_preferences_default(), AppPreferences::default());
    }

    #[test]
    fn test_app_preferences_has_gateway_with_url() {
        assert!(app_preferences_has_gateway_with_url(
            SUT::sample(),
            &FfiUrl::from_str("https://mainnet.radixdlt.com").unwrap()
        ));
    }
}
