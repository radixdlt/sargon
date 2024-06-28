use crate::prelude::*;

/// Collection of all settings, preferences and configuration related to how the wallet
/// behaves and looks.
///
/// Current and other saved Gateways, security settings,
/// App Display settings and preferences for transaction.
#[derive(
    Debug,
    Default,
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    Clone,
    Hash,
    derive_more::Display,
    uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
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

impl AppPreferences {
    pub fn description(&self) -> String {
        format!(
            r#"
        display: {}
        gateways: {}
        security: {}
        transaction: {}
        "#,
            self.display, self.gateways, self.security, self.transaction
        )
    }
}

impl AppPreferences {
    pub fn new(
        display: AppDisplay,
        gateways: SavedGateways,
        security: Security,
        transaction: TransactionPreferences,
    ) -> Self {
        Self {
            display,
            gateways,
            security,
            transaction,
        }
    }
}

impl HasSampleValues for AppPreferences {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::new(
            AppDisplay::sample(),
            SavedGateways::sample(),
            Security::sample(),
            TransactionPreferences::sample(),
        )
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::new(
            AppDisplay::sample_other(),
            SavedGateways::sample_other(),
            Security::sample_other(),
            TransactionPreferences::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AppPreferences;

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
    fn get_display() {
        assert_eq!(SUT::sample().display, AppDisplay::sample())
    }

    #[test]
    fn get_gateways() {
        assert_eq!(SUT::sample().gateways, SavedGateways::sample())
    }

    #[test]
    fn get_security() {
        assert_eq!(SUT::sample().security, Security::sample())
    }

    #[test]
    fn get_transaction() {
        assert_eq!(SUT::sample().transaction, TransactionPreferences::sample())
    }

    #[test]
    fn json_roundtrip() {
        let sut = SUT::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "display": {
                    "fiatCurrencyPriceTarget": "usd",
                    "isCurrencyAmountVisible": true
                },
                "gateways": {
                    "current": "https://mainnet.radixdlt.com/",
                    "saved": [
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
                "security": {
                    "isCloudProfileSyncEnabled": true,
                    "securityStructuresOfFactorSourceIDs": [],
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
