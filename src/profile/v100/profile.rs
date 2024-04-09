use crate::prelude::*;

/// Representation of the Radix Wallet, contains a list of
/// users Accounts, Personas, Authorized Dapps per network
/// the user has used. It also contains all FactorSources,
/// FactorInstances and wallet App preferences.
///
/// ```
/// extern crate sargon;
/// use sargon::prelude::*;
///
/// assert_eq!(Profile::sample(), Profile::sample())
/// ```
#[derive(
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
#[display("{}", self.description())]
#[debug("{}", self.pretty_json())]
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
    pub networks: ProfileNetworks,
}

impl Profile {
    pub fn description(&self) -> String {
        format!(
            r#"
			header: {}
			factor_sources: {}
			networks: {}
			app_pref: {}
			"#,
            self.header,
            self.factor_sources,
            self.networks,
            self.app_preferences,
        )
    }

    pub fn pretty_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("should never fail")
    }
}

impl Profile {
    /// Creates a new Profile from the `PrivateHierarchicalDeterministicFactorSource`, without any
    /// networks (thus no accounts), with creating device info as "unknown".
    pub fn new(
        private_device_factor_source: PrivateHierarchicalDeterministicFactorSource,
        creating_device_name: &str,
    ) -> Self {
        let bdfs = private_device_factor_source.factor_source;
        let creating_device = DeviceInfo::with_description(
            format!("{} - {}", creating_device_name, bdfs.hint.model).as_str(),
        );
        let header = Header::new(creating_device);
        Self::with(
            header,
            FactorSources::with_bdfs(bdfs),
            AppPreferences::default(),
            ProfileNetworks::new(),
        )
    }

    /// Panics if `factor_sources` is empty, since FactorSources MUST not be empty.
    pub fn with(
        header: Header,
        factor_sources: FactorSources,
        app_preferences: AppPreferences,
        networks: ProfileNetworks,
    ) -> Self {
        factor_sources.assert_not_empty();
        Self {
            header,
            factor_sources,
            app_preferences,
            networks,
        }
    }

    /// Creates a new `Profile` from json in the form of `BagOfBytes`.
    /// This is a temporarily exported method that allows wallet clients to
    /// integrate Profile in steps.
    ///
    /// Should be replaced later with `WalletClientStorage`
    pub fn new_from_json_bytes(json: BagOfBytes) -> Result<Self> {
        let bytes = json.bytes();
        serde_json::from_slice::<Self>(json.bytes()).map_err(|_| {
            CommonError::FailedToDeserializeJSONToValue {
                json_byte_count: bytes.len() as u64,
                type_name: std::any::type_name::<Profile>().to_string(),
            }
        })
    }
}

impl Profile {
    /// Returns the unique ID of this Profile (just an alias for `header.id`).
    pub fn id(&self) -> ProfileID {
        self.header.id
    }

    /// Returns a clone of the updated account if found, else None.
    pub fn update_account<F>(
        &mut self,
        address: &AccountAddress,
        mutate: F,
    ) -> Option<Account>
    where
        F: FnMut(&mut Account),
    {
        self.networks.update_account(address, mutate)
    }

    pub fn update_factor_source<S, M>(
        &mut self,
        factor_source_id: &FactorSourceID,
        mut mutate: M,
    ) -> Result<bool>
    where
        S: IsFactorSource,
        M: FnMut(S) -> Result<S>,
    {
        self.factor_sources.try_update_with(factor_source_id, |f| {
            S::try_from(f.clone())
                .map_err(|_| CommonError::CastFactorSourceWrongKind {
                    expected: S::kind(),
                    found: f.factor_source_kind(),
                })
                .and_then(|element| {
                    mutate(element).map(|modified| modified.into())
                })
        })
    }

    /// Converts this `Profile` to json in the form of `BagOfBytes`
    /// This is a temporarily exported method that allows wallet clients to
    /// integrate Profile in steps.
    ///
    /// Should be replaced later with `WalletClientStorage`
    pub fn to_json_bytes(&self) -> Result<BagOfBytes> {
        serde_json::to_vec(self)
            .map_err(|_| CommonError::FailedToSerializeToJSON)
            .map(BagOfBytes::from)
    }
}

impl HasSampleValues for Profile {
    fn sample() -> Self {
        let networks = ProfileNetworks::sample();
        let mut header = Header::sample();
        header.content_hint = networks.content_hint();
        Self::with(
            header,
            FactorSources::sample(),
            AppPreferences::sample(),
            networks,
        )
    }

    fn sample_other() -> Self {
        let networks = ProfileNetworks::sample_other();
        let mut header = Header::sample_other();
        header.content_hint = networks.content_hint();
        Self::with(
            header,
            FactorSources::sample_other(),
            AppPreferences::sample_other(),
            networks,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Profile;

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn equality_display() {
        // This test might seem trivial, in fact it is not,
        // Profile is such a big data type that it is easy
        // to accidentally print internals (debug) for display
        // if not done right.
        pretty_assertions::assert_eq!(
            SUT::sample().to_string(),
            SUT::sample().to_string(),
        );
    }

    #[test]
    fn equality_debug() {
        // This test might seem trivial, in fact it is not,
        // Profile is such a big data type that it is easy
        // to accidentally print internals (debug) for display
        // if not done right.
        pretty_assertions::assert_eq!(
            format!("{:?}", SUT::sample()),
            format!("{:?}", SUT::sample())
        );
    }

    #[test]
    fn update_factor_source_not_update_when_factor_source_not_found() {
        let mut sut = SUT::sample();
        let wrong_id: &FactorSourceID =
            &LedgerHardwareWalletFactorSource::sample_other().id.into();

        assert_eq!(
            sut.update_factor_source(
                wrong_id,
                |lfs: LedgerHardwareWalletFactorSource| { Ok(lfs) }
            ),
            Ok(false)
        );
    }

    #[test]
    fn change_supported_curve_of_factor_source() {
        let mut sut = SUT::sample();
        let id: &FactorSourceID = &DeviceFactorSource::sample().id.into();
        assert!(sut
            .factor_sources
            .contains_id(&DeviceFactorSource::sample().id.into()));

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

        // test failure
        assert_eq!(
            sut.update_factor_source(id, |_: DeviceFactorSource| {
                Err(CommonError::UpdateFactorSourceMutateFailed)
            }),
            Err(CommonError::UpdateFactorSourceMutateFailed)
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
    fn add_supported_curve_to_factor_source_failure_cast_wrong_factor_source_kind(
    ) {
        let mut sut = SUT::sample();
        let id: &FactorSourceID = &DeviceFactorSource::sample().id.into();

        assert!(sut
            .factor_sources
            .contains_id(&DeviceFactorSource::sample().id.into()));

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
            sut.update_factor_source(
                id,
                |mut lfs: LedgerHardwareWalletFactorSource| {
                    lfs.common.crypto_parameters =
                    FactorSourceCryptoParameters::babylon_olympia_compatible();
                    Ok(lfs)
                }
            ),
            Err(CommonError::CastFactorSourceWrongKind {
                expected: FactorSourceKind::LedgerHQHardwareWallet,
                found: FactorSourceKind::Device
            })
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
        let mut sut = SUT::sample();
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
        SUT::with(
            Header::sample(),
            FactorSources::new(),
            AppPreferences::sample(),
            ProfileNetworks::sample(),
        );
    }

    #[test]
    fn hash() {
        let n = 100;
        let set = (0..n)
            .map(|_| {
                SUT::new(
                    PrivateHierarchicalDeterministicFactorSource::generate_new(
                        WalletClientModel::Unknown,
                    ),
                    "Foo",
                )
            })
            .collect::<HashSet<_>>();
        assert_eq!(set.len(), n);
    }

    #[test]
    fn to_json_bytes_new_from_json_bytes() {
        let sut = SUT::sample();

        let encoded = sut.to_json_bytes().unwrap();
        let profile_result = SUT::new_from_json_bytes(encoded).unwrap();
        assert_eq!(profile_result, sut);

        let malformed_profile_snapshot = BagOfBytes::from("{}".as_bytes());
        assert_eq!(
            SUT::new_from_json_bytes(malformed_profile_snapshot.clone()),
            Result::Err(CommonError::FailedToDeserializeJSONToValue {
                json_byte_count: malformed_profile_snapshot.len() as u64,
                type_name: std::any::type_name::<Profile>().to_string()
            })
        );
    }

    #[test]
    fn json_roundtrip() {
        let sut = SUT::sample();
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
				"networks": [
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
								"flags": ["deletedByUser"],
								"displayName": "Bob",
								"onLedgerSettings": {
									"thirdPartyDeposits": {
										"depositRule": "acceptAll",
										"assetsExceptionList": [],
										"depositorsAllowList": []
									}
								},
								"address": "account_rdx129a9wuey40lducsf6yu232zmzk5kscpvnl6fv472r0ja39f3hced69"
							}
						],
						"personas": [
							{
								"networkID": 1,
								"address": "identity_rdx122kttqch0eehzj6f9nkkxcw7msfeg9udurq5u0ysa0e92c59w0mg6x",
								"displayName": "Satoshi",
								"securityState": {
									"discriminator": "unsecured",
									"unsecuredEntityControl": {
										"transactionSigning": {
											"factorSourceID": {
												"discriminator": "fromHash",
												"fromHash": {
													"kind": "device",
													"body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
												}
											},
											"badge": {
												"discriminator": "virtualSource",
												"virtualSource": {
													"discriminator": "hierarchicalDeterministicPublicKey",
													"hierarchicalDeterministicPublicKey": {
														"publicKey": {
															"curve": "curve25519",
															"compressedData": "983ab1d3a77dd6b30bb8a5d59d490a0380cc0aa9ab464983d3fc581fcf64543f"
														},
														"derivationPath": {
															"scheme": "cap26",
															"path": "m/44H/1022H/1H/618H/1460H/0H"
														}
													}
												}
											}
										}
									}
								},
								"flags": [],
								"personaData": {
									"name": {
										"id": "00000000-0000-0000-0000-000000000000",
										"value": {
											"variant": "eastern",
											"familyName": "Nakamoto",
											"givenNames": "Satoshi",
											"nickname": "Satoshi"
										}
									},
									"phoneNumbers": [
										{
											"id": "00000000-0000-0000-0000-000000000001",
											"value": "+46123456789"
										},
										{
											"id": "00000000-0000-0000-0000-000000000002",
											"value": "+44987654321"
										}
									],
									"emailAddresses": [
										{
											"id": "00000000-0000-0000-0000-000000000003",
											"value": "sat@os.hi"
										},
										{
											"id": "00000000-0000-0000-0000-000000000004",
											"value": "satoshi@nakamoto.btc"
										}
									]
								}
							},
							{
								"networkID": 1,
								"address": "identity_rdx12gcd4r799jpvztlffgw483pqcen98pjnay988n8rmscdswd872xy62",
								"displayName": "Batman",
								"securityState": {
									"discriminator": "unsecured",
									"unsecuredEntityControl": {
										"transactionSigning": {
											"factorSourceID": {
												"discriminator": "fromHash",
												"fromHash": {
													"kind": "device",
													"body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
												}
											},
											"badge": {
												"discriminator": "virtualSource",
												"virtualSource": {
													"discriminator": "hierarchicalDeterministicPublicKey",
													"hierarchicalDeterministicPublicKey": {
														"publicKey": {
															"curve": "curve25519",
															"compressedData": "1fe80badc0520334ee339e4010491d417ca3aed0c9621698b10655529f0ee506"
														},
														"derivationPath": {
															"scheme": "cap26",
															"path": "m/44H/1022H/1H/618H/1460H/1H"
														}
													}
												}
											}
										}
									}
								},
								"flags": ["deletedByUser"],
								"personaData": {
									"name": {
										"id": "00000000-0000-0000-0000-000000000000",
										"value": {
											"variant": "western",
											"familyName": "Wayne",
											"givenNames": "Bruce",
											"nickname": "Batman"
										}
									},
									"phoneNumbers": [
										{
											"id": "00000000-0000-0000-0000-000000000001",
											"value": "+1 13 371 337"
										}
									],
									"emailAddresses": [
										{
											"id": "00000000-0000-0000-0000-000000000002",
											"value": "bat@m.an"
										}
									]
								}
							}
						],
						"authorizedDapps":	[
							{
								"networkID": 1,
								"dAppDefinitionAddress": "account_rdx12x0xfz2yumu2qsh6yt0v8xjfc7et04vpsz775kc3yd3xvle4w5d5k5",
								"displayName": "Radix Dashboard",
								"referencesToAuthorizedPersonas": [
									{
										"identityAddress": "identity_rdx122kttqch0eehzj6f9nkkxcw7msfeg9udurq5u0ysa0e92c59w0mg6x",
										"lastLogin": "2024-01-31T14:23:45.000Z",
										"sharedAccounts": {
											"request": {
												"quantifier": "exactly",
												"quantity": 2
											},
											"ids": [
												"account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr",
												"account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264"
											]
										},
										"sharedPersonaData": {
											"name": "00000000-0000-0000-0000-000000000000",
											"emailAddresses": {
												"request": {
													"quantifier": "exactly",
													"quantity": 2
												},
												"ids": [
													"00000000-0000-0000-0000-000000000001",
													"00000000-0000-0000-0000-000000000002"
												]
											},
											"phoneNumbers": {
												"request": {
													"quantifier": "atLeast",
													"quantity": 1
												},
												"ids": [
													"00000000-0000-0000-0000-000000000003",
													"00000000-0000-0000-0000-000000000004"
												]
											}
										}
									},
									{
										"identityAddress": "identity_rdx12gcd4r799jpvztlffgw483pqcen98pjnay988n8rmscdswd872xy62",
										"lastLogin": "2024-01-31T14:23:45.000Z",
										"sharedAccounts": {
											"request": {
												"quantifier": "atLeast",
												"quantity": 1
											},
											"ids": [
												"account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264"
											]
										},
										"sharedPersonaData": {
											"name": "00000000-0000-0000-0000-0000000000f0",
											"emailAddresses": {
												"request": {
													"quantifier": "exactly",
													"quantity": 2
												},
												"ids": [
													"00000000-0000-0000-0000-0000000000f1",
													"00000000-0000-0000-0000-0000000000f2"
												]
											},
											"phoneNumbers": {
												"request": {
													"quantifier": "atLeast",
													"quantity": 1
												},
												"ids": [
													"00000000-0000-0000-0000-0000000000f3",
													"00000000-0000-0000-0000-0000000000f4"
												]
											}
										}
									}
								]
							},
							{
								"networkID": 1,
								"dAppDefinitionAddress": "account_rdx12xuhw6v30chdkhcu7qznz9vu926vxefr4h4tdvc0mdckg9rq4afx9t",
								"displayName": "Gumball Club",
								"referencesToAuthorizedPersonas": [
									{
										"identityAddress": "identity_rdx12gcd4r799jpvztlffgw483pqcen98pjnay988n8rmscdswd872xy62",
										"lastLogin": "2024-01-31T14:23:45.000Z",
										"sharedAccounts": {
											"request": {
												"quantifier": "atLeast",
												"quantity": 1
											},
											"ids": [
												"account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264"
											]
										},
										"sharedPersonaData": {
											"name": "00000000-0000-0000-0000-0000000000f0",
											"emailAddresses": {
												"request": {
													"quantifier": "exactly",
													"quantity": 2
												},
												"ids": [
													"00000000-0000-0000-0000-0000000000f1",
													"00000000-0000-0000-0000-0000000000f2"
												]
											},
											"phoneNumbers": {
												"request": {
													"quantifier": "atLeast",
													"quantity": 1
												},
												"ids": [
													"00000000-0000-0000-0000-0000000000f3",
													"00000000-0000-0000-0000-0000000000f4"
												]
											}
										}
									}
								]
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
								"displayName": "Nadia",
								"onLedgerSettings": {
									"thirdPartyDeposits": {
										"depositRule": "acceptAll",
										"assetsExceptionList": [],
										"depositorsAllowList": []
									}
								},
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
								"flags": ["deletedByUser"],
								"displayName": "Olivia",
								"onLedgerSettings": {
									"thirdPartyDeposits": {
										"depositRule": "acceptAll",
										"assetsExceptionList": [],
										"depositorsAllowList": []
									}
								},
								"address": "account_tdx_2_129663ef7fj8azge3y6sl73lf9vyqt53ewzlf7ul2l76mg5wyqlqlpr"
							}
						],
						"personas": [
							{
								"networkID": 2,
								"address": "identity_tdx_2_12fk6qyu2860xyx2jk7j6ex464ccrnxrve4kpaa8qyxx99y5627ahhc",
								"displayName": "Skywalker",
								"securityState": {
									"discriminator": "unsecured",
									"unsecuredEntityControl": {
										"transactionSigning": {
											"factorSourceID": {
												"discriminator": "fromHash",
												"fromHash": {
													"kind": "device",
													"body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
												}
											},
											"badge": {
												"discriminator": "virtualSource",
												"virtualSource": {
													"discriminator": "hierarchicalDeterministicPublicKey",
													"hierarchicalDeterministicPublicKey": {
														"publicKey": {
															"curve": "curve25519",
															"compressedData": "3c4d6f1267485854313c1ed81aea193b8f750cd081e3aa4dea29b93c34ca2261"
														},
														"derivationPath": {
															"scheme": "cap26",
															"path": "m/44H/1022H/2H/618H/1460H/0H"
														}
													}
												}
											}
										}
									}
								},
								"flags": [],
								"personaData": {
									"name": {
										"id": "00000000-0000-0000-0000-000000000000",
										"value": {
											"variant": "eastern",
											"familyName": "Skywalker",
											"givenNames": "Leia",
											"nickname": "Princess Leia"
										}
									},
									"phoneNumbers": [
										{
											"id": "00000000-0000-0000-0000-000000000001",
											"value": "+42 3 456 789"
										}
									],
									"emailAddresses": [
										{
											"id": "00000000-0000-0000-0000-000000000002",
											"value": "leia@sky.walker"
										}
									]
								}
							},
							{
								"networkID": 2,
								"address": "identity_tdx_2_12gr0d9da3jvye7mdrreljyqs35esjyjsl9r8t5v96hq6fq367cln08",
								"displayName": "Granger",
								"securityState": {
									"discriminator": "unsecured",
									"unsecuredEntityControl": {
										"transactionSigning": {
											"factorSourceID": {
												"discriminator": "fromHash",
												"fromHash": {
													"kind": "device",
													"body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
												}
											},
											"badge": {
												"discriminator": "virtualSource",
												"virtualSource": {
													"discriminator": "hierarchicalDeterministicPublicKey",
													"hierarchicalDeterministicPublicKey": {
														"publicKey": {
															"curve": "curve25519",
															"compressedData": "b6885032393165d56cce19850c2a3dbb80733d21c78c7314223e9c3a75f64c8d"
														},
														"derivationPath": {
															"scheme": "cap26",
															"path": "m/44H/1022H/2H/618H/1460H/1H"
														}
													}
												}
											}
										}
									}
								},
								"flags": ["deletedByUser"],
								"personaData": {
									"name": {
										"id": "00000000-0000-0000-0000-000000000000",
										"value": {
											"variant": "western",
											"familyName": "Granger",
											"givenNames": "Hermione",
											"nickname": "Hermy"
										}
									},
									"phoneNumbers": [
										{
											"id": "00000000-0000-0000-0000-000000000001",
											"value": "+44 123 456 77"
										}
									],
									"emailAddresses": [
										{
											"id": "00000000-0000-0000-0000-000000000002",
											"value": "granger.h@hogwarts.uk.co"
										}
									]
								}
							}
						],
						"authorizedDapps": 	[
							{
								"networkID": 2,
								"dAppDefinitionAddress": "account_tdx_2_128evrrwfp8gj9240qq0m06ukhwaj2cmejluxxreanzjwq62vmlf8r4",
								"displayName": "Dev Console",
								"referencesToAuthorizedPersonas": [
									{
										"identityAddress": "identity_tdx_2_12fk6qyu2860xyx2jk7j6ex464ccrnxrve4kpaa8qyxx99y5627ahhc",
										"lastLogin": "2024-01-31T14:23:45.000Z",
										"sharedAccounts": {
											"request": {
												"quantifier": "exactly",
												"quantity": 2
											},
											"ids": [
												"account_tdx_2_1289zm062j788dwrjefqkfgfeea5tkkdnh8htqhdrzdvjkql4kxceql",
												"account_tdx_2_129663ef7fj8azge3y6sl73lf9vyqt53ewzlf7ul2l76mg5wyqlqlpr"
											]
										},
										"sharedPersonaData": {
											"name": "00000000-0000-0000-0000-000000000000",
											"emailAddresses": {
												"request": {
													"quantifier": "exactly",
													"quantity": 2
												},
												"ids": [
													"00000000-0000-0000-0000-000000000001",
													"00000000-0000-0000-0000-000000000002"
												]
											},
											"phoneNumbers": {
												"request": {
													"quantifier": "atLeast",
													"quantity": 1
												},
												"ids": [
													"00000000-0000-0000-0000-000000000003",
													"00000000-0000-0000-0000-000000000004"
												]
											}
										}
									},
									{
										"identityAddress": "identity_tdx_2_12gr0d9da3jvye7mdrreljyqs35esjyjsl9r8t5v96hq6fq367cln08",
										"lastLogin": "2024-01-31T14:23:45.000Z",
										"sharedAccounts": {
											"request": {
												"quantifier": "atLeast",
												"quantity": 1
											},
											"ids": [
												"account_tdx_2_129663ef7fj8azge3y6sl73lf9vyqt53ewzlf7ul2l76mg5wyqlqlpr"
											]
										},
										"sharedPersonaData": {
											"name": "00000000-0000-0000-0000-0000000000f0",
											"emailAddresses": {
												"request": {
													"quantifier": "exactly",
													"quantity": 2
												},
												"ids": [
													"00000000-0000-0000-0000-0000000000f1",
													"00000000-0000-0000-0000-0000000000f2"
												]
											},
											"phoneNumbers": {
												"request": {
													"quantifier": "atLeast",
													"quantity": 1
												},
												"ids": [
													"00000000-0000-0000-0000-0000000000f3",
													"00000000-0000-0000-0000-0000000000f4"
												]
											}
										}
									}
								]
							},
							{
								"networkID": 2,
								"dAppDefinitionAddress": "account_tdx_2_12yf9gd53yfep7a669fv2t3wm7nz9zeezwd04n02a433ker8vza6rhe",
								"displayName": "Sandbox",
								"referencesToAuthorizedPersonas": [
									{
										"identityAddress": "identity_tdx_2_12gr0d9da3jvye7mdrreljyqs35esjyjsl9r8t5v96hq6fq367cln08",
										"lastLogin": "2024-01-31T14:23:45.000Z",
										"sharedAccounts": {
											"request": {
												"quantifier": "atLeast",
												"quantity": 1
											},
											"ids": [
												"account_tdx_2_129663ef7fj8azge3y6sl73lf9vyqt53ewzlf7ul2l76mg5wyqlqlpr"
											]
										},
										"sharedPersonaData": {
											"name": "00000000-0000-0000-0000-0000000000f0",
											"emailAddresses": {
												"request": {
													"quantifier": "exactly",
													"quantity": 2
												},
												"ids": [
													"00000000-0000-0000-0000-0000000000f1",
													"00000000-0000-0000-0000-0000000000f2"
												]
											},
											"phoneNumbers": {
												"request": {
													"quantifier": "atLeast",
													"quantity": 1
												},
												"ids": [
													"00000000-0000-0000-0000-0000000000f3",
													"00000000-0000-0000-0000-0000000000f4"
												]
											}
										}
									}
								]
							}
						]
					}
				]
			}
            "#,
        );
    }
}
