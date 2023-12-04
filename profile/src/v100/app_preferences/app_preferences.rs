use super::{
    display::AppDisplay, gateways::Gateways, p2p_links::p2p_links::P2PLinks, security::Security,
    transaction::Transaction,
};

/// Collection of all settings, preferences and configuration related to how the wallet
/// behaves and looks.
///
/// Current and other saved Gateways, security settings, connected P2P clients,
/// App Display settings and preferences for transaction.
pub struct AppPreferences {
    /// Default config related to making of transactions
    pub transaction: Transaction,

    /// Controls e.g. if Profile Snapshot gets synced to iCloud/Google backup or not.
    pub security: Security,

    /// Display settings in the wallet app, such as appearances, currency etc.
    pub display: AppDisplay,

    /// Collection of clients user have connected P2P with, typically these
    /// are WebRTC connections with DApps, but might be Android or iPhone
    /// clients as well.
    pub p2p_links: P2PLinks,

    /// The gateway of the active network and collection of other saved gateways.
    pub gateways: Gateways,
}
