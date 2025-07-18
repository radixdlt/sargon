use std::ops::{Deref, DerefMut};

use crate::prelude::*;

/// A Persona is an identity a user chooses to login to a dApp with, using
/// RadixConnect - Radix decentralized login solution. A persona is very
/// similar to [`Account`]s, in the sense that they are On-Network/On-Ledger
/// components, with a unique network dependent address ([`IdentityAddress`])
/// and with a security state (see [`EntitySecurityState`]) knowing which
/// factor instances that control this component, but with one important
/// difference: a Persona cannot hold funds. It is impossible to transfer
/// any asset to a Persona. The On-Network component representation of
/// the Persona is called `Identity`. The concept "Persona" is a Radix
/// Wallet (Profile) *application* of an Identity.
///
/// Personas have data (see [`PersonaData`]), which is personal information
/// a user has associated with a this Persona, of different kinds, such as name,
/// email address(es) or phone number(s). The `PersonaData` is **never** uploaded
/// to the Radix Network, i.e. it is a pure Radix Wallet (Profile) construct,
/// On-Network Identities does not know of PersonaData, and never will (well
/// technically, nothing stops a user from building their own wallet and uploading
/// personal information to the metadata of the Identity component... but `Sargon`
/// never will, nor will the Radix Wallet.).
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Hash,
    Eq,
    derive_more::Display,
)]
#[display("{} | {}", self.display_name, self.address)]
#[serde(rename_all = "camelCase")]
pub struct Persona {
    #[serde(flatten)]
    base: BaseEntity<IdentityAddress>,

    /// Personal information a user has associated with a certain Persona, of different kinds, such as name,
    /// email address(es) or phone number(s). This information is only ever stored in Profile and is never
    /// uploaded to the Radix Network.
    pub persona_data: PersonaData,
}

impl Persona {
    pub fn with(
        network_id: impl Into<NetworkID>,
        address: impl Into<IdentityAddress>,
        display_name: impl Into<DisplayName>,
        security_state: impl Into<EntitySecurityState>,
        flags: impl IntoIterator<Item = EntityFlag>,
        persona_data: impl Into<PersonaData>,
    ) -> Self {
        Self {
            base: BaseEntity::new(
                network_id,
                address,
                display_name,
                security_state,
                flags,
            ),
            persona_data: persona_data.into(),
        }
    }
}

impl Deref for Persona {
    type Target = BaseEntity<IdentityAddress>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Persona {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl HasEntityKind for Persona {
    fn entity_kind() -> CAP26EntityKind {
        CAP26EntityKind::Identity
    }
}

impl IsSecurityStateAware for Persona {
    fn is_securified(&self) -> bool {
        self.security_state().is_securified()
    }
}

impl HasFactorInstances for Persona {
    fn unique_tx_signing_factor_instances(&self) -> IndexSet<FactorInstance> {
        self.security_state().unique_tx_signing_factor_instances()
    }
}

impl HasSecurityState for Persona {
    fn security_state(&self) -> EntitySecurityState {
        self.security_state.clone()
    }
    fn set_security_state_unchecked(&mut self, new_state: EntitySecurityState) {
        self.security_state = new_state;
    }
}

impl IsBaseEntity for Persona {
    type Address = IdentityAddress;

    fn address(&self) -> Self::Address {
        self.address
    }
    fn flags(&self) -> EntityFlags {
        self.flags.clone()
    }
}

impl IsNetworkAware for Persona {
    fn network_id(&self) -> NetworkID {
        self.address().network_id()
    }
}

impl IsEntityWithoutConcreteTypes for Persona {
    type Path = IdentityPath;

    fn with_veci_and_name(
        veci: HDFactorInstanceTransactionSigning<Self::Path>,
        name: DisplayName,
    ) -> Self {
        let address =
            IdentityAddress::from_hd_factor_instance_virtual_entity_creation(
                veci.clone(),
            );
        Self::with(
            veci.network_id(),
            address,
            name,
            UnsecuredEntityControl::with_entity_creating_factor_instance(veci),
            EntityFlags::default(),
            PersonaData::default(),
        )
    }
}

impl Persona {
    /// Creates a new `Persona`, if `persona_data` is `None`, an empty object will be created.
    pub fn new(
        persona_creating_factor_instance: HDFactorInstanceIdentityCreation,
        display_name: DisplayName,
        persona_data: impl Into<Option<PersonaData>>,
    ) -> Self {
        let mut self_ = Self::with_veci_and_name(
            persona_creating_factor_instance,
            display_name,
        );
        if let Some(persona_data) = persona_data.into() {
            self_.persona_data = persona_data;
        }
        self_
    }
}

impl Persona {
    fn sample_at_index_name_network<P, E>(
        network_id: NetworkID,
        index: u32,
        display_name: &str,
        is_hidden: bool,
        name: PersonaDataEntryName,
        phone_numbers: P,
        email_addresses: E,
    ) -> Self
    where
        P: IntoIterator<Item = String>,
        E: IntoIterator<Item = String>,
    {
        let mwp = MnemonicWithPassphrase::sample();
        let bdfs = DeviceFactorSource::babylon(&mwp, &HostInfo::sample());

        let private_hd_factor_source =
            PrivateHierarchicalDeterministicFactorSource::new(mwp, bdfs);

        let id = IDStepper::<PersonaDataEntryID>::new();
        let name =
            PersonaDataIdentifiedName::with_id(unsafe { id.next() }, name);
        let phone_numbers = CollectionOfPhoneNumbers::from_iter(
            phone_numbers
                .into_iter()
                .map(|s| s.parse::<PersonaDataEntryPhoneNumber>().unwrap())
                .map(|v| unsafe {
                    PersonaDataIdentifiedPhoneNumber::with_id(id.next(), v)
                }),
        );

        let email_addresses = CollectionOfEmailAddresses::from_iter(
            email_addresses
                .into_iter()
                .map(|s| s.parse::<PersonaDataEntryEmailAddress>().unwrap())
                .map(|v| unsafe {
                    PersonaDataIdentifiedEmailAddress::with_id(id.next(), v)
                }),
        );

        let mut persona = Self::new(
            private_hd_factor_source._derive_entity_creation_factor_instance(
                network_id,
                HDPathComponent::Unsecurified(Unsecurified::Hardened(
                    UnsecurifiedHardened::from_local_key_space(index).unwrap(),
                )),
            ),
            DisplayName::new(display_name).unwrap(),
            PersonaData::new(name, phone_numbers, email_addresses),
        );
        if is_hidden {
            persona.flags.insert(EntityFlag::HiddenByUser);
        }
        persona
    }

    fn sample_at_index_name<P, E>(
        index: u32,
        display_name: &str,
        is_hidden: bool,
        name: PersonaDataEntryName,
        phone_numbers: P,
        email_addresses: E,
    ) -> Self
    where
        P: IntoIterator<Item = String>,
        E: IntoIterator<Item = String>,
    {
        Self::sample_at_index_name_network(
            NetworkID::Mainnet,
            index,
            display_name,
            is_hidden,
            name,
            phone_numbers,
            email_addresses,
        )
    }

    pub fn sample_mainnet() -> Self {
        Self::sample_mainnet_satoshi()
    }

    pub fn sample_mainnet_other() -> Self {
        Self::sample_mainnet_batman()
    }

    pub fn sample_mainnet_third() -> Self {
        Self::sample_mainnet_ripley()
    }

    pub fn sample_mainnet_satoshi() -> Self {
        let name = PersonaDataEntryName::new(
            PersonaDataNameVariant::Eastern,
            "Nakamoto",
            "Satoshi",
            "Satoshi",
        )
        .expect("Failure to construct sample Name should not be possible");
        Self::sample_at_index_name(
            0,
            "Satoshi",
            false,
            name,
            ["+46123456789", "+44987654321"]
                .into_iter()
                .map(|s| s.to_string())
                .collect_vec(),
            ["sat@os.hi", "satoshi@nakamoto.btc"]
                .into_iter()
                .map(|s| s.to_string())
                .collect_vec(),
        )
    }

    pub fn sample_mainnet_batman() -> Self {
        let name = PersonaDataEntryName::new(
            PersonaDataNameVariant::Western,
            "Wayne",
            "Bruce",
            "Batman",
        )
        .expect("Failure to construct sample Name should not be possible");
        Self::sample_at_index_name(
            1,
            "Batman",
            false,
            name,
            ["+1 13 371 337"]
                .into_iter()
                .map(|s| s.to_string())
                .collect_vec(),
            ["bat@m.an"]
                .into_iter()
                .map(|s| s.to_string())
                .collect_vec(),
        )
    }

    pub fn sample_mainnet_ripley() -> Self {
        let name = PersonaDataEntryName::new(
            PersonaDataNameVariant::Western,
            "Ripley",
            "Ellen",
            "",
        )
        .expect("Failure to construct sample Name should not be possible");
        Self::sample_at_index_name(
            2,
            "Ellen Ripley",
            false,
            name,
            ["+1-210-456-9876"]
                .into_iter()
                .map(|s| s.to_string())
                .collect_vec(),
            ["ellen.riplay@weylandyutani.corp"]
                .into_iter()
                .map(|s| s.to_string())
                .collect_vec(),
        )
    }

    pub fn sample_mainnet_turing() -> Self {
        let name = PersonaDataEntryName::new(
            PersonaDataNameVariant::Western,
            "Alan",
            "Turing",
            "",
        )
        .expect("Failure to construct sample Name should not be possible");
        Self::sample_at_index_name(
            2,
            "Turing",
            true,
            name,
            ["+1-211-564-7698"]
                .into_iter()
                .map(|s| s.to_string())
                .collect_vec(),
            ["alan@turing.hero"]
                .into_iter()
                .map(|s| s.to_string())
                .collect_vec(),
        )
    }

    pub fn sample_stokenet_leia_skywalker() -> Self {
        Self::sample_at_index_name_network(
            NetworkID::Stokenet,
            0,
            "Skywalker",
            false,
            PersonaDataEntryName::new(
                PersonaDataNameVariant::Eastern,
                "Skywalker",
                "Leia",
                "Princess Leia",
            )
            .expect("Failure to construct sample Name should not be possible"),
            ["+42 3 456 789"]
                .into_iter()
                .map(|s| s.to_string())
                .collect_vec(),
            ["leia@sky.walker"]
                .into_iter()
                .map(|s| s.to_string())
                .collect_vec(),
        )
    }

    pub fn sample_stokenet_hermione() -> Self {
        Self::sample_at_index_name_network(
            NetworkID::Stokenet,
            1,
            "Granger",
            true,
            PersonaDataEntryName::new(
                PersonaDataNameVariant::Western,
                "Granger",
                "Hermione",
                "Hermy",
            )
            .expect("Failure to construct sample Name should not be possible"),
            ["+44 123 456 77"]
                .into_iter()
                .map(|s| s.to_string())
                .collect_vec(),
            ["granger.h@hogwarts.uk.co"]
                .into_iter()
                .map(|s| s.to_string())
                .collect_vec(),
        )
    }

    pub fn sample_stokenet_connor() -> Self {
        Self::sample_at_index_name_network(
            NetworkID::Stokenet,
            2,
            "Sarah Connor",
            false,
            PersonaDataEntryName::new(
                PersonaDataNameVariant::Western,
                "Connor",
                "Sarah",
                "",
            )
            .expect("Failure to construct sample Name should not be possible"),
            ["+1-210-456-7890"]
                .into_iter()
                .map(|s| s.to_string())
                .collect_vec(),
            ["sarah.connor@resistance.now"]
                .into_iter()
                .map(|s| s.to_string())
                .collect_vec(),
        )
    }

    pub fn sample_stokenet() -> Self {
        Self::sample_stokenet_leia_skywalker()
    }

    pub fn sample_stokenet_other() -> Self {
        Self::sample_stokenet_hermione()
    }

    pub fn sample_stokenet_third() -> Self {
        Self::sample_stokenet_connor()
    }
}

/// Add conformance to Identifiable in order to use `IdentifiedVecOf`
impl Identifiable for Persona {
    type ID = IdentityAddress;

    fn id(&self) -> Self::ID {
        self.address
    }
}

/// Sample conformance to facilitate unit-tests
impl HasSampleValues for Persona {
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    fn sample_other() -> Self {
        Self::sample_mainnet_other()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Persona;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());

        assert_eq!(SUT::sample_mainnet(), SUT::sample_mainnet());
        assert_eq!(SUT::sample_mainnet_other(), SUT::sample_mainnet_other());

        assert_eq!(SUT::sample_stokenet(), SUT::sample_stokenet());
        assert_eq!(SUT::sample_stokenet_other(), SUT::sample_stokenet_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
        assert_ne!(SUT::sample_mainnet(), SUT::sample_mainnet_other());
        assert_ne!(SUT::sample_stokenet(), SUT::sample_stokenet_other());
        assert_ne!(SUT::sample_stokenet(), SUT::sample_mainnet());
        assert_ne!(SUT::sample_stokenet_other(), SUT::sample_mainnet_other());

        assert_ne!(SUT::sample_mainnet_other(), SUT::sample_mainnet_third());

        assert_ne!(SUT::sample_stokenet_other(), SUT::sample_stokenet_third());
    }

    #[test]
    fn test_is_network_aware() {
        assert_eq!(SUT::sample().network_id(), NetworkID::Mainnet);
    }

    #[test]
    fn new_with_identity_and_name() {
        let identity_address: IdentityAddress =
			"identity_rdx12tw6rt9c4l56rz6p866e35tmzp556nymxmpj8hagfewq82kspctdyw"
				.parse()
				.unwrap();
        let persona = SUT::sample_mainnet_other();
        assert_eq!(persona.address, identity_address);
    }

    #[test]
    fn display() {
        let account = SUT::sample_mainnet_other();
        assert_eq!(
			format!("{account}"),
			"Batman | identity_rdx12tw6rt9c4l56rz6p866e35tmzp556nymxmpj8hagfewq82kspctdyw"
		);
    }

    #[test]
    fn identifiable() {
        let persona = SUT::sample_mainnet_other();
        let identity_address: IdentityAddress =
			"identity_rdx12tw6rt9c4l56rz6p866e35tmzp556nymxmpj8hagfewq82kspctdyw"
				.parse()
				.unwrap();
        assert_eq!(persona.id(), identity_address);
    }

    #[test]
    fn new_with_persona_data() {
        let persona_data = PersonaData::sample_other();
        let sut = SUT::new(
            HDFactorInstanceTransactionSigning::<IdentityPath>::sample(),
            DisplayName::sample(),
            persona_data.clone(),
        );
        assert_eq!(sut.persona_data, persona_data);
    }

    #[test]
    fn json_roundtrip_mainnet_satoshi() {
        let model = SUT::sample_mainnet();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
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
			}
			"#,
        );
    }

    #[test]
    fn json_roundtrip_mainnet_batman() {
        let model = SUT::sample_mainnet_other();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
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
			"#,
        );
    }

    #[test]
    fn json_roundtrip_stokenet_leia() {
        let model = SUT::sample_stokenet_leia_skywalker();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
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
			}
			"#,
        );
    }

    #[test]
    fn json_roundtrip_stokenet_hermione() {
        let model = SUT::sample_stokenet_hermione();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
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
			"#,
        );
    }

    #[test]
    fn json_deserialization_works_without_flags_as_version_1_0_0_of_app() {
        let json = serde_json::from_str(
			r#"
			{
				"networkID": 1,
				"address": "identity_rdx122yy9pkfdrkam4evxcwh235c4qc52wujkwnt52q7vqxefhnlen489g",
				"displayName": "No Flags",
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
				"personaData": {
					"phoneNumbers": [],
					"emailAddresses": []
				}
			}
			"#,
		).unwrap();
        let persona = serde_json::from_value::<SUT>(json).unwrap();
        assert_eq!(persona.flags.len(), 0); // assert Default value is empty flags.
    }
}
