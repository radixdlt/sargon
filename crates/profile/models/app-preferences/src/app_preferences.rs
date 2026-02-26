use crate::prelude::*;

/// Collection of all settings, preferences and configuration related to how the wallet
/// behaves and looks.
///
/// Current and other saved Gateways, P2P transport profiles, security settings,
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
)]
#[serde(rename_all = "camelCase")]
#[display("{}", self.description())]
pub struct AppPreferences {
    /// Display settings in the wallet app, such as appearances, currency etc.
    pub display: AppDisplay,

    /// The gateway of the active network and collection of other saved gateways.
    pub gateways: SavedGateways,

    /// Current and other saved P2P transport profiles containing signaling server
    /// and full TURN/STUN ICE server configuration.
    #[serde(default)]
    pub p2p_transport_profiles: SavedP2PTransportProfiles,

    /// Current and other saved Radix Connect relay services.
    #[serde(default)]
    pub relay_services: SavedRelayServices,

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
        p2p_transport_profiles: {}
        relay_services: {}
        security: {}
        transaction: {}
        "#,
            self.display,
            self.gateways,
            self.p2p_transport_profiles.current.name,
            self.relay_services.current.name,
            self.security,
            self.transaction
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
            p2p_transport_profiles: SavedP2PTransportProfiles::default(),
            relay_services: SavedRelayServices::default(),
            security,
            transaction,
        }
    }
}

impl HasSampleValues for AppPreferences {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self {
            display: AppDisplay::sample(),
            gateways: SavedGateways::sample(),
            p2p_transport_profiles: SavedP2PTransportProfiles::sample(),
            relay_services: SavedRelayServices::sample(),
            security: Security::sample(),
            transaction: TransactionPreferences::sample(),
        }
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self {
            display: AppDisplay::sample_other(),
            gateways: SavedGateways::sample_other(),
            p2p_transport_profiles: SavedP2PTransportProfiles::sample_other(),
            relay_services: SavedRelayServices::sample_other(),
            security: Security::sample_other(),
            transaction: TransactionPreferences::sample_other(),
        }
    }
}

impl AppPreferences {
    pub fn has_gateway_with_url(&self, url: Url) -> bool {
        self.gateways.all().into_iter().any(|g| g.id() == url)
    }

    pub fn has_p2p_transport_profile_with_signaling_server(
        &self,
        signaling_server: impl AsRef<str>,
    ) -> bool {
        self.p2p_transport_profiles
            .has_signaling_server(signaling_server)
    }

    pub fn has_relay_service_with_url(&self, url: Url) -> bool {
        self.relay_services.has_url(url)
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
    fn get_p2p_transport_profiles() {
        assert_eq!(
            SUT::sample().p2p_transport_profiles,
            SavedP2PTransportProfiles::sample()
        )
    }

    #[test]
    fn get_relay_services() {
        assert_eq!(SUT::sample().relay_services, SavedRelayServices::sample())
    }

    #[test]
    fn get_transaction() {
        assert_eq!(SUT::sample().transaction, TransactionPreferences::sample())
    }

    #[test]
    fn test_has_gateway_with_url() {
        let sut = SUT::sample();
        // Test without the "/" at the end
        let mut url = Url::parse("https://mainnet.radixdlt.com").unwrap();
        assert!(sut.has_gateway_with_url(url));

        // Test with the "/" at the end
        url = Url::parse("https://mainnet.radixdlt.com/").unwrap();
        assert!(sut.has_gateway_with_url(url));

        // Test with a Url that isn't present
        url = Url::parse("https://radixdlt.com/").unwrap();
        assert!(!sut.has_gateway_with_url(url));
    }

    #[test]
    fn test_has_p2p_transport_profile_with_signaling_server() {
        let sut = SUT::sample();

        let existing = "wss://signaling-server.radixdlt.com/";
        assert!(sut.has_p2p_transport_profile_with_signaling_server(existing));

        let missing = "wss://example.com/";
        assert!(!sut.has_p2p_transport_profile_with_signaling_server(missing));
    }

    #[test]
    fn test_has_relay_service_with_url() {
        let sut = SUT::sample();

        let existing =
            Url::parse("https://radix-connect-relay.radixdlt.com/api/v1")
                .unwrap();
        assert!(sut.has_relay_service_with_url(existing));

        let missing = Url::parse("https://example.com/api/v1").unwrap();
        assert!(!sut.has_relay_service_with_url(missing));
    }

    #[test]
    fn json_roundtrip() {
        let sut = SUT::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "display": {
                    "isCurrencyAmountVisible": true,
                    "fiatCurrencyPriceTarget": "usd"
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
                    "isDeveloperModeEnabled": false,
                    "isAdvancedLockEnabled": false,
                    "securityStructuresOfFactorSourceIDs": []
                },
                "p2pTransportProfiles": {
                    "current": {
                        "name": "Sample Production",
                        "signalingServer": "wss://signaling-server.radixdlt.com/",
                        "iceServers": []
                    },
                    "other": [
                        {
                            "name": "Sample Development",
                            "signalingServer": "wss://signaling-server-dev.rdx-works-main.extratools.works/",
                            "iceServers": []
                        }
                    ]
                },
                "relayServices": {
                    "current": {
                        "name": "Sample Relay Production",
                        "url": "https://radix-connect-relay.radixdlt.com/api/v1"
                    },
                    "other": [
                        {
                            "name": "Sample Relay Alternate",
                            "url": "https://relay-alt.example/api/v1"
                        }
                    ]
                },
                "transaction": {
                    "defaultDepositGuarantee": "0.975"
                }
            }               
            "#,
        )
    }

    #[test]
    fn json_deserialize_without_p2p_transport_profiles_uses_default() {
        let json = r#"
        {
            "display": {
                "isCurrencyAmountVisible": true,
                "fiatCurrencyPriceTarget": "usd"
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
                    }
                ]
            },
            "security": {
                "isCloudProfileSyncEnabled": true,
                "isDeveloperModeEnabled": false,
                "isAdvancedLockEnabled": false,
                "securityStructuresOfFactorSourceIDs": []
            },
            "transaction": {
                "defaultDepositGuarantee": "0.975"
            }
        }
        "#;

        let decoded: SUT = serde_json::from_str(json).unwrap();
        assert_eq!(
            decoded.p2p_transport_profiles,
            SavedP2PTransportProfiles::default()
        );
        assert_eq!(decoded.relay_services, SavedRelayServices::default());
    }
}
