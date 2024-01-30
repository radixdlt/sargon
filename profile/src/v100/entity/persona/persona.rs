use crate::prelude::*;

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
    fn placeholder_at_index_name_network(
        network_id: NetworkID,
        index: HDPathValue,
        display_name: &str,
        name: Name,
        phone_numbers: CollectionOfPhoneNumbers,
        email_addresses: CollectionOfEmailAddresses,
    ) -> Self {
        let mwp = MnemonicWithPassphrase::placeholder();
        let bdfs = DeviceFactorSource::babylon(
            true,
            mwp.clone(),
            WalletClientModel::Iphone,
        );
        let private_hd_factor_source =
            PrivateHierarchicalDeterministicFactorSource::new(mwp, bdfs);
        let persona_creating_factor_instance: HDFactorInstanceTransactionSigning<IdentityPath> =
            private_hd_factor_source.derive_entity_creation_factor_instance(network_id, index);

        Self::new(
            persona_creating_factor_instance,
            DisplayName::new(display_name).unwrap(),
            Some(PersonaData::new(
                Some(PersonaDataIdentifiedName::with_id(Uuid::nil(), name)),
                phone_numbers,
                email_addresses,
            )),
        )
    }

    fn placeholder_at_index_name(
        index: HDPathValue,
        display_name: &str,
        name: Name,
        phone_numbers: CollectionOfPhoneNumbers,
        email_addresses: CollectionOfEmailAddresses,
    ) -> Self {
        Self::placeholder_at_index_name_network(
            NetworkID::Mainnet,
            index,
            display_name,
            name,
            phone_numbers,
            email_addresses,
        )
    }

    pub fn placeholder_satoshi() -> Self {
        let name =
            Name::new(Variant::Eastern, "Nakamoto", "Satoshi", "Satoshi")
                .expect(
                "Failure to construct placeholder Name should not be possible",
            );
        Self::placeholder_at_index_name(
            0,
            "Satoshi",
            name,
            CollectionOfPhoneNumbers::placeholder(),
            CollectionOfEmailAddresses::placeholder(),
        )
    }

    pub fn placeholder_batman() -> Self {
        let name = Name::new(Variant::Western, "Wayne", "Bruce", "Batman")
            .expect(
                "Failure to construct placeholder Name should not be possible",
            );
        Self::placeholder_at_index_name(
            1,
            "Batman",
            name,
            CollectionOfPhoneNumbers::default(),
            CollectionOfEmailAddresses::default(),
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
    fn display_name_get_set() {
        let mut persona = Persona::placeholder_batman();
        assert_eq!(persona.display_name.value, "Batman");
        let new_display_name = DisplayName::new("Satoshi").unwrap();
        persona.display_name = new_display_name.clone();
        assert_eq!(persona.display_name, new_display_name);
    }

    #[test]
    fn update() {
        let mut persona = Persona::placeholder_satoshi();
        assert_eq!(persona.display_name.value, "Satoshi");
        persona.display_name = DisplayName::new("Batman").unwrap();
        assert_eq!(persona.display_name.value, "Batman");
    }

    #[test]
    fn flags_get_set() {
        let mut persona = Persona::placeholder_batman();
        assert_eq!(persona.flags, EntityFlags::default());
        let new_flags = EntityFlags::with_flag(EntityFlag::DeletedByUser);
        persona.flags = new_flags.clone();
        assert_eq!(persona.flags, new_flags);
    }

    #[test]
    fn placeholder_display_name() {
        let placeholder = Persona::placeholder();
        assert_eq!(
            placeholder.display_name,
            DisplayName::new("Satoshi").unwrap()
        );
    }

    #[test]
    fn placeholder_other_display_name() {
        let placeholder = Persona::placeholder_other();
        assert_eq!(
            placeholder.display_name,
            DisplayName::new("Batman").unwrap()
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
    fn json_roundtrip_batman() {
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
				"flags": [],
				"personaData": {
					"name": {
						"id": "00000000-0000-0000-0000-000000000000",
						"value": {
							"variant": "Western",
							"familyName": "Wayne",
							"givenName": "Bruce",
							"nickname": "Batman"
						}
					},
                    "phoneNumbers": []
				}
			}
			"#,
        );
    }

    #[test]
    fn json_roundtrip_satoshi() {
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
							"variant": "Eastern",
							"familyName": "Nakamoto",
							"givenName": "Satoshi",
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
				"personaData": {
					"name": {
						"id": "00000000-0000-0000-0000-000000000000",
						"value": {
							"variant": "Eastern",
							"familyName": "Nakamoto",
							"givenName": "Satoshi",
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
                    ]
				}
			}
            "#,
        ).unwrap();
        let persona = serde_json::from_value::<Persona>(json).unwrap();
        assert_eq!(persona.display_name.value, "Satoshi".to_string()); // soundness
        assert_eq!(persona.flags.len(), 0); // assert Default value is empty flags.
                                            // assert_eq!(persona.persona_data, PersonaData::new());
    }

    #[test]
    fn placeholder_mainnet() {
        let persona = Persona::placeholder_batman();
        assert_eq!(persona.display_name.value, "Batman".to_string());
        assert_eq!(persona.network_id, NetworkID::Mainnet);
    }

    #[test]
    fn placeholder_stokenet() {
        let persona = Persona::placeholder_at_index_name_network(
            NetworkID::Stokenet,
            1,
            "Batman",
            Name::placeholder(),
            CollectionOfPhoneNumbers::placeholder(),
            CollectionOfEmailAddresses::placeholder(),
        );
        assert_eq!(persona.display_name.value, "Batman".to_string());
        assert_eq!(persona.network_id, NetworkID::Stokenet);
        assert_eq!(persona.address.address, "identity_tdx_2_12gr0d9da3jvye7mdrreljyqs35esjyjsl9r8t5v96hq6fq367cln08");
    }

    #[test]
    fn placeholder_stokenet_satoshi() {
        let persona = Persona::placeholder_at_index_name_network(
            NetworkID::Stokenet,
            0,
            "Satoshi",
            Name::placeholder(),
            CollectionOfPhoneNumbers::placeholder(),
            CollectionOfEmailAddresses::placeholder(),
        );
        assert_eq!(persona.display_name.value, "Satoshi".to_string());
        assert_eq!(persona.network_id, NetworkID::Stokenet);
        assert_eq!(persona.address.address, "identity_tdx_2_12fk6qyu2860xyx2jk7j6ex464ccrnxrve4kpaa8qyxx99y5627ahhc");
    }
}
