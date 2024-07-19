use crate::prelude::*;

/// The canonical representation of a users accounts, personas,
/// authorized dapps, security factors, settings and more.
///
/// This large structure of values is called 'wallet backup data'
/// in user facing tests in host applications, but internally at
/// RDX Works known as "the Profile".
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
    pub fn analyze_contents_of_file(
        json_string: impl AsRef<str>,
    ) -> ProfileFileContents {
        let json_string = json_string.as_ref();
        if let Ok(profile) = Profile::new_from_json_string(json_string) {
            return ProfileFileContents::PlaintextProfile(profile);
        };

        if serde_json::from_str::<EncryptedProfileSnapshot>(json_string).is_ok()
        {
            return ProfileFileContents::EncryptedProfile;
        };

        ProfileFileContents::NotProfile
    }
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
    /// Creates a new Profile from the `DeviceFactorSource` and `DeviceInfo`.
    ///
    /// The Profile is initialized with a Mainnet ProfileNetwork, which is
    /// "empty" (no Accounts, Personas etc).
    ///
    /// # Panics
    /// Panics if the `device_factor_source` is not a BDFS and not marked "main".
    pub fn from_device_factor_source(
        device_factor_source: DeviceFactorSource,
        creating_device: HostInfo,
    ) -> Self {
        if !device_factor_source.is_main_bdfs() {
            panic!("DeviceFactorSource is not main BDFS");
        }
        let bdfs = device_factor_source;
        let header = Header::new(creating_device.into());
        Self::with(
            header,
            FactorSources::with_bdfs(bdfs),
            AppPreferences::default(),
            ProfileNetworks::just(ProfileNetwork::new_empty_on(
                NetworkID::Mainnet,
            )),
        )
    }

    /// Creates a new Profile from the `MnemonicWithPassphrase` and `DeviceInfo`,
    /// by initializing a `DeviceFactorInstance` using `DeviceInfo` as source for
    /// `DeviceFactorSourceHint` which will be the BDFS of the Profile.
    ///
    /// The Profile is initialized with a Mainnet ProfileNetwork, which is
    /// "empty" (no Accounts, Personas etc).
    pub fn from_mnemonic_with_passphrase(
        mnemonic_with_passphrase: MnemonicWithPassphrase,
        host_info: HostInfo,
    ) -> Self {
        let bdfs = DeviceFactorSource::babylon(
            true,
            &mnemonic_with_passphrase,
            &host_info,
        );
        Self::from_device_factor_source(bdfs, host_info)
    }

    /// Creates a new Profile from the `Mnemonic` (no passphrase) and `DeviceInfo`,
    /// by initializing a `DeviceFactorInstance` using `DeviceInfo` as source for
    /// `DeviceFactorSourceHint` which will be the BDFS of the Profile.
    ///
    /// The Profile is initialized with a Mainnet ProfileNetwork, which is
    /// "empty" (no Accounts, Personas etc).
    pub fn new(mnemonic: Mnemonic, host_info: HostInfo) -> Self {
        Self::from_mnemonic_with_passphrase(
            MnemonicWithPassphrase::new(mnemonic),
            host_info,
        )
    }

    pub fn with(
        header: Header,
        factor_sources: FactorSources,
        app_preferences: AppPreferences,
        networks: ProfileNetworks,
    ) -> Self {
        if factor_sources.is_empty() {
            panic!("FactorSources MUST NOT be empty.")
        }
        debug!("Creating new Profile, header: {:?}", &header);
        Self {
            header,
            factor_sources,
            app_preferences,
            networks,
        }
    }

    pub fn new_from_encrypted_profile_json_string(
        json_string: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> Result<Self> {
        let json_string = json_string.as_ref();
        serde_json::from_str::<EncryptedProfileSnapshot>(json_string)
            .map_failed_to_deserialize_string::<EncryptedProfileSnapshot>(
                json_string,
            )
            .and_then(|encrypted| encrypted.decrypt(password))
    }

    pub fn to_encrypted_profile_json_str(
        &self,
        password: impl AsRef<str>,
    ) -> String {
        let encrypted =
            EncryptedProfileSnapshot::encrypting(self, password, None, None);
        serde_json::to_string(&encrypted).expect(
            "JSON serialization of EncryptedProfileSnapshot should never fail.",
        )
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

    pub fn update_last_used_of_factor_source(
        &mut self,
        id: &FactorSourceID,
    ) -> Result<()> {
        self.update_any_factor_source_common(id, |common| {
            common.last_used_on = now();
        })
    }

    pub fn update_factor_source_remove_flag_main(
        &mut self,
        id: &FactorSourceID,
    ) -> Result<()> {
        self.update_any_factor_source_common(id, |common| {
            common.flags.remove_id(&FactorSourceFlag::Main);
        })
    }

    pub fn update_any_factor_source_common<F>(
        &mut self,
        factor_source_id: &FactorSourceID,
        mut mutate: F,
    ) -> Result<()>
    where
        F: FnMut(&mut FactorSourceCommon),
    {
        self.update_any_factor_source(factor_source_id, |fs| {
            let mut common = fs.common_properties();
            mutate(&mut common);
            fs.set_common_properties(common);
        })
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
        self.factor_sources
            .maybe_update_with(factor_source_id, |f| {
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

    pub fn update_any_factor_source<F>(
        &mut self,
        factor_source_id: &FactorSourceID,
        mutate: F,
    ) -> Result<()>
    where
        F: FnMut(&mut FactorSource),
    {
        self.factor_sources
            .try_update_with(factor_source_id, mutate)
    }
}

impl ProtoProfileMaybeWithLegacyP2PLinks {
    pub fn contains_legacy_links(&self) -> bool {
        !self.app_preferences.p2p_links.is_empty()
    }
}

impl Profile {
    pub fn check_if_profile_json_contains_legacy_p2p_links(
        json_str: impl AsRef<str>,
    ) -> bool {
        let json_str = json_str.as_ref();
        serde_json::from_str::<ProtoProfileMaybeWithLegacyP2PLinks>(json_str)
            .map_or_else(|_| false, |s| s.contains_legacy_links())
    }

    pub fn check_if_profile_json_bytes_contains_legacy_p2p_links(
        json: impl AsRef<[u8]>,
    ) -> bool {
        let json = json.as_ref();
        serde_json::from_slice::<ProtoProfileMaybeWithLegacyP2PLinks>(json)
            .map_or_else(|_| false, |s| s.contains_legacy_links())
    }

    pub fn check_if_encrypted_profile_json_contains_legacy_p2p_links(
        json_string: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> bool {
        let json_string = json_string.as_ref();
        serde_json::from_str::<EncryptedProfileSnapshot>(json_string)
            .map_failed_to_deserialize_string::<EncryptedProfileSnapshot>(
                json_string,
            )
            .and_then(|encrypted| encrypted.decrypt_to_bytes(password))
            .map_or_else(
                |_| false,
                Profile::check_if_profile_json_bytes_contains_legacy_p2p_links,
            )
    }
}

impl JsonDataDeserializing for Profile {}
impl JsonDataSerializing for Profile {}

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
    fn new_creates_empty_mainnet_network() {
        let sut = SUT::new(Mnemonic::sample(), HostInfo::sample());
        assert_eq!(
            sut.networks,
            ProfileNetworks::just(ProfileNetwork::new_empty_on(
                NetworkID::Mainnet
            ))
        );
    }

    #[should_panic(expected = "FactorSources MUST NOT be empty.")]
    #[test]
    fn not_allowed_to_create_profile_with_empty_factor_source() {
        let _ = SUT::with(
            Header::sample(),
            IdentifiedVecOf::new(),
            AppPreferences::sample(),
            ProfileNetworks::sample(),
        );
    }

    #[test]
    fn serialize_empty_factor_sources_is_err() {
        let mut sut = SUT::sample();
        sut.factor_sources = FactorSources::new();
        assert!(serde_json::to_value(sut).is_err());
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
    #[should_panic(expected = "DeviceFactorSource is not main BDFS")]
    fn new_from_non_main_bdfs_panics() {
        let _ = SUT::from_device_factor_source(
            DeviceFactorSource::sample_other(),
            HostInfo::sample(),
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
        assert!(sut.factor_sources.contains_id(FactorSourceID::from(
            DeviceFactorSource::sample().id
        )));

        assert_eq!(
            sut.factor_sources
                .get_id(id)
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
                .get_id(id)
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

        assert!(sut.factor_sources.contains_id(FactorSourceID::from(
            DeviceFactorSource::sample().id
        )));

        assert_eq!(
            sut.factor_sources
                .get_id(id)
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
                .get_id(id)
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
    fn test_analyze_contents_of_file_plaintext_profile() {
        let sut = SUT::sample();
        let json_str = sut.to_json_string(false);
        let contents = SUT::analyze_contents_of_file(json_str);
        assert_eq!(contents, ProfileFileContents::PlaintextProfile(sut));
    }

    #[test]
    fn test_analyze_contents_of_file_encrypted_profile() {
        let sut = SUT::sample();
        let json_str = sut.to_encrypted_profile_json_str("super secret");
        let contents = SUT::analyze_contents_of_file(json_str);
        assert_eq!(contents, ProfileFileContents::EncryptedProfile);
    }

    #[test]
    fn test_analyze_contents_of_file_not_profile() {
        let contents = SUT::analyze_contents_of_file("bello");
        assert_eq!(contents, ProfileFileContents::NotProfile);
    }

    #[test]
    fn update_name_of_accounts() {
        let mut sut = SUT::sample();
        let account = sut
            .networks
            .get_id(NetworkID::Mainnet)
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
                .get_id(NetworkID::Mainnet)
                .unwrap()
                .accounts
                .get_at_index(0)
                .unwrap()
                .display_name
                .value,
            "Satoshi"
        );
    }

    #[test]
    fn hash() {
        let n = 100;
        let set = (0..n)
            .map(|_| SUT::new(Mnemonic::generate_new(), HostInfo::sample()))
            .collect::<HashSet<_>>();
        assert_eq!(set.len(), n);
    }

    #[test]
    fn to_json_bytes_new_from_json_bytes() {
        let sut = SUT::sample();

        let encoded = sut.to_json_bytes();
        let profile_result = SUT::new_from_json_bytes(encoded).unwrap();
        assert_eq!(profile_result, sut);
    }

    #[test]
    fn new_from_json_bytes_error() {
        let malformed_profile_snapshot = BagOfBytes::from("{}".as_bytes());

        assert_eq!(
            SUT::new_from_json_bytes(malformed_profile_snapshot.clone()),
            Result::Err(CommonError::FailedToDeserializeJSONToValue {
                json_byte_count: malformed_profile_snapshot.len() as u64,
                type_name: "Profile".to_string(),
                serde_message: "missing field `header` at line 1 column 2"
                    .to_string()
            })
        );
    }

    #[test]
    fn from_encrypted_profile_json_str_valid() {
        let json_str =
            serde_json::to_string(&EncryptedProfileSnapshot::sample()).unwrap();
        let sut =
            SUT::new_from_encrypted_profile_json_string(json_str, "babylon")
                .unwrap();
        assert_eq!(
            sut.header.id,
            ProfileID::from_str("e5e4477b-e47b-4b64-bbc8-f8f40e8beb74")
                .unwrap()
        );
    }

    #[test]
    fn from_encrypted_profile_json_str_invalid_is_err() {
        assert_eq!(
            SUT::new_from_encrypted_profile_json_string(
                "We came we saw we kicked its ass!",
                "Mellon"
            ),
            Err(CommonError::FailedToDeserializeJSONToValue {
                json_byte_count: 33,
                type_name: "EncryptedProfileSnapshot".to_string(),
                serde_message: "expected value at line 1 column 1".to_string()
            })
        );
    }

    #[test]
    fn encryption_roundtrip() {
        let sut = SUT::sample();
        let password = "super secret";
        let encrypted = sut.to_encrypted_profile_json_str(password);
        assert_eq!(
            SUT::new_from_encrypted_profile_json_string(encrypted, password)
                .unwrap(),
            sut
        );
    }

    #[test]
    fn check_if_profile_json_contains_legacy_p2p_links_when_p2p_links_are_present(
    ) {
        let json = r#"
        {
            "appPreferences": {
              "p2pLinks": [
                {
                  "connectionPassword": "babebabebabebabebabebabebabebabebabebabebabebabebabebabebabebabe",
                  "displayName": "Brave on PC"
                }
              ]
            }
          }
        "#;
        assert!(SUT::check_if_profile_json_contains_legacy_p2p_links(json));
    }

    #[test]
    fn check_if_profile_json_contains_legacy_p2p_links_when_empty_json_str() {
        assert!(!SUT::check_if_profile_json_contains_legacy_p2p_links(""));
    }

    #[test]
    fn check_if_profile_json_contains_legacy_p2p_links_when_empty_p2p_links() {
        let json = r#"
        {
            "appPreferences": {
                "p2pLinks": []
              }
            }
          }
        "#;
        assert!(!SUT::check_if_profile_json_contains_legacy_p2p_links(json));
    }

    #[test]
    fn check_if_profile_json_contains_legacy_p2p_links_in_profile_snapshot_version_100(
    ) {
        let json = include_str!(concat!(
            env!("FIXTURES_VECTOR"),
            "only_plaintext_profile_snapshot_version_100.json"
        ));
        assert!(SUT::check_if_profile_json_contains_legacy_p2p_links(json));
    }

    #[test]
    fn check_if_encrypted_profile_json_contains_legacy_p2p_links_when_p2p_links_are_present(
    ) {
        let json =
            serde_json::to_string(&EncryptedProfileSnapshot::sample()).unwrap();
        let password = "babylon";
        assert!(
            SUT::check_if_encrypted_profile_json_contains_legacy_p2p_links(
                json, password
            )
        );
    }

    #[test]
    fn check_if_encrypted_profile_json_contains_legacy_p2p_links_when_empty_json(
    ) {
        let password = "babylon";
        assert!(
            !SUT::check_if_encrypted_profile_json_contains_legacy_p2p_links(
                "", password
            )
        );
    }

    #[test]
    fn json_roundtrip() {
        let sut = SUT::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
				"header": {
					"snapshotVersion": 100,
					"id": "12345678-bbbb-cccc-dddd-abcd12345678",
					"creatingDevice": {
						"id": "66f07ca2-a9d9-49e5-8152-77aca3d1dd74",
						"date": "2023-09-11T16:05:56.000Z",
						"description": "iPhone (iPhone)"
					},
					"lastUsedOnDevice": {
						"id": "66f07ca2-a9d9-49e5-8152-77aca3d1dd74",
						"date": "2023-09-11T16:05:56.000Z",
						"description": "iPhone (iPhone)"
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
								"body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
							},
							"common": {
								"cryptoParameters": {
									"supportedCurves": [
										"curve25519"
									],
									"supportedDerivationPathSchemes": [
										"cap26"
									]
								},
								"addedOn": "2023-09-11T16:05:56.000Z",
								"lastUsedOn": "2023-09-11T16:05:56.000Z",
								"flags": [
									"main"
								]
							},
							"hint": {
								"name": "My precious",
								"model": "iPhone SE 2nd gen",
								"mnemonicWordCount": 24,
								"systemVersion": "iOS 17.4.1",
                                 "hostAppVersion": "1.6.4",
                                 "hostVendor": "Apple"
							}
						}
					},
					{
						"discriminator": "ledgerHQHardwareWallet",
						"ledgerHQHardwareWallet": {
							"id": {
								"kind": "ledgerHQHardwareWallet",
								"body": "ab59987eedd181fe98e512c1ba0f5ff059f11b5c7c56f15614dcc9fe03fec58b"
							},
							"common": {
								"cryptoParameters": {
									"supportedCurves": [
										"curve25519"
									],
									"supportedDerivationPathSchemes": [
										"cap26"
									]
								},
								"addedOn": "2023-09-11T16:05:56.000Z",
								"lastUsedOn": "2023-09-11T16:05:56.000Z",
								"flags": []
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
						"isDeveloperModeEnabled": true,
						"securityStructuresOfFactorSourceIDs": []
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
								"networkID": 1,
								"address": "account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87",
								"displayName": "Alice",
								"securityState": {
									"discriminator": "unsecured",
									"unsecuredEntityControl": {
										"transactionSigning": {
											"factorSourceID": {
												"discriminator": "fromHash",
												"fromHash": {
													"kind": "device",
													"body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
												}
											},
											"badge": {
												"discriminator": "virtualSource",
												"virtualSource": {
													"discriminator": "hierarchicalDeterministicPublicKey",
													"hierarchicalDeterministicPublicKey": {
														"publicKey": {
															"curve": "curve25519",
															"compressedData": "c05f9fa53f203a01cbe43e89086cae29f6c7cdd5a435daa9e52b69e656739b36"
														},
														"derivationPath": {
															"scheme": "cap26",
															"path": "m/44H/1022H/1H/525H/1460H/0H"
														}
													}
												}
											}
										}
									}
								},
								"appearanceID": 0,
								"flags": [],
								"onLedgerSettings": {
									"thirdPartyDeposits": {
										"depositRule": "acceptAll",
										"assetsExceptionList": [],
										"depositorsAllowList": []
									}
								}
							},
							{
								"networkID": 1,
								"address": "account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7",
								"displayName": "Bob",
								"securityState": {
									"discriminator": "unsecured",
									"unsecuredEntityControl": {
										"transactionSigning": {
											"factorSourceID": {
												"discriminator": "fromHash",
												"fromHash": {
													"kind": "device",
													"body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
												}
											},
											"badge": {
												"discriminator": "virtualSource",
												"virtualSource": {
													"discriminator": "hierarchicalDeterministicPublicKey",
													"hierarchicalDeterministicPublicKey": {
														"publicKey": {
															"curve": "curve25519",
															"compressedData": "a3a14ce3c0e549ac35f1875738c243bb6f4037f08d7d2a52ef749091a92a0c71"
														},
														"derivationPath": {
															"scheme": "cap26",
															"path": "m/44H/1022H/1H/525H/1460H/1H"
														}
													}
												}
											}
										}
									}
								},
								"appearanceID": 1,
								"flags": [],
								"onLedgerSettings": {
									"thirdPartyDeposits": {
										"depositRule": "acceptAll",
										"assetsExceptionList": [],
										"depositorsAllowList": []
									}
								}
							}
						],
						"personas": [
							{
								"networkID": 1,
								"address": "identity_rdx122yy9pkfdrkam4evxcwh235c4qc52wujkwnt52q7vqxefhnlen489g",
								"displayName": "Satoshi",
								"securityState": {
									"discriminator": "unsecured",
									"unsecuredEntityControl": {
										"transactionSigning": {
											"factorSourceID": {
												"discriminator": "fromHash",
												"fromHash": {
													"kind": "device",
													"body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
												}
											},
											"badge": {
												"discriminator": "virtualSource",
												"virtualSource": {
													"discriminator": "hierarchicalDeterministicPublicKey",
													"hierarchicalDeterministicPublicKey": {
														"publicKey": {
															"curve": "curve25519",
															"compressedData": "e284e28bfca2103d554854d7cce822a2682610eb16b4c27bcd1b9cbd78bb931a"
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
								"address": "identity_rdx12tw6rt9c4l56rz6p866e35tmzp556nymxmpj8hagfewq82kspctdyw",
								"displayName": "Batman",
								"securityState": {
									"discriminator": "unsecured",
									"unsecuredEntityControl": {
										"transactionSigning": {
											"factorSourceID": {
												"discriminator": "fromHash",
												"fromHash": {
													"kind": "device",
													"body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
												}
											},
											"badge": {
												"discriminator": "virtualSource",
												"virtualSource": {
													"discriminator": "hierarchicalDeterministicPublicKey",
													"hierarchicalDeterministicPublicKey": {
														"publicKey": {
															"curve": "curve25519",
															"compressedData": "675aa54df762f24df8f6b38122e75058a18fe55a3dbb030b4c0bb504bacc7e81"
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
								"flags": [],
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
						"authorizedDapps": [
							{
								"networkID": 1,
								"dAppDefinitionAddress": "account_rdx12x0xfz2yumu2qsh6yt0v8xjfc7et04vpsz775kc3yd3xvle4w5d5k5",
								"displayName": "Radix Dashboard",
								"referencesToAuthorizedPersonas": [
									{
										"identityAddress": "identity_rdx122yy9pkfdrkam4evxcwh235c4qc52wujkwnt52q7vqxefhnlen489g",
										"lastLogin": "2024-01-31T14:23:45.000Z",
										"sharedAccounts": {
											"request": {
												"quantifier": "exactly",
												"quantity": 2
											},
											"ids": [
												"account_rdx128dtethfy8ujrsfdztemyjk0kvhnah6dafr57frz85dcw2c8z0td87",
												"account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7"
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
													"00000000-0000-0000-0000-000000000003",
													"00000000-0000-0000-0000-000000000004"
												]
											},
											"phoneNumbers": {
												"request": {
													"quantifier": "exactly",
													"quantity": 2
												},
												"ids": [
													"00000000-0000-0000-0000-000000000001",
													"00000000-0000-0000-0000-000000000002"
												]
											}
										}
									},
									{
										"identityAddress": "identity_rdx12tw6rt9c4l56rz6p866e35tmzp556nymxmpj8hagfewq82kspctdyw",
										"lastLogin": "2024-01-31T14:23:45.000Z",
										"sharedAccounts": {
											"request": {
												"quantifier": "atLeast",
												"quantity": 1
											},
											"ids": [
												"account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7"
											]
										},
										"sharedPersonaData": {
											"name": "00000000-0000-0000-0000-000000000000",
											"emailAddresses": {
												"request": {
													"quantifier": "exactly",
													"quantity": 1
												},
												"ids": [
													"00000000-0000-0000-0000-000000000002"
												]
											},
											"phoneNumbers": {
												"request": {
													"quantifier": "exactly",
													"quantity": 1
												},
												"ids": [
													"00000000-0000-0000-0000-000000000001"
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
										"identityAddress": "identity_rdx12tw6rt9c4l56rz6p866e35tmzp556nymxmpj8hagfewq82kspctdyw",
										"lastLogin": "2024-01-31T14:23:45.000Z",
										"sharedAccounts": {
											"request": {
												"quantifier": "atLeast",
												"quantity": 1
											},
											"ids": [
												"account_rdx12y02nen8zjrq0k0nku98shjq7n05kvl3j9m5d3a6cpduqwzgmenjq7"
											]
										},
										"sharedPersonaData": {
											"name": "00000000-0000-0000-0000-000000000000",
											"emailAddresses": {
												"request": {
													"quantifier": "exactly",
													"quantity": 1
												},
												"ids": [
													"00000000-0000-0000-0000-000000000002"
												]
											},
											"phoneNumbers": {
												"request": {
													"quantifier": "exactly",
													"quantity": 1
												},
												"ids": [
													"00000000-0000-0000-0000-000000000001"
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
								"networkID": 2,
								"address": "account_tdx_2_128jx5fmru80v38a7hun8tdhajf2exef756c92tfg4atwl3y4pqn48m",
								"displayName": "Nadia",
								"securityState": {
									"discriminator": "unsecured",
									"unsecuredEntityControl": {
										"transactionSigning": {
											"factorSourceID": {
												"discriminator": "fromHash",
												"fromHash": {
													"kind": "device",
													"body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
												}
											},
											"badge": {
												"discriminator": "virtualSource",
												"virtualSource": {
													"discriminator": "hierarchicalDeterministicPublicKey",
													"hierarchicalDeterministicPublicKey": {
														"publicKey": {
															"curve": "curve25519",
															"compressedData": "535e0b74beffc99d96acd36ae73444c0e35ebb5707f077f9bf1120b1bb8894c0"
														},
														"derivationPath": {
															"scheme": "cap26",
															"path": "m/44H/1022H/2H/525H/1460H/0H"
														}
													}
												}
											}
										}
									}
								},
								"appearanceID": 0,
								"flags": [],
								"onLedgerSettings": {
									"thirdPartyDeposits": {
										"depositRule": "acceptAll",
										"assetsExceptionList": [],
										"depositorsAllowList": []
									}
								}
							},
							{
								"networkID": 2,
								"address": "account_tdx_2_12xvlee7xtg7dx599yv69tzkpeqzn4wr2nlnn3gpsm0zu0v9luqdpnp",
								"displayName": "Olivia",
								"securityState": {
									"discriminator": "unsecured",
									"unsecuredEntityControl": {
										"transactionSigning": {
											"factorSourceID": {
												"discriminator": "fromHash",
												"fromHash": {
													"kind": "device",
													"body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
												}
											},
											"badge": {
												"discriminator": "virtualSource",
												"virtualSource": {
													"discriminator": "hierarchicalDeterministicPublicKey",
													"hierarchicalDeterministicPublicKey": {
														"publicKey": {
															"curve": "curve25519",
															"compressedData": "436c67c678713be6a4306bf2a64d62d29c9bccb92a776175e5cb6e95e87be55d"
														},
														"derivationPath": {
															"scheme": "cap26",
															"path": "m/44H/1022H/2H/525H/1460H/1H"
														}
													}
												}
											}
										}
									}
								},
								"appearanceID": 1,
								"flags": [
									"deletedByUser"
								],
								"onLedgerSettings": {
									"thirdPartyDeposits": {
										"depositRule": "acceptAll",
										"assetsExceptionList": [],
										"depositorsAllowList": []
									}
								}
							}
						],
						"personas": [
							{
								"networkID": 2,
								"address": "identity_tdx_2_122r7248dkyjwt2kxf36de26w7htdwpzsm3lyjr4p0nvrgwn025dds8",
								"displayName": "Skywalker",
								"securityState": {
									"discriminator": "unsecured",
									"unsecuredEntityControl": {
										"transactionSigning": {
											"factorSourceID": {
												"discriminator": "fromHash",
												"fromHash": {
													"kind": "device",
													"body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
												}
											},
											"badge": {
												"discriminator": "virtualSource",
												"virtualSource": {
													"discriminator": "hierarchicalDeterministicPublicKey",
													"hierarchicalDeterministicPublicKey": {
														"publicKey": {
															"curve": "curve25519",
															"compressedData": "d3dd2992834813ba76d6619021560b759e81f7391a5cdbb8478feb3bfa8cb9e4"
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
								"address": "identity_tdx_2_12tltwh00wvvur4yymv63pwhhwhjzvu4za2fy7vnyue36v5dtq3pgvq",
								"displayName": "Granger",
								"securityState": {
									"discriminator": "unsecured",
									"unsecuredEntityControl": {
										"transactionSigning": {
											"factorSourceID": {
												"discriminator": "fromHash",
												"fromHash": {
													"kind": "device",
													"body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
												}
											},
											"badge": {
												"discriminator": "virtualSource",
												"virtualSource": {
													"discriminator": "hierarchicalDeterministicPublicKey",
													"hierarchicalDeterministicPublicKey": {
														"publicKey": {
															"curve": "curve25519",
															"compressedData": "c287e135eac194e4d6b6c65a2545988686b941509043bab026ef9717fd6b4f4e"
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
								"flags": [
									"deletedByUser"
								],
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
						"authorizedDapps": [
							{
								"networkID": 2,
								"dAppDefinitionAddress": "account_tdx_2_128evrrwfp8gj9240qq0m06ukhwaj2cmejluxxreanzjwq62vmlf8r4",
								"displayName": "Dev Console",
								"referencesToAuthorizedPersonas": [
									{
										"identityAddress": "identity_tdx_2_122r7248dkyjwt2kxf36de26w7htdwpzsm3lyjr4p0nvrgwn025dds8",
										"lastLogin": "2024-01-31T14:23:45.000Z",
										"sharedAccounts": {
											"request": {
												"quantifier": "exactly",
												"quantity": 2
											},
											"ids": [
												"account_tdx_2_128jx5fmru80v38a7hun8tdhajf2exef756c92tfg4atwl3y4pqn48m",
												"account_tdx_2_12xvlee7xtg7dx599yv69tzkpeqzn4wr2nlnn3gpsm0zu0v9luqdpnp"
											]
										},
										"sharedPersonaData": {
											"name": "00000000-0000-0000-0000-000000000000",
											"emailAddresses": {
												"request": {
													"quantifier": "exactly",
													"quantity": 1
												},
												"ids": [
													"00000000-0000-0000-0000-000000000002"
												]
											},
											"phoneNumbers": {
												"request": {
													"quantifier": "exactly",
													"quantity": 1
												},
												"ids": [
													"00000000-0000-0000-0000-000000000001"
												]
											}
										}
									},
									{
										"identityAddress": "identity_tdx_2_12tltwh00wvvur4yymv63pwhhwhjzvu4za2fy7vnyue36v5dtq3pgvq",
										"lastLogin": "2024-01-31T14:23:45.000Z",
										"sharedAccounts": {
											"request": {
												"quantifier": "atLeast",
												"quantity": 1
											},
											"ids": [
												"account_tdx_2_12xvlee7xtg7dx599yv69tzkpeqzn4wr2nlnn3gpsm0zu0v9luqdpnp"
											]
										},
										"sharedPersonaData": {
											"name": "00000000-0000-0000-0000-000000000000",
											"emailAddresses": {
												"request": {
													"quantifier": "exactly",
													"quantity": 1
												},
												"ids": [
													"00000000-0000-0000-0000-000000000002"
												]
											},
											"phoneNumbers": {
												"request": {
													"quantifier": "exactly",
													"quantity": 1
												},
												"ids": [
													"00000000-0000-0000-0000-000000000001"
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
										"identityAddress": "identity_tdx_2_12tltwh00wvvur4yymv63pwhhwhjzvu4za2fy7vnyue36v5dtq3pgvq",
										"lastLogin": "2024-01-31T14:23:45.000Z",
										"sharedAccounts": {
											"request": {
												"quantifier": "atLeast",
												"quantity": 1
											},
											"ids": [
												"account_tdx_2_12xvlee7xtg7dx599yv69tzkpeqzn4wr2nlnn3gpsm0zu0v9luqdpnp"
											]
										},
										"sharedPersonaData": {
											"name": "00000000-0000-0000-0000-000000000000",
											"emailAddresses": {
												"request": {
													"quantifier": "exactly",
													"quantity": 1
												},
												"ids": [
													"00000000-0000-0000-0000-000000000002"
												]
											},
											"phoneNumbers": {
												"request": {
													"quantifier": "exactly",
													"quantity": 1
												},
												"ids": [
													"00000000-0000-0000-0000-000000000001"
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
