use std::cell::RefCell;

use serde::{Deserialize, Serialize};

use super::{
    display::AppDisplay, gateways::gateways::Gateways, p2p_links::p2p_links::P2PLinks,
    security::Security, transaction::Transaction,
};

/// Collection of all settings, preferences and configuration related to how the wallet
/// behaves and looks.
///
/// Current and other saved Gateways, security settings, connected P2P clients,
/// App Display settings and preferences for transaction.
#[derive(Clone, Debug, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppPreferences {
    /// Display settings in the wallet app, such as appearances, currency etc.
    display: RefCell<AppDisplay>,

    /// The gateway of the active network and collection of other saved gateways.
    gateways: RefCell<Gateways>,

    /// Collection of clients user have connected P2P with, typically these
    /// are WebRTC connections with DApps, but might be Android or iPhone
    /// clients as well.
    p2p_links: RefCell<P2PLinks>,

    /// Controls e.g. if Profile Snapshot gets synced to iCloud/Google backup or not.
    security: RefCell<Security>,

    /// Default config related to making of transactions
    transaction: RefCell<Transaction>,
}

impl AppPreferences {
    pub fn display(&self) -> AppDisplay {
        self.display.borrow().clone()
    }

    pub fn gateways(&self) -> Gateways {
        self.gateways.borrow().clone()
    }

    pub fn p2p_links(&self) -> P2PLinks {
        self.p2p_links.borrow().clone()
    }

    pub fn security(&self) -> Security {
        self.security.borrow().clone()
    }

    pub fn transaction(&self) -> Transaction {
        self.transaction.borrow().clone()
    }
}

impl AppPreferences {
    pub fn set_display(&self, new: AppDisplay) {
        *self.display.borrow_mut() = new
    }

    pub fn set_gateways(&self, new: Gateways) {
        *self.gateways.borrow_mut() = new
    }

    pub fn set_p2p_links(&self, new: P2PLinks) {
        *self.p2p_links.borrow_mut() = new
    }

    pub fn set_security(&self, new: Security) {
        *self.security.borrow_mut() = new
    }

    pub fn set_transaction(&self, new: Transaction) {
        *self.transaction.borrow_mut() = new
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
            display: RefCell::new(display),
            gateways: RefCell::new(gateways),
            p2p_links: RefCell::new(p2p_links),
            security: RefCell::new(security),
            transaction: RefCell::new(transaction),
        }
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl AppPreferences {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder() -> Self {
        Self::new(
            AppDisplay::placeholder(),
            Gateways::placeholder(),
            P2PLinks::placeholder(),
            Security::placeholder(),
            Transaction::placeholder(),
        )
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_other() -> Self {
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
    use wallet_kit_common::json::assert_eq_after_json_roundtrip;

    use crate::v100::app_preferences::{
        display::AppDisplay, gateways::gateways::Gateways, p2p_links::p2p_links::P2PLinks,
        security::Security, transaction::Transaction,
    };

    use super::AppPreferences;

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
