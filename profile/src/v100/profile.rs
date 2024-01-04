use std::fmt::Debug;

use identified_vec::IsIdentifiedVec;
use serde::{Deserialize, Serialize};

use crate::CommonError;

use crate::HasPlaceholder;
use crate::PrivateHierarchicalDeterministicFactorSource;

use super::{
    Account, AccountAddress, AppPreferences, FactorSourceID, FactorSources, Header, IsFactorSource,
    Networks,
};

/// Representation of the Radix Wallet, contains a list of
/// users Accounts, Personas, Authorized Dapps per network
/// the user has used. It also contains all FactorSources,
/// FactorInstances and wallet App preferences.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    /// The header of a Profile(Snapshot) contains crucial metadata
    /// about this Profile, such as which JSON data format it is
    /// compatible with and which device was used to create it and
    /// a hint about its contents.
    pub header: Header,

    /// All sources of factors, used for authorization such as spending funds, contains no
    /// secrets.
    pub factor_sources: FactorSources,

    /// Settings for this profile in the app, contains default security configs
    /// as well as display settings.
    pub app_preferences: AppPreferences,

    /// An ordered mapping of NetworkID -> `Profile.Network`, containing
    /// all the users Accounts, Personas and AuthorizedDapps the user
    /// has created and interacted with on this network.
    pub networks: Networks,
}

#[uniffi::export]
pub fn new_profile_placeholder() -> Profile {
    Profile::placeholder()
}

#[uniffi::export]
pub fn new_profile_placeholder_other() -> Profile {
    Profile::placeholder_other()
}

impl Profile {
    /// Creates a new Profile from the `PrivateHierarchicalDeterministicFactorSource`, without any
    /// networks (thus no accounts), with creating device info as "unknown".
    pub fn new(private_device_factor_source: PrivateHierarchicalDeterministicFactorSource) -> Self {
        let bdfs = private_device_factor_source.factor_source;
        Self::with(
            Header::default(),
            FactorSources::with_bdfs(bdfs),
            AppPreferences::default(),
            Networks::new(),
        )
    }

    /// Panics if `factor_sources` is empty, since FactorSources MUST not be empty.
    pub fn with(
        header: Header,
        factor_sources: FactorSources,
        app_preferences: AppPreferences,
        networks: Networks,
    ) -> Self {
        factor_sources.assert_not_empty();
        Self {
            header,
            factor_sources,
            app_preferences,
            networks,
        }
    }
}

impl Profile {
    /// Returns a clone of the updated account if found, else None.
    pub fn update_account<F>(&mut self, address: &AccountAddress, mutate: F) -> Option<Account>
    where
        F: FnMut(&mut Account) -> (),
    {
        self.networks.update_account(address, mutate)
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
        self.factor_sources.try_update_with(factor_source_id, |f| {
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

impl HasPlaceholder for Profile {
    fn placeholder() -> Self {
        let networks = Networks::placeholder();
        let mut header = Header::placeholder();
        header.content_hint = networks.content_hint();
        Self::with(
            header,
            FactorSources::placeholder(),
            AppPreferences::placeholder(),
            networks,
        )
    }

    fn placeholder_other() -> Self {
        let networks = Networks::placeholder_other();
        let mut header = Header::placeholder_other();
        header.content_hint = networks.content_hint();
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
    use identified_vec::{IsIdentifiedVec, ItemsCloned};

    use crate::{
        assert_eq_after_json_roundtrip, AppPreferences, CommonError, DeviceFactorSource,
        DisplayName, FactorSourceCryptoParameters, FactorSourceID, FactorSources, HasPlaceholder,
        Header, LedgerHardwareWalletFactorSource, NetworkID, Networks, Profile, SLIP10Curve,
    };

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
    fn change_supported_curve_of_factor_source() {
        let mut sut = Profile::placeholder();
        let id: &FactorSourceID = &DeviceFactorSource::placeholder().id.into();

        assert!(sut
            .factor_sources
            .contains_id(&DeviceFactorSource::placeholder().id.into()));

        assert_eq!(
            sut.factor_sources
                .get(id)
                .unwrap()
                .as_device()
                .unwrap()
                .common
                .crypto_parameters
                .supported_curves
                .items(),
            [SLIP10Curve::Curve25519]
        );

        assert_eq!(
            sut.update_factor_source(id, |mut dfs: DeviceFactorSource| {
                dfs.common.crypto_parameters =
                    FactorSourceCryptoParameters::babylon_olympia_compatible();
                Ok(dfs)
            }),
            Ok(true)
        );

        assert_eq!(
            sut.factor_sources
                .get(id)
                .unwrap()
                .as_device()
                .unwrap()
                .common
                .crypto_parameters
                .supported_curves
                .items(),
            [SLIP10Curve::Curve25519, SLIP10Curve::Secp256k1]
        );
    }

    #[test]
    fn add_supported_curve_to_factor_source_failure_cast_wrong_factor_source_kind() {
        let mut sut = Profile::placeholder();
        let id: &FactorSourceID = &DeviceFactorSource::placeholder().id.into();

        assert!(sut
            .factor_sources
            .contains_id(&DeviceFactorSource::placeholder().id.into()));

        assert_eq!(
            sut.factor_sources
                .get(id)
                .unwrap()
                .as_device()
                .unwrap()
                .common
                .crypto_parameters
                .supported_curves
                .items(),
            [SLIP10Curve::Curve25519]
        );

        assert_eq!(
            sut.update_factor_source(id, |mut lfs: LedgerHardwareWalletFactorSource| {
                lfs.common.crypto_parameters =
                    FactorSourceCryptoParameters::babylon_olympia_compatible();
                Ok(lfs)
            }),
            Err(CommonError::CastFactorSourceWrongKind)
        );

        // Remains unchanged
        assert_eq!(
            sut.factor_sources
                .get(id)
                .unwrap()
                .as_device()
                .unwrap()
                .common
                .crypto_parameters
                .supported_curves
                .items(),
            [SLIP10Curve::Curve25519]
        );
    }

    #[test]
    fn update_name_of_accounts() {
        let mut sut = Profile::placeholder();
        let account = sut
            .networks
            .get(&NetworkID::Mainnet)
            .unwrap()
            .accounts
            .get_at_index(0)
            .unwrap()
            .clone();

        assert_eq!(account.display_name.value, "Alice");
        assert!(sut
            .update_account(&account.address, |a| a.display_name =
                DisplayName::new("Satoshi").unwrap())
            .is_some());

        assert_eq!(
            sut.networks
                .get(&NetworkID::Mainnet)
                .unwrap()
                .accounts
                .get_at_index(0)
                .unwrap()
                .display_name
                .value,
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

#[cfg(test)]
mod uniffi_tests {
    use crate::{new_profile_placeholder, new_profile_placeholder_other, HasPlaceholder};

    use super::Profile;

    #[test]
    fn equality_placeholders() {
        assert_eq!(Profile::placeholder(), new_profile_placeholder());
        assert_eq!(
            Profile::placeholder_other(),
            new_profile_placeholder_other()
        );
    }
}
