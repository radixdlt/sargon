use std::cell::RefCell;

use serde::{Deserialize, Serialize};

use super::{
    app_preferences::app_preferences::AppPreferences,
    factors::factor_sources::factor_sources::FactorSources, header::header::Header,
    networks::networks::Networks,
};

/// Representation of the Radix Wallet, contains a list of
/// users Accounts, Personas, Authorized Dapps per network
/// the user has used. It also contains all FactorSources,
/// FactorInstances and wallet App preferences.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    /// The header of a Profile(Snapshot) contains crucial metadata
    /// about this Profile, such as which JSON data format it is
    /// compatible with and which device was used to create it and
    /// a hint about its contents.
    header: RefCell<Header>,

    /// All sources of factors, used for authorization such as spending funds, contains no
    /// secrets.
    factor_sources: RefCell<FactorSources>,

    /// Settings for this profile in the app, contains default security configs
    /// as well as display settings.
    app_preferences: RefCell<AppPreferences>,

    /// An ordered mapping of NetworkID -> `Profile.Network`, containing
    /// all the users Accounts, Personas and AuthorizedDapps the user
    /// has created and interacted with on this network.
    networks: RefCell<Networks>,
}

impl Profile {
    /// Panics if `factor_sources` is empty, since FactorSources MUST not be empty.
    pub fn with(
        header: Header,
        factor_sources: FactorSources,
        app_preferences: AppPreferences,
        networks: Networks,
    ) -> Self {
        factor_sources.assert_not_empty();
        Self {
            header: RefCell::new(header),
            factor_sources: RefCell::new(factor_sources),
            app_preferences: RefCell::new(app_preferences),
            networks: RefCell::new(networks),
        }
    }
}

impl Profile {
    pub fn header(&self) -> Header {
        self.header.borrow().clone()
    }

    pub fn set_header(&self, new: Header) {
        *self.header.borrow_mut() = new
    }

    pub fn factor_sources(&self) -> FactorSources {
        self.factor_sources.borrow().clone()
    }

    /// Panics if `new` is empty, since FactorSources MUST not be empty.
    pub fn set_factor_sources(&self, new: FactorSources) {
        new.assert_not_empty();
        *self.factor_sources.borrow_mut() = new
    }

    pub fn app_preferences(&self) -> AppPreferences {
        self.app_preferences.borrow().clone()
    }

    pub fn set_app_preferences(&self, new: AppPreferences) {
        *self.app_preferences.borrow_mut() = new
    }

    pub fn networks(&self) -> Networks {
        self.networks.borrow().clone()
    }

    pub fn set_networks(&self, new: Networks) {
        *self.networks.borrow_mut() = new
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl Profile {
    pub fn placeholder() -> Self {
        let networks = Networks::placeholder();
        let header = Header::placeholder();
        header.set_content_hint(networks.content_hint());
        Self::with(
            header,
            FactorSources::placeholder(),
            AppPreferences::placeholder(),
            networks,
        )
    }
}

#[cfg(test)]
mod tests {
    use identified_vec::IsIdentifiedVecOf;
    use wallet_kit_common::assert_eq_after_json_roundtrip;

    use crate::v100::{
        app_preferences::app_preferences::AppPreferences,
        factors::factor_sources::factor_sources::FactorSources, header::header::Header,
        networks::networks::Networks,
    };

    use super::Profile;

    #[should_panic(expected = "FactorSources empty, which must never happen.")]
    #[test]
    fn panic_when_factor_sources_empty_in_profile_constructor() {
        Profile::with(
            Header::placeholder(),
            FactorSources::new(),
            AppPreferences::placeholder(),
            Networks::placeholder(),
        );
    }

    #[should_panic(expected = "FactorSources empty, which must never happen.")]
    #[test]
    fn panic_when_factor_sources_empty_when_update_factor_sources() {
        let sut = Profile::placeholder();
        sut.set_factor_sources(FactorSources::new());
    }

    #[test]
    fn set_header() {
        let profile = Profile::placeholder();
        assert_eq!(profile.header(), Header::placeholder());
        profile.set_header(Header::placeholder_other());
        assert_eq!(profile.header(), Header::placeholder_other());
    }

    #[test]
    fn set_factor_sources() {
        let profile = Profile::placeholder();
        assert_eq!(profile.factor_sources(), FactorSources::placeholder());
        profile.set_factor_sources(FactorSources::placeholder_other());
        assert_eq!(profile.factor_sources(), FactorSources::placeholder_other());
    }

    #[test]
    fn set_app_preferences() {
        let profile = Profile::placeholder();
        assert_eq!(profile.app_preferences(), AppPreferences::placeholder());
        profile.set_app_preferences(AppPreferences::placeholder_other());
        assert_eq!(
            profile.app_preferences(),
            AppPreferences::placeholder_other()
        );
    }

    #[test]
    fn set_networks() {
        let profile = Profile::placeholder();
        assert_eq!(profile.networks(), Networks::placeholder());
        profile.set_networks(Networks::placeholder_other());
        assert_eq!(profile.networks(), Networks::placeholder_other());
    }

    #[test]
    fn json_roundtrip() {
        let sut = Profile::placeholder();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {	
				"header":  {
					"snapshotVersion": 100,
					"id": "12345678-bbbb-cccc-dddd-abcd12345678",
					"creatingDevice": {
						"id": "66f07ca2-a9d9-49e5-8152-77aca3d1dd74",
						"date": "2023-09-11T16:05:56.000Z",
						"description": "iPhone"
					},
					"lastUsedOnDevice": {
						"id": "66f07ca2-a9d9-49e5-8152-77aca3d1dd74",
						"date": "2023-09-11T16:05:56.000Z",
						"description": "iPhone"
					},
					"lastModified": "2023-09-11T16:05:56.000Z",
					"contentHint": {
						"numberOfAccountsOnAllNetworksInTotal": 4,
						"numberOfPersonasOnAllNetworksInTotal": 0,
						"numberOfNetworks": 2
					}
				},
				"factorSources": [
					{
						"discriminator": "device",
						"device": {
							"id": {
								"kind": "device",
								"body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
							},
							"common": {
								"flags": ["main"],
								"addedOn": "2023-09-11T16:05:56.000Z",
								"cryptoParameters": {
									"supportedCurves": ["curve25519"],
									"supportedDerivationPathSchemes": ["cap26"]
								},
								"lastUsedOn": "2023-09-11T16:05:56.000Z"
							},
							"hint": {
								"name": "Unknown Name",
								"model": "iPhone",
								"mnemonicWordCount": 24
							}
						}
					},
					{
						"discriminator": "ledgerHQHardwareWallet",
						"ledgerHQHardwareWallet": {
							"id": {
								"kind": "ledgerHQHardwareWallet",
								"body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
							},
							"common": {
								"addedOn": "2023-09-11T16:05:56.000Z",
								"cryptoParameters": {
									"supportedCurves": ["curve25519"],
									"supportedDerivationPathSchemes": ["cap26"]
								},
								"flags": ["main"],
								"lastUsedOn": "2023-09-11T16:05:56.000Z"
							},
							"hint": {
								"name": "Orange, scratched",
								"model": "nanoS+"
							}
						}
					}
				],
				"appPreferences": {
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
				},
				"networks":	[	
					{
						"networkID": 1,
						"accounts": [
							{
								"securityState": {
									"unsecuredEntityControl": {
										"transactionSigning": {
											"badge": {
												"virtualSource": {
													"hierarchicalDeterministicPublicKey": {
														"publicKey": {
															"curve": "curve25519",
															"compressedData": "d24cc6af91c3f103d7f46e5691ce2af9fea7d90cfb89a89d5bba4b513b34be3b"
														},
														"derivationPath": {
															"scheme": "cap26",
															"path": "m/44H/1022H/1H/525H/1460H/0H"
														}
													},
													"discriminator": "hierarchicalDeterministicPublicKey"
												},
												"discriminator": "virtualSource"
											},
											"factorSourceID": {
												"fromHash": {
													"kind": "device",
													"body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
												},
												"discriminator": "fromHash"
											}
										}
									},
									"discriminator": "unsecured"
								},
								"networkID": 1,
								"appearanceID": 0,
								"flags": [],
								"displayName": "Alice",
								"onLedgerSettings": {
									"thirdPartyDeposits": {
										"depositRule": "acceptAll",
										"assetsExceptionList": [],
										"depositorsAllowList": []
									}
								},
								"flags": [],
								"address": "account_rdx12yy8n09a0w907vrjyj4hws2yptrm3rdjv84l9sr24e3w7pk7nuxst8"
							},
							{
								"securityState": {
									"unsecuredEntityControl": {
										"transactionSigning": {
											"badge": {
												"virtualSource": {
													"hierarchicalDeterministicPublicKey": {
														"publicKey": {
															"curve": "curve25519",
															"compressedData": "08740a2fd178c40ce71966a6537f780978f7f00548cfb59196344b5d7d67e9cf"
														},
														"derivationPath": {
															"scheme": "cap26",
															"path": "m/44H/1022H/1H/525H/1460H/1H"
														}
													},
													"discriminator": "hierarchicalDeterministicPublicKey"
												},
												"discriminator": "virtualSource"
											},
											"factorSourceID": {
												"fromHash": {
													"kind": "device",
													"body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
												},
												"discriminator": "fromHash"
											}
										}
									},
									"discriminator": "unsecured"
								},
								"networkID": 1,
								"appearanceID": 1,
								"flags": [],
								"displayName": "Bob",
								"onLedgerSettings": {
									"thirdPartyDeposits": {
										"depositRule": "acceptAll",
										"assetsExceptionList": [],
										"depositorsAllowList": []
									}
								},
								"flags": [],
								"address": "account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69"
							}
						]
					},
					{
						"networkID": 2,
						"accounts": [
							{
								"securityState": {
									"unsecuredEntityControl": {
										"transactionSigning": {
											"badge": {
												"virtualSource": {
													"hierarchicalDeterministicPublicKey": {
														"publicKey": {
															"curve": "curve25519",
															"compressedData": "18c7409458a82281711b668f833b0485e8fb58a3ceb8a728882bf6b83d3f06a9"
														},
														"derivationPath": {
															"scheme": "cap26",
															"path": "m/44H/1022H/2H/525H/1460H/0H"
														}
													},
													"discriminator": "hierarchicalDeterministicPublicKey"
												},
												"discriminator": "virtualSource"
											},
											"factorSourceID": {
												"fromHash": {
													"kind": "device",
													"body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
												},
												"discriminator": "fromHash"
											}
										}
									},
									"discriminator": "unsecured"
								},
								"networkID": 2,
								"appearanceID": 0,
								"flags": [],
								"displayName": "Carol",
								"onLedgerSettings": {
									"thirdPartyDeposits": {
										"depositRule": "acceptAll",
										"assetsExceptionList": [],
										"depositorsAllowList": []
									}
								},
								"flags": [],
								"address": "account_tdx_2_1289zm062j788dwrjefqkfgfeea5tkkdnh8htqhdrzdvjkql4kxceql"
							},
							{
								"securityState": {
									"unsecuredEntityControl": {
										"transactionSigning": {
											"badge": {
												"virtualSource": {
													"hierarchicalDeterministicPublicKey": {
														"publicKey": {
															"curve": "curve25519",
															"compressedData": "26b3fd7f65f01ff8e418a56722fde9cc6fc18dc983e0474e6eb6c1cf3bd44f23"
														},
														"derivationPath": {
															"scheme": "cap26",
															"path": "m/44H/1022H/2H/525H/1460H/1H"
														}
													},
													"discriminator": "hierarchicalDeterministicPublicKey"
												},
												"discriminator": "virtualSource"
											},
											"factorSourceID": {
												"fromHash": {
													"kind": "device",
													"body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
												},
												"discriminator": "fromHash"
											}
										}
									},
									"discriminator": "unsecured"
								},
								"networkID": 2,
								"appearanceID": 1,
								"flags": [],
								"displayName": "Diana",
								"onLedgerSettings": {
									"thirdPartyDeposits": {
										"depositRule": "acceptAll",
										"assetsExceptionList": [],
										"depositorsAllowList": []
									}
								},
								"flags": [],
								"address": "account_tdx_2_129663ef7fj8azge3y6sl73lf9vyqt53ewzlf7ul2l76mg5wyqlqlpr"
							}
						]
					}
				]
			}
            "#,
        );
    }
}
