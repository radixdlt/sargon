use serde::{Deserialize, Serialize};

use super::{AppDisplay, Gateways, P2PLinks, Security, Transaction};

#[cfg(any(test, feature = "placeholder"))]
use crate::HasPlaceholder;

/// Collection of all settings, preferences and configuration related to how the wallet
/// behaves and looks.
///
/// Current and other saved Gateways, security settings, connected P2P clients,
/// App Display settings and preferences for transaction.
#[derive(Debug, Default, Deserialize, Serialize, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct AppPreferences {
    /// Display settings in the wallet app, such as appearances, currency etc.
    display: AppDisplay,

    /// The gateway of the active network and collection of other saved gateways.
    gateways: Gateways,

    /// Collection of clients user have connected P2P with, typically these
    /// are WebRTC connections with DApps, but might be Android or iPhone
    /// clients as well.
    // p2p_links: P2PLinks, // FIXME: NOW!

    /// Controls e.g. if Profile Snapshot gets synced to iCloud/Google backup or not.
    security: Security,

    /// Default config related to making of transactions
    transaction: Transaction,
}

impl Eq for AppPreferences {}
impl PartialEq for AppPreferences {
    fn eq(&self, other: &Self) -> bool {
        self.display() == other.display()
            && self.gateways() == other.gateways()
            // && self.p2p_links() == other.p2p_links()
            && self.security() == other.security()
            && self.transaction() == other.transaction()
    }
}

impl Clone for AppPreferences {
    fn clone(&self) -> Self {
        Self::new(
            self.display(),
            self.gateways(),
            self.p2p_links(),
            self.security(),
            self.transaction(),
        )
    }
}

impl AppPreferences {
    pub fn display(&self) -> AppDisplay {
        self.display
            .lock()
            .expect("`self.display` to not have been locked.")
            .clone()
    }

    pub fn gateways(&self) -> Gateways {
        self.gateways
            .lock()
            .expect("`self.gateways` to not have been locked.")
            .clone()
    }

    pub fn p2p_links(&self) -> P2PLinks {
        self.p2p_links
            .lock()
            .expect("`self.p2p_links` to not have been locked.")
            .clone()
    }

    pub fn security(&self) -> Security {
        self.security
            .lock()
            .expect("`self.security` to not have been locked.")
            .clone()
    }

    pub fn transaction(&self) -> Transaction {
        self.transaction
            .lock()
            .expect("`self.transaction` to not have been locked.")
            .clone()
    }
}

impl AppPreferences {
    pub fn set_display(&self, new: AppDisplay) {
        *self
            .display
            .lock()
            .expect("`self.display` to not have been locked.") = new
    }

    pub fn set_gateways(&self, new: Gateways) {
        *self
            .gateways
            .lock()
            .expect("`self.gateways` to not have been locked.") = new
    }

    pub fn set_p2p_links(&self, new: P2PLinks) {
        *self
            .p2p_links
            .lock()
            .expect("`self.p2p_links` to not have been locked.") = new
    }

    pub fn set_security(&self, new: Security) {
        *self
            .security
            .lock()
            .expect("`self.security` to not have been locked.") = new
    }

    pub fn set_transaction(&self, new: Transaction) {
        *self
            .transaction
            .lock()
            .expect("`self.transaction` to not have been locked.") = new
    }
}

impl AppPreferences {
    pub fn new(
        display: AppDisplay,
        gateways: Gateways,
        p2p_links: P2PLinks,
        security: Security,
        transaction: Transaction,
    ) -> Self {
        Self {
            display,
            gateways,
            // p2p_links,
            security,
            transaction,
        }
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl HasPlaceholder for AppPreferences {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::new(
            AppDisplay::placeholder(),
            Gateways::placeholder(),
            P2PLinks::placeholder(),
            Security::placeholder(),
            Transaction::placeholder(),
        )
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
        Self::new(
            AppDisplay::placeholder_other(),
            Gateways::placeholder_other(),
            P2PLinks::placeholder(),
            Security::placeholder_other(),
            Transaction::placeholder_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_eq_after_json_roundtrip, HasPlaceholder};

    use super::{AppDisplay, AppPreferences, Gateways, P2PLinks, Security, Transaction};

    #[test]
    fn equality() {
        assert_eq!(AppPreferences::placeholder(), AppPreferences::placeholder());
        assert_eq!(
            AppPreferences::placeholder_other(),
            AppPreferences::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            AppPreferences::placeholder(),
            AppPreferences::placeholder_other()
        );
    }

    #[test]
    fn get_display() {
        assert_eq!(
            AppPreferences::placeholder().display(),
            AppDisplay::placeholder()
        )
    }

    #[test]
    fn set_display() {
        let sut = AppPreferences::placeholder();
        sut.set_display(AppDisplay::placeholder_other());
        assert_eq!(sut.display(), AppDisplay::placeholder_other())
    }

    #[test]
    fn get_gateways() {
        assert_eq!(
            AppPreferences::placeholder().gateways(),
            Gateways::placeholder()
        )
    }

    #[test]
    fn set_gateways() {
        let sut = AppPreferences::placeholder();
        sut.set_gateways(Gateways::placeholder_other());
        assert_eq!(sut.gateways(), Gateways::placeholder_other())
    }

    #[test]
    fn get_p2p_links() {
        assert_eq!(
            AppPreferences::placeholder().p2p_links(),
            P2PLinks::placeholder()
        )
    }

    #[test]
    fn set_p2p_links() {
        let sut = AppPreferences::placeholder();
        sut.set_p2p_links(P2PLinks::placeholder_other());
        assert_eq!(sut.p2p_links(), P2PLinks::placeholder_other())
    }

    #[test]
    fn get_security() {
        assert_eq!(
            AppPreferences::placeholder().security(),
            Security::placeholder()
        )
    }

    #[test]
    fn set_security() {
        let sut = AppPreferences::placeholder();
        sut.set_security(Security::placeholder_other());
        assert_eq!(sut.security(), Security::placeholder_other())
    }

    #[test]
    fn get_transaction() {
        assert_eq!(
            AppPreferences::placeholder().transaction(),
            Transaction::placeholder()
        )
    }

    #[test]
    fn set_transaction() {
        let sut = AppPreferences::placeholder();
        sut.set_transaction(Transaction::placeholder_other());
        assert_eq!(sut.transaction(), Transaction::placeholder_other())
    }

    #[test]
    fn json_roundtrip() {
        let sut = AppPreferences::placeholder();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "display": {
                    "fiatCurrencyPriceTarget": "usd",
                    "isCurrencyAmountVisible": true
                },
                "gateways": {
                    "current": "https://rcnet-v3.radixdlt.com/",
                    "saved": [
                        {
                            "network": {
                                "name": "zabanet",
                                "id": 14,
                                "displayDescription": "RCnet-V3 (Test Network)"
                            },
                            "url": "https://rcnet-v3.radixdlt.com/"
                        },
                        {
                            "network": {
                                "name": "mainnet",
                                "id": 1,
                                "displayDescription": "Mainnet"
                            },
                            "url": "https://mainnet.radixdlt.com/"
                        },
                        {
                            "network": {
                                "name": "stokenet",
                                "id": 2,
                                "displayDescription": "Stokenet"
                            },
                            "url": "https://babylon-stokenet-gateway.radixdlt.com/"
                        }
                    ]
                },
                "p2pLinks": [
                    {
                        "connectionPassword": "babebabebabebabebabebabebabebabebabebabebabebabebabebabebabebabe",
                        "displayName": "Brave on PC"
                    },
                    {
                        "connectionPassword": "cafecafecafecafecafecafecafecafecafecafecafecafecafecafecafecafe",
                        "displayName": "Chrome on Macbook"
                    }
                ],
                "security": {
                    "isCloudProfileSyncEnabled": true,
                    "structureConfigurationReferences": [],
                    "isDeveloperModeEnabled": true
                },
                "transaction": {
                    "defaultDepositGuarantee": "0.975"
                }
            }
            "#,
        )
    }
}
