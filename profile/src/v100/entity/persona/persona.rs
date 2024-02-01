use crate::prelude::*;

/// A Persona is an identity a user choses to login to a dApp with, using
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
    uniffi::Record,
)]
#[display("{} | {}", display_name, address)]
#[serde(rename_all = "camelCase")]
pub struct Persona {
    /// The ID of the network this account can be used with.
    #[serde(rename = "networkID")]
    pub network_id: NetworkID,

    /// The address of an identity, used by Personas, a bech32 encoding of a public key hash
    /// that starts with the prefix `"identity_"`, dependent on NetworkID, meaning the same
    /// public key used for two IdentityAddresses on two different networks will not have
    /// the same address.
    pub address: IdentityAddress,

    /// An off-ledger display name or description chosen by the user when they
    /// created this persona.
    pub display_name: DisplayName,

    /// Describes the state this Persona is in, in regards to how
    /// the user controls it, i.e. if it is controlled by a single factor (private key)
    ///  or an `AccessController` with a potential Multi-Factor setup.
    pub security_state: EntitySecurityState,

    /// An order set of `EntityFlag`s used to describe certain Off-ledger
    /// user state about this Persona, e.g. if it is marked as hidden or not.
    #[serde(default)]
    pub flags: EntityFlags,

    /// Personal information a user has associated with a certain Persona, of different kinds, such as name,
    /// email address(es) or phone number(s). This information is only ever stored in Profile and is never
    /// uploaded to the Radix Network.
    pub persona_data: PersonaData,
}

impl Persona {
    /// Creates a new `Persona`, if `persona_data` is `None`, an empty object will be created.
    pub fn new(
        persona_creating_factor_instance: HDFactorInstanceIdentityCreation,
        display_name: DisplayName,
        persona_data: Option<PersonaData>,
    ) -> Self {
        let address =
            IdentityAddress::from_hd_factor_instance_virtual_entity_creation(
                persona_creating_factor_instance.clone(),
            );
        Self {
            network_id: persona_creating_factor_instance.network_id(),
            address,
            display_name,
            security_state:
                UnsecuredEntityControl::with_entity_creating_factor_instance(
                    persona_creating_factor_instance,
                )
                .into(),
            flags: EntityFlags::default(),
            persona_data: persona_data.unwrap_or_default(),
        }
    }
}

impl Persona {
    #[cfg(not(tarpaulin_include))] // false negative
    fn placeholder_at_index_name_network<P, E>(
        network_id: NetworkID,
        index: HDPathValue,
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
        let mwp = MnemonicWithPassphrase::placeholder();
        let bdfs = DeviceFactorSource::babylon(
            true,
            mwp.clone(),
            WalletClientModel::Iphone,
        );

        let private_hd_factor_source =
            PrivateHierarchicalDeterministicFactorSource::new(mwp, bdfs);

        let id = IDGenerator::<PersonaDataEntryID>::starting_at(1);
        let phone_numbers = CollectionOfPhoneNumbers::entries(
            phone_numbers
                .into_iter()
                .map(|s| s.parse::<PersonaDataEntryPhoneNumber>().unwrap())
                .map(|v| {
                    PersonaDataIdentifiedPhoneNumber::with_id(id.next(), v)
                }),
        );

        let email_addresses = CollectionOfEmailAddresses::entries(
            email_addresses
                .into_iter()
                .map(|s| s.parse::<PersonaDataEntryEmailAddress>().unwrap())
                .map(|v| {
                    PersonaDataIdentifiedEmailAddress::with_id(id.next(), v)
                }),
        );

        let mut persona = Self::new(
            private_hd_factor_source
                .derive_entity_creation_factor_instance(network_id, index),
            DisplayName::new(display_name).unwrap(),
            Some(PersonaData::new(
                Some(PersonaDataIdentifiedName::with_id(
                    PersonaDataEntryID::nil(),
                    name,
                )),
                phone_numbers,
                email_addresses,
            )),
        );
        if is_hidden {
            persona.flags.insert_flag(EntityFlag::DeletedByUser);
        }
        persona
    }

    #[cfg(not(tarpaulin_include))] // false negative
    fn placeholder_at_index_name<P, E>(
        index: HDPathValue,
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
        Self::placeholder_at_index_name_network(
            NetworkID::Mainnet,
            index,
            display_name,
            is_hidden,
            name,
            phone_numbers,
            email_addresses,
        )
    }

    pub fn placeholder_satoshi() -> Self {
        Self::placeholder_mainnet_satoshi()
    }

    pub fn placeholder_batman() -> Self {
        Self::placeholder_mainnet_batman()
    }

    pub fn placeholder_mainnet_satoshi() -> Self {
        let name = PersonaDataEntryName::new(
            Variant::Eastern,
            "Nakamoto",
            "Satoshi",
            "Satoshi",
        )
        .expect("Failure to construct placeholder Name should not be possible");
        Self::placeholder_at_index_name(
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

    pub fn placeholder_mainnet_batman() -> Self {
        let name = PersonaDataEntryName::new(
            Variant::Western,
            "Wayne",
            "Bruce",
            "Batman",
        )
        .expect("Failure to construct placeholder Name should not be possible");
        Self::placeholder_at_index_name(
            1,
            "Batman",
            true,
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

    pub fn placeholder_stokenet_leia_skywalker() -> Self {
        Self::placeholder_at_index_name_network(
            NetworkID::Stokenet,
            0,
            "Skywalker",
            false,
            PersonaDataEntryName::new(
                Variant::Eastern,
                "Skywalker",
                "Leia",
                "Princess Leia",
            )
            .expect(
                "Failure to construct placeholder Name should not be possible",
            ),
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

    pub fn placeholder_stokenet_hermione() -> Self {
        Self::placeholder_at_index_name_network(
            NetworkID::Stokenet,
            1,
            "Granger",
            true,
            PersonaDataEntryName::new(
                Variant::Western,
                "Granger",
                "Hermione",
                "Hermy",
            )
            .expect(
                "Failure to construct placeholder Name should not be possible",
            ),
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
}

impl Ord for Persona {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.security_state, &other.security_state) {
            (
                EntitySecurityState::Unsecured { value: l },
                EntitySecurityState::Unsecured { value: r },
            ) => l
                .transaction_signing
                .derivation_path()
                .last_component()
                .cmp(r.transaction_signing.derivation_path().last_component()),
        }
    }
}

impl PartialOrd for Persona {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Add conformance to Identifiable in order to use `identified_vec`
impl Identifiable for Persona {
    type ID = IdentityAddress;

    fn id(&self) -> Self::ID {
        self.address.clone()
    }
}

/// Placeholder conformance to facilitate unit-tests
impl HasPlaceholder for Persona {
    fn placeholder() -> Self {
        Self::placeholder_satoshi()
    }

    fn placeholder_other() -> Self {
        Self::placeholder_batman()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(Persona::placeholder(), Persona::placeholder());
        assert_eq!(Persona::placeholder_other(), Persona::placeholder_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Persona::placeholder(), Persona::placeholder_other());
    }

    #[test]
    fn compare() {
        assert!(Persona::placeholder_batman() > Persona::placeholder_satoshi());
    }

    #[test]
    fn new_with_identity_and_name() {
        let identity_address: IdentityAddress =
			"identity_rdx12gcd4r799jpvztlffgw483pqcen98pjnay988n8rmscdswd872xy62"
				.parse()
				.unwrap();
        let persona = Persona::placeholder_batman();
        assert_eq!(persona.address, identity_address);
    }

    #[test]
    fn display() {
        let account = Persona::placeholder_batman();
        assert_eq!(
			format!("{account}"),
			"Batman | identity_rdx12gcd4r799jpvztlffgw483pqcen98pjnay988n8rmscdswd872xy62"
		);
    }

    #[test]
    fn identifiable() {
        let persona = Persona::placeholder_batman();
        let identity_address: IdentityAddress =
			"identity_rdx12gcd4r799jpvztlffgw483pqcen98pjnay988n8rmscdswd872xy62"
				.parse()
				.unwrap();
        assert_eq!(persona.id(), identity_address);
    }

    #[test]
    fn json_roundtrip_mainnet_satoshi() {
        let model = Persona::placeholder_satoshi();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
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
			}
			"#,
        );
    }

    #[test]
    fn json_roundtrip_mainnet_batman() {
        let model = Persona::placeholder_batman();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
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
			"#,
        );
    }

    #[test]
    fn json_roundtrip_stokenet_leia() {
        let model = Persona::placeholder_stokenet_leia_skywalker();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
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
			}
			"#,
        );
    }

    #[test]
    fn json_roundtrip_stokenet_hermione() {
        let model = Persona::placeholder_stokenet_hermione();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
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
			"#,
        );
    }

    #[test]
    fn json_deserialization_works_without_flags_as_version_1_0_0_of_app() {
        let json = serde_json::from_str(
			r#"
			{
				"networkID": 1,
				"address": "identity_rdx122kttqch0eehzj6f9nkkxcw7msfeg9udurq5u0ysa0e92c59w0mg6x",
				"displayName": "No Flags",
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
				"personaData": {
					"phoneNumbers": [],
					"emailAddresses": []
				}
			}
			"#,
		).unwrap();
        let persona = serde_json::from_value::<Persona>(json).unwrap();
        assert_eq!(persona.flags.len(), 0); // assert Default value is empty flags.
    }
}
