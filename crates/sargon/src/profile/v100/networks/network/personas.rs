use crate::prelude::*;

decl_identified_vec_of!(
    /// An ordered set of [`Persona`]s on a specific network.
    Persona
);

impl HasSampleValues for Personas {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::sample_stokenet()
    }
}

impl HasSampleValuesOnNetworks for Personas {
    /// A sample used to facilitate unit tests.
    fn sample_mainnet() -> Self {
        Self::from_iter([
            Persona::sample_mainnet_satoshi(),
            Persona::sample_mainnet_batman(),
        ])
    }

    /// A sample used to facilitate unit tests.
    fn sample_stokenet() -> Self {
        Self::from_iter([
            Persona::sample_stokenet_leia_skywalker(),
            Persona::sample_stokenet_hermione(),
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
        assert_ne!(Personas::sample(), Personas::sample_other());
    }

    #[test]
    fn equality() {
        assert_eq!(Personas::sample(), Personas::sample());
        assert_eq!(Personas::sample_other(), Personas::sample_other());
    }

    #[test]
    fn duplicates_are_prevented() {
        assert_eq!(
            Personas::from_iter(
                [Persona::sample(), Persona::sample()].into_iter()
            )
            .len(),
            1
        )
    }

    #[test]
    fn with_one() {
        assert_eq!(Personas::just(Persona::sample()).len(), 1)
    }

    #[test]
    fn get_all() {
        assert_eq!(Personas::sample().get_all().len(), 2);
    }

    #[test]
    fn get_by_address() {
        let persona = Persona::sample();
        let address = persona.address;
        let personas = Personas::just(persona.clone());
        assert_eq!(personas.get_id(address), Some(&persona));
    }

    #[test]
    fn json_roundtrip_mainnet() {
        let sut = Personas::sample_mainnet();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            [
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
            ]
            "#,
        );
    }

    #[test]
    fn json_roundtrip_stokenet() {
        let sut = Personas::sample_stokenet();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
			[
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
