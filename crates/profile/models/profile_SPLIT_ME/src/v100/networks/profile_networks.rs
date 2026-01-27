use crate::prelude::*;

decl_identified_vec_of!(
    /// An ordered mapping of NetworkID -> `Profile.Network`, containing
    /// all the users Accounts, Personas and AuthorizedDapps the user
    /// has created and interacted with on this network.
    ProfileNetwork
);

impl ProfileNetworks {
    pub fn content_hint(&self) -> ContentHint {
        let number_of_accounts =
            self.iter().fold(0, |acc, x| acc + x.accounts.len());
        let number_of_personas =
            self.iter().fold(0, |per, x| per + x.personas.len());
        ContentHint::with_counters(
            number_of_accounts,
            number_of_personas,
            self.len(),
        )
    }
}

impl HasSampleValues for ProfileNetworks {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::from_iter([
            ProfileNetwork::sample_mainnet(),
            ProfileNetwork::sample_stokenet(),
        ])
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::just(ProfileNetwork::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ProfileNetworks;

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
    fn append_is_noop_if_already_contains() {
        let mut sut = SUT::sample();
        assert_eq!(sut.len(), 2);
        assert_eq!(sut[1].accounts.len(), 2);

        let outcome =
            sut.append(ProfileNetwork::new_empty_on(NetworkID::Stokenet));
        assert_eq!(outcome, (false, 1));

        // assert NOOP
        assert_eq!(sut.len(), 2);
        assert_eq!(sut[1].accounts.len(), 2);
    }

    #[test]
    fn duplicates_are_prevented() {
        assert_eq!(
            SUT::from_iter(
                [ProfileNetwork::sample(), ProfileNetwork::sample()]
                    .into_iter()
            )
            .len(),
            1
        )
    }

    #[test]
    fn duplicates_are_prevented_and_first_added_is_retained() {
        let mut sut = SUT::from_iter([ProfileNetwork::new(
            NetworkID::Mainnet,
            Accounts::from_iter([
                Account::sample_mainnet_alice(),
                Account::sample_mainnet_bob(),
            ]),
            Personas::default(),
            AuthorizedDapps::default(),
            ResourcePreferences::default(),
            MFAFactorInstances::default(),
        )]);
        assert!(
            !sut.append(ProfileNetwork::new(
                NetworkID::Mainnet,
                Accounts::from_iter([Account::sample_mainnet_carol()]),
                Personas::default(),
                AuthorizedDapps::default(),
                ResourcePreferences::default(),
                MFAFactorInstances::default(),
            ))
            .0
        );

        assert_eq!(
            sut.get_id(NetworkID::Mainnet).unwrap().accounts.items(),
            [
                Account::sample_mainnet_alice(),
                Account::sample_mainnet_bob()
            ]
        );
    }

    #[test]
    fn with_network() {
        let network = ProfileNetwork::new(
            NetworkID::Mainnet,
            Accounts::just(Account::sample_mainnet()),
            Personas::default(),
            AuthorizedDapps::default(),
            ResourcePreferences::default(),
            MFAFactorInstances::default(),
        );
        assert_eq!(SUT::just(network).len(), 1);
    }

    #[test]
    fn content_hint() {
        assert_eq!(
            SUT::sample().content_hint(),
            ContentHint::with_counters(4, 4, 2)
        );
    }

    #[test]
    fn json_roundtrip() {
        let sut = SUT::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
                       [
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
                    "flags": [],
                    "appearanceID": 0,
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
                    "flags": [],
                    "appearanceID": 1,
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
                    ],
                    "preferences": {
                      "deposits": "visible"
                    }
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
                    ],
                    "preferences": {
                      "deposits": "visible"
                    }
                  }
                ],
                "resource_preferences": [
                  {
                    "resource": {
                      "kind": "fungible",
                      "value": "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd"
                    },
                    "visibility": "hidden"
                  },
                  {
                    "resource": {
                      "kind": "nonFungible",
                      "value": "resource_rdx1t4dy69k6s0gv040xa64cyadyefwtett62ng6xfdnljyydnml7t6g3j"
                    },
                    "visibility": "visible"
                  }
                ],
                "mfa_factor_instances": [
                  {
                    "factorInstance": {
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
                              "compressedData": "427969814e15d74c3ff4d9971465cb709d210c8a7627af9466bdaa67bd0929b7"
                            },
                            "derivationPath": {
                              "scheme": "cap26",
                              "path": "m/44H/1022H/1H/525H/1460H/0S"
                            }
                          }
                        }
                      }
                    }
                  },
                  {
                    "factorInstance": {
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
                              "compressedData": "10ccd9b906660d49b3fe89651baa1284fc7b19ad2c3d423a7828ec350c0e5fe0"
                            },
                            "derivationPath": {
                              "scheme": "cap26",
                              "path": "m/44H/1022H/1H/525H/1460H/1S"
                            }
                          }
                        }
                      }
                    }
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
                    "flags": [],
                    "appearanceID": 0,
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
                    "flags": [
                      "deletedByUser"
                    ],
                    "appearanceID": 1,
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
                    ],
                    "preferences": {
                      "deposits": "visible"
                    }
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
                    ],
                    "preferences": {
                      "deposits": "visible"
                    }
                  }
                ],
                "resource_preferences": [
                  {
                    "resource": {
                      "kind": "nonFungible",
                      "value": "resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc"
                    },
                    "visibility": "visible"
                  }
                ],
                "mfa_factor_instances": [
                  {
                    "factorInstance": {
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
                              "compressedData": "a621ea332665b993d4e9e1727e2e9a589129cae85823bb536d5c4d96a9adea5a"
                            },
                            "derivationPath": {
                              "scheme": "cap26",
                              "path": "m/44H/1022H/2H/525H/1460H/0S"
                            }
                          }
                        }
                      }
                    }
                  },
                  {
                    "factorInstance": {
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
                              "compressedData": "2adb1b05e5378bd7659b14bfa24b98a61c6a10d189f6c46ff68090f02858fa6e"
                            },
                            "derivationPath": {
                              "scheme": "cap26",
                              "path": "m/44H/1022H/2H/525H/1460H/1S"
                            }
                          }
                        }
                      }
                    }
                  }
                ]
              }
            ]
        	"#,
        );
    }
}
