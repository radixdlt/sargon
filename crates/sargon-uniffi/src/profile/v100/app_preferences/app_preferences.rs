use crate::prelude::*;
use sargon::AppPreferences as InternalAppPreferences;

/// Collection of all settings, preferences and configuration related to how the wallet
/// behaves and looks.
///
/// Current and other saved Gateways, security settings,
/// App Display settings and preferences for transaction.
#[derive(PartialEq, Eq, Clone, Hash, InternalConversionV2, uniffi::Record)]
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
    InternalAppPreferences::sample().into()
}

#[uniffi::export]
pub fn new_app_preferences_sample_other() -> AppPreferences {
    InternalAppPreferences::sample_other().into()
}

#[uniffi::export]
pub fn new_app_preferences_default() -> AppPreferences {
    InternalAppPreferences::default().into()
}

#[uniffi::export]
pub fn app_preferences_has_gateway_with_url(
    app_preferences: AppPreferences,
    url: &FfiUrl,
) -> bool {
    app_preferences
        .into_internal()
        .has_gateway_with_url(url.url.clone())
}
