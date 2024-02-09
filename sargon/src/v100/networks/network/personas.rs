use crate::prelude::*;

/// An ordered set of Personas on a specific network.
pub type Personas = IdentifiedVecVia<Persona>;

impl Personas {
    /// Instantiates a new collection of personas from
    /// and iterator of personas.
    pub fn with_personas<I>(personas: I) -> Self
    where
        I: IntoIterator<Item = Persona>,
    {
        Self::from_iter(personas)
    }

    /// Instantiates a new collection of personas from a
    /// single persona.
    pub fn with_persona(persona: Persona) -> Self {
        Self::with_personas([persona])
    }
}

// Trait: Default
impl Default for Personas {
    /// Instantiates a new empty collection.
    fn default() -> Self {
        Self::new()
    }
}

impl Personas {
    /// Returns a reference to the persona identified by `address`, if it exists.
    pub fn get_persona_by_address(
        &self,
        address: &IdentityAddress,
    ) -> Option<&Persona> {
        self.get(address)
    }

    /// Returns references to **all** personas, including hidden ones.
    pub fn get_all(&self) -> Vec<&Persona> {
        self.elements()
    }
}

impl HasPlaceholder for Personas {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::placeholder_mainnet()
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
        Self::placeholder_stokenet()
    }
}

impl Personas {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_mainnet() -> Self {
        Self::with_personas([
            Persona::placeholder_mainnet_satoshi(),
            Persona::placeholder_mainnet_batman(),
        ])
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_stokenet() -> Self {
        Self::with_personas([
            Persona::placeholder_stokenet_leia_skywalker(),
            Persona::placeholder_stokenet_hermione(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn default_is_empty() {
        assert_eq!(Personas::default().len(), 0);
    }

    #[test]
    fn inequality() {
        assert_ne!(Personas::placeholder(), Personas::placeholder_other());
    }

    #[test]
    fn equality() {
        assert_eq!(Personas::placeholder(), Personas::placeholder());
        assert_eq!(
            Personas::placeholder_other(),
            Personas::placeholder_other()
        );
    }

    #[test]
    fn duplicates_are_prevented() {
        assert_eq!(
            Personas::with_personas(
                [Persona::placeholder(), Persona::placeholder()].into_iter()
            )
            .len(),
            1
        )
    }

    #[test]
    fn with_one() {
        assert_eq!(Personas::with_persona(Persona::placeholder()).len(), 1)
    }

    #[test]
    fn get_all() {
        assert_eq!(Personas::placeholder().get_all().len(), 2);
    }

    #[test]
    fn get_by_address() {
        let persona = Persona::placeholder();
        let address = persona.address.clone();
        let personas = Personas::with_persona(persona.clone());
        assert_eq!(personas.get_persona_by_address(&address), Some(&persona));
    }

    #[test]
    fn json_roundtrip_mainnet() {
        let sut = Personas::placeholder_mainnet();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            [
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
            ]
            "#,
        );
    }

    #[test]
    fn json_roundtrip_stokenet() {
        let sut = Personas::placeholder_stokenet();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
			[
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
			]
            "#,
        );
    }
}
