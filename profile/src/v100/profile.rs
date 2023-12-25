use std::{cell::RefCell, fmt::Debug};

use identified_vec::IsIdentifiedVec;
use serde::{Deserialize, Serialize};

use crate::CommonError;
#[cfg(any(test, feature = "placeholder"))]
use crate::HasPlaceholder;

use super::{
    Account, AccountAddress, AppPreferences, FactorSourceID, FactorSources, Header, IsFactorSource,
    Networks,
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

impl Profile {
    /// Returns `false` if no account with `address` was found, otherwise if found,
    /// the account gets updated by `mutate` closure and this function returns
    /// `true`.
    pub fn update_account<F>(&mut self, address: &AccountAddress, mutate: F) -> bool
    where
        F: FnMut(&Account) -> (),
    {
        self.networks.borrow_mut().update_account(address, mutate)
    }

    pub fn update_factor_source<S, M>(
        &mut self,
        factor_source_id: &FactorSourceID,
        mut mutate: M,
    ) -> Result<bool, CommonError>
    where
        S: IsFactorSource,
        M: FnMut(S) -> Result<S, CommonError>,
    {
        self.factor_sources
            .borrow_mut()
            .try_update_with(factor_source_id, |f| {
                S::try_from(f.clone())
                    .map_err(|_| CommonError::CastFactorSourceWrongKind)
                    .and_then(|element| {
                        mutate(element)
                            .map(|modified| modified.into())
                            .map_err(|_| CommonError::UpdateFactorSourceMutateFailed)
                    })
            })
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl HasPlaceholder for Profile {
    fn placeholder() -> Self {
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

    fn placeholder_other() -> Self {
        let networks = Networks::placeholder_other();
        let header = Header::placeholder_other();
        header.set_content_hint(networks.content_hint());
        Self::with(
            header,
            FactorSources::placeholder_other(),
            AppPreferences::placeholder_other(),
            networks,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_eq_after_json_roundtrip, CommonError, HasPlaceholder, SLIP10Curve};
    use identified_vec::{IsIdentifiedVec, IsIdentifiedVecOf};

    use crate::{
        v100::{DeviceFactorSource, DisplayName, FactorSourceID, LedgerHardwareWalletFactorSource},
        NetworkID,
    };

    use super::{AppPreferences, FactorSources, Header, Networks, Profile};

    #[test]
    fn inequality() {
        assert_ne!(Profile::placeholder(), Profile::placeholder_other());
    }

    #[test]
    fn equality() {
        assert_eq!(Profile::placeholder(), Profile::placeholder());
        assert_eq!(Profile::placeholder_other(), Profile::placeholder_other());
    }

    #[test]
    fn add_supported_curve_to_factor_source() {
        let mut sut = Profile::placeholder();
        let id: &FactorSourceID = &DeviceFactorSource::placeholder().id().into();

        assert!(sut
            .factor_sources()
            .contains_id(&DeviceFactorSource::placeholder().id().into()));

        assert_eq!(
            sut.factor_sources()
                .get(id)
                .unwrap()
                .as_device()
                .unwrap()
                .common()
                .crypto_parameters()
                .supported_curves(),
            [SLIP10Curve::Curve25519]
        );

        assert_eq!(
            sut.update_factor_source(id, |dfs: DeviceFactorSource| {
                let common = dfs.common();
                let cp = common.crypto_parameters();
                cp.add_supported_curve(SLIP10Curve::Secp256k1);
                common.set_crypto_parameters(cp);
                dfs.set_common(common);
                Ok(dfs)
            }),
            Ok(true)
        );

        assert_eq!(
            sut.factor_sources()
                .get(id)
                .unwrap()
                .as_device()
                .unwrap()
                .common()
                .crypto_parameters()
                .supported_curves(),
            [SLIP10Curve::Curve25519, SLIP10Curve::Secp256k1]
        );
    }

    #[test]
    fn add_supported_curve_to_factor_source_failure_cast_wrong_factor_source_kind() {
        let mut sut = Profile::placeholder();
        let id: &FactorSourceID = &DeviceFactorSource::placeholder().id().into();

        assert!(sut
            .factor_sources()
            .contains_id(&DeviceFactorSource::placeholder().id().into()));

        assert_eq!(
            sut.factor_sources()
                .get(id)
                .unwrap()
                .as_device()
                .unwrap()
                .common()
                .crypto_parameters()
                .supported_curves(),
            [SLIP10Curve::Curve25519]
        );

        assert_eq!(
            sut.update_factor_source(id, |lfs: LedgerHardwareWalletFactorSource| {
                let common = lfs.common();
                let cp = common.crypto_parameters();
                cp.add_supported_curve(SLIP10Curve::Secp256k1);
                common.set_crypto_parameters(cp);
                lfs.set_common(common);
                Ok(lfs)
            }),
            Err(CommonError::CastFactorSourceWrongKind)
        );

        // Remains unchanged
        assert_eq!(
            sut.factor_sources()
                .get(id)
                .unwrap()
                .as_device()
                .unwrap()
                .common()
                .crypto_parameters()
                .supported_curves(),
            [SLIP10Curve::Curve25519]
        );
    }

    #[test]
    fn update_name_of_accounts() {
        let mut sut = Profile::placeholder();
        let account =
            sut.networks()
                .get(&NetworkID::Mainnet)
                .unwrap()
                .accounts()
                .get_at_index(0)
                .unwrap()
                .clone();

        assert_eq!(account.display_name(), "Alice");
        assert!(sut.update_account(&account.address(), |a| a
            .set_display_name(DisplayName::new("Satoshi").unwrap())));

        assert_eq!(
            sut.networks()
                .get(&NetworkID::Mainnet)
                .unwrap()
                .accounts()
                .get_at_index(0)
                .unwrap()
                .display_name(),
            "Satoshi"
        );
    }

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
