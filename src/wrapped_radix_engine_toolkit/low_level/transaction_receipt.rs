use crate::prelude::*;

use radix_engine::transaction::TransactionReceiptV1 as ScryptoTransactionReceipt;
use radix_engine::transaction::VersionedTransactionReceipt as ScryptoVersionedTransactionReceipt;
use radix_engine_common::data::scrypto::{scrypto_decode, scrypto_encode};
use sbor::HasLatestVersion;

#[derive(Clone, Debug)]
pub struct TransactionReceipt {
    pub(crate) decoded: ScryptoTransactionReceipt,
    pub(crate) encoded: BagOfBytes,
}
impl PartialEq for TransactionReceipt {
    fn eq(&self, other: &Self) -> bool {
        self.encoded == other.encoded
    }
}
impl Eq for TransactionReceipt {}

impl TryFrom<BagOfBytes> for TransactionReceipt {
    type Error = crate::CommonError;

    fn try_from(encoded: BagOfBytes) -> Result<Self, Self::Error> {
        scrypto_decode::<ScryptoVersionedTransactionReceipt>(&encoded)
            .map(|r| r.into_latest())
            .map_err(|e| {
                error!("Failed to decode encoded receipt, {:?}", e);
                CommonError::FailedToDecodeEncodedReceipt
            })
            .map(|decoded| Self { decoded, encoded })
    }
}

impl HasSampleValues for TransactionReceipt {
    fn sample() -> Self {
        // from first to second account in profile
        /*
              {
          "appPreferences" : {
            "display" : {
              "fiatCurrencyPriceTarget" : "usd",
              "isCurrencyAmountVisible" : true
            },
            "gateways" : {
              "current" : "https://babylon-stokenet-gateway.radixdlt.com/",
              "saved" : [
                {
                  "network" : {
                    "displayDescription" : "Stokenet",
                    "id" : 2,
                    "name" : "stokenet"
                  },
                  "url" : "https://babylon-stokenet-gateway.radixdlt.com/"
                },
                {
                  "network" : {
                    "displayDescription" : "Mainnet",
                    "id" : 1,
                    "name" : "mainnet"
                  },
                  "url" : "https://mainnet.radixdlt.com/"
                }
              ]
            },
            "p2pLinks" : [

            ],
            "security" : {
              "isCloudProfileSyncEnabled" : true,
              "isDeveloperModeEnabled" : false,
              "structureConfigurationReferences" : [

              ]
            },
            "transaction" : {
              "defaultDepositGuarantee" : "1"
            }
          },
          "factorSources" : [
            {
              "device" : {
                "common" : {
                  "addedOn" : "2024-02-29T13:22:55Z",
                  "cryptoParameters" : {
                    "supportedCurves" : [
                      "curve25519"
                    ],
                    "supportedDerivationPathSchemes" : [
                      "cap26"
                    ]
                  },
                  "flags" : [
                    "main"
                  ],
                  "lastUsedOn" : "2024-02-29T13:22:55Z"
                },
                "hint" : {
                  "mnemonicWordCount" : 24,
                  "model" : "iPhone",
                  "name" : "iPhone"
                },
                "id" : {
                  "body" : "be308327db97b638d33c80beea593cc1ea2891d9f004cbaa86b55c7a2b02ba91",
                  "kind" : "device"
                }
              },
              "discriminator" : "device"
            }
          ],
          "header" : {
            "contentHint" : {
              "numberOfAccountsOnAllNetworksInTotal" : 3,
              "numberOfNetworks" : 2,
              "numberOfPersonasOnAllNetworksInTotal" : 0
            },
            "creatingDevice" : {
              "date" : "2024-02-29T13:22:55Z",
              "description" : "iPhone (iPhone 15 Pro Max)",
              "id" : "79BE3C56-C51A-4801-A83F-51C24F378956"
            },
            "id" : "927A091E-77A7-469E-82DB-72355A4A81A1",
            "lastModified" : "2024-02-29T13:28:57Z",
            "lastUsedOnDevice" : {
              "date" : "2024-02-29T13:22:55Z",
              "description" : "iPhone (iPhone 15 Pro Max)",
              "id" : "79BE3C56-C51A-4801-A83F-51C24F378956"
            },
            "snapshotVersion" : 100
          },
          "networks" : [
            {
              "accounts" : [
                {
                  "address" : "account_rdx12x782efxx6ecu0cmgq5sza85ddp6j6pykktk5jzfe3ajapczkssamg",
                  "appearanceID" : 0,
                  "displayName" : "Main0",
                  "flags" : [

                  ],
                  "networkID" : 1,
                  "onLedgerSettings" : {
                    "thirdPartyDeposits" : {
                      "assetsExceptionList" : [

                      ],
                      "depositorsAllowList" : [

                      ],
                      "depositRule" : "acceptAll"
                    }
                  },
                  "securityState" : {
                    "discriminator" : "unsecured",
                    "unsecuredEntityControl" : {
                      "transactionSigning" : {
                        "badge" : {
                          "discriminator" : "virtualSource",
                          "virtualSource" : {
                            "discriminator" : "hierarchicalDeterministicPublicKey",
                            "hierarchicalDeterministicPublicKey" : {
                              "derivationPath" : {
                                "path" : "m/44H/1022H/1H/525H/1460H/0H",
                                "scheme" : "cap26"
                              },
                              "publicKey" : {
                                "compressedData" : "7171a0a09e5cf4a2e8acdad424cf7ab98a1a833e16034373f86ba0c7a4f20057",
                                "curve" : "curve25519"
                              }
                            }
                          }
                        },
                        "factorSourceID" : {
                          "discriminator" : "fromHash",
                          "fromHash" : {
                            "body" : "be308327db97b638d33c80beea593cc1ea2891d9f004cbaa86b55c7a2b02ba91",
                            "kind" : "device"
                          }
                        }
                      }
                    }
                  }
                }
              ],
              "authorizedDapps" : [

              ],
              "networkID" : 1,
              "personas" : [

              ]
            },
            {
              "accounts" : [
                {
                  "address" : "account_tdx_2_128h2zv5m4mnprnfjxn4nf96pglgx064mut8np26hp7w9mm064es2dn",
                  "appearanceID" : 0,
                  "displayName" : "Stok0",
                  "flags" : [

                  ],
                  "networkID" : 2,
                  "onLedgerSettings" : {
                    "thirdPartyDeposits" : {
                      "assetsExceptionList" : [

                      ],
                      "depositorsAllowList" : [

                      ],
                      "depositRule" : "acceptAll"
                    }
                  },
                  "securityState" : {
                    "discriminator" : "unsecured",
                    "unsecuredEntityControl" : {
                      "transactionSigning" : {
                        "badge" : {
                          "discriminator" : "virtualSource",
                          "virtualSource" : {
                            "discriminator" : "hierarchicalDeterministicPublicKey",
                            "hierarchicalDeterministicPublicKey" : {
                              "derivationPath" : {
                                "path" : "m/44H/1022H/2H/525H/1460H/0H",
                                "scheme" : "cap26"
                              },
                              "publicKey" : {
                                "compressedData" : "6b5ce93949e6d01b36cf6a996e6f8fd250ba5fca40454e8228e4461643f06b98",
                                "curve" : "curve25519"
                              }
                            }
                          }
                        },
                        "factorSourceID" : {
                          "discriminator" : "fromHash",
                          "fromHash" : {
                            "body" : "be308327db97b638d33c80beea593cc1ea2891d9f004cbaa86b55c7a2b02ba91",
                            "kind" : "device"
                          }
                        }
                      }
                    }
                  }
                },
                {
                  "address" : "account_tdx_2_128x8q5es2dstqtcc8wqm843xdtfs0lgetfcdn62a54wxspj6yhpxkf",
                  "appearanceID" : 1,
                  "displayName" : "Stok1",
                  "flags" : [

                  ],
                  "networkID" : 2,
                  "onLedgerSettings" : {
                    "thirdPartyDeposits" : {
                      "assetsExceptionList" : [

                      ],
                      "depositorsAllowList" : [

                      ],
                      "depositRule" : "acceptAll"
                    }
                  },
                  "securityState" : {
                    "discriminator" : "unsecured",
                    "unsecuredEntityControl" : {
                      "transactionSigning" : {
                        "badge" : {
                          "discriminator" : "virtualSource",
                          "virtualSource" : {
                            "discriminator" : "hierarchicalDeterministicPublicKey",
                            "hierarchicalDeterministicPublicKey" : {
                              "derivationPath" : {
                                "path" : "m/44H/1022H/2H/525H/1460H/1H",
                                "scheme" : "cap26"
                              },
                              "publicKey" : {
                                "compressedData" : "4c15c5270231d7fd8d115813387aa8a072209d0efa431f595dd380c88cc7dd35",
                                "curve" : "curve25519"
                              }
                            }
                          }
                        },
                        "factorSourceID" : {
                          "discriminator" : "fromHash",
                          "fromHash" : {
                            "body" : "be308327db97b638d33c80beea593cc1ea2891d9f004cbaa86b55c7a2b02ba91",
                            "kind" : "device"
                          }
                        }
                      }
                    }
                  }
                }
              ],
              "authorizedDapps" : [

              ],
              "networkID" : 2,
              "personas" : [

              ]
            }
          ]
        }
              */
        // mnemonic: first note banner manual catch then plunge task select chest dove august rule inflict fox dash seek rib episode wage six blossom change infant
        /*
              🔮 manifestToSign: ~~~ MANIFEST ~~~
        CALL_METHOD
            Address("account_tdx_2_128h2zv5m4mnprnfjxn4nf96pglgx064mut8np26hp7w9mm064es2dn")
            "withdraw"
            Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
            Decimal("123");

        TAKE_FROM_WORKTOP
            Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
            Decimal("123")
            Bucket("bucket1");

        CALL_METHOD
            Address("account_tdx_2_128x8q5es2dstqtcc8wqm843xdtfs0lgetfcdn62a54wxspj6yhpxkf")
            "try_deposit_or_abort"
            Bucket("bucket1")
            Enum<0u8>()
          ;

        🔮 transactionPreviewRequest: TransactionPreviewRequest(manifest: "CALL_METHOD\n    Address(\"account_tdx_2_128h2zv5m4mnprnfjxn4nf96pglgx064mut8np26hp7w9mm064es2dn\")\n    \"withdraw\"\n    Address(\"resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc\")\n    Decimal(\"123\")\n;\nTAKE_FROM_WORKTOP\n    Address(\"resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc\")\n    Decimal(\"123\")\n    Bucket(\"bucket1\")\n;\nCALL_METHOD\n    Address(\"account_tdx_2_128x8q5es2dstqtcc8wqm843xdtfs0lgetfcdn62a54wxspj6yhpxkf\")\n    \"try_deposit_or_abort\"\n    Bucket(\"bucket1\")\n    Enum<0u8>()\n;\n", blobsHex: Optional([]), startEpochInclusive: 39905, endEpochExclusive: 39915, notaryPublicKey: Optional(Radix_Wallet_Dev.GatewayAPI.PublicKey.eddsaEd25519(Radix_Wallet_Dev.GatewayAPI.PublicKeyEddsaEd25519(keyType: Radix_Wallet_Dev.GatewayAPI.PublicKeyType.eddsaEd25519, keyHex: "2c3ad79010325e7963ae1a80d8cc5913588ced223abbbf9a90ad69ca966db13c"))), notaryIsSignatory: Optional(false), tipPercentage: 0, nonce: 3769943665, signerPublicKeys: [Radix_Wallet_Dev.GatewayAPI.PublicKey.eddsaEd25519(Radix_Wallet_Dev.GatewayAPI.PublicKeyEddsaEd25519(keyType: Radix_Wallet_Dev.GatewayAPI.PublicKeyType.eddsaEd25519, keyHex: "6b5ce93949e6d01b36cf6a996e6f8fd250ba5fca40454e8228e4461643f06b98"))], flags: Radix_Wallet_Dev.GatewayAPI.TransactionPreviewRequestFlags(useFreeCredit: true, assumeAllSignatureProofs: false, skipEpochCheck: false))
        🔮 encodedReceipt: 5c22..<SEE BELOW>
        🔮 analyzedManifestToReview: ExecutionSummary(accountWithdraws: ["account_tdx_2_128h2zv5m4mnprnfjxn4nf96pglgx064mut8np26hp7w9mm064es2dn": [EngineToolkit.ResourceIndicator.fungible(resourceAddress: EngineToolkit.Address, indicator: EngineToolkit.FungibleResourceIndicator.guaranteed(amount: EngineToolkit.Decimal))]], accountDeposits: ["account_tdx_2_128x8q5es2dstqtcc8wqm843xdtfs0lgetfcdn62a54wxspj6yhpxkf": [EngineToolkit.ResourceIndicator.fungible(resourceAddress: EngineToolkit.Address, indicator: EngineToolkit.FungibleResourceIndicator.guaranteed(amount: EngineToolkit.Decimal))]], presentedProofs: [], newEntities: EngineToolkit.NewEntities(componentAddresses: [EngineToolkit.Address], resourceAddresses: [], packageAddresses: [], metadata: ["account_tdx_2_128x8q5es2dstqtcc8wqm843xdtfs0lgetfcdn62a54wxspj6yhpxkf": ["owner_keys": Optional(EngineToolkit.MetadataValue.publicKeyHashArrayValue(value: [EngineToolkit.PublicKeyHash.ed25519(value: 29 bytes)])), "owner_badge": Optional(EngineToolkit.MetadataValue.nonFungibleLocalIdValue(value: EngineToolkit.NonFungibleLocalId.bytes(value: 30 bytes)))]]), encounteredEntities: [EngineToolkit.Address, EngineToolkit.Address, EngineToolkit.Address], accountsRequiringAuth: [EngineToolkit.Address], identitiesRequiringAuth: [], reservedInstructions: [], feeLocks: EngineToolkit.FeeLocks(lock: EngineToolkit.Decimal, contingentLock: EngineToolkit.Decimal), feeSummary: EngineToolkit.FeeSummary(executionCost: EngineToolkit.Decimal, finalizationCost: EngineToolkit.Decimal, storageExpansionCost: EngineToolkit.Decimal, royaltyCost: EngineToolkit.Decimal), detailedClassification: [EngineToolkit.DetailedManifestClass.transfer(isOneToOne: true), EngineToolkit.DetailedManifestClass.general], newlyCreatedNonFungibles: [])
              */
        let bag_of_bytes = BagOfBytes::from_hex("5c22000121062108a000743ba40b000000000000000000000000000000000000000900e1f5050900093d00a000743ba40b000000000000000000000000000000000000000980f0fa02a0aaaa829007e54be700000000000000000000000000000000a080cdc975bc56000000000000000000000000000000000000a080cdc975bc560000000000000000000000000000000000002102080000a0000000a1edccce1bc2d30000000000000000000000000000210709988e3b0009bfa40f00a000e0a439b655b50200000000000000000000000000000000a0008cab6a611db60000000000000000000000000000000000a0000000000000000000000000000000000000000000000000a080fbb8bb9095500200000000000000000000000000000000a00000000000000000000000000000000000000000000000002201012102230c09220b4166746572496e766f6b656e0200000e416c6c6f636174654e6f64654964560800000c4265666f7265496e766f6b65320a00000d436c6f73655375627374617465209100000a4372656174654e6f64650e4b00000844726f704e6f6465157a000009456d69744576656e746c0b0000174d61726b537562737461746541735472616e7369656e74370000000a4d6f76654d6f64756c65780500002b4f70656e53756273746174653a3a476c6f62616c46756e6769626c655265736f757263654d616e61676572a7a502002e4f70656e53756273746174653a3a476c6f62616c4e6f6e46756e6769626c655265736f757263654d616e61676572bea600001b4f70656e53756273746174653a3a476c6f62616c5061636b616765430e1f00294f70656e53756273746174653a3a476c6f62616c5669727475616c456432353531394163636f756e7437c80b00234f70656e53756273746174653a3a496e7465726e616c46756e6769626c655661756c744c650100264f70656e53756273746174653a3a496e7465726e616c47656e65726963436f6d706f6e656e7496f000000750696e4e6f6465f00000000a51756572794163746f72dc0500000c526561645375627374617465665102001b52756e4e6174697665436f64653a3a576f726b746f705f64726f70fe4500001a52756e4e6174697665436f64653a3a576f726b746f705f707574697100001b52756e4e6174697665436f64653a3a576f726b746f705f74616b652e4600001552756e4e6174697665436f64653a3a637265617465106000003952756e4e6174697665436f64653a3a6372656174655f656d7074795f7661756c745f46756e6769626c655265736f757263654d616e61676572f28a00001f52756e4e6174697665436f64653a3a6372656174655f776974685f646174614f6b00002852756e4e6174697665436f64653a3a6765745f616d6f756e745f46756e6769626c654275636b657420ac00001c52756e4e6174697665436f64653a3a6f6e5f7669727475616c697a65d88600002052756e4e6174697665436f64653a3a7075745f46756e6769626c655661756c74ea5f00002152756e4e6174697665436f64653a3a74616b655f46756e6769626c655661756c74d9a500002352756e4e6174697665436f64653a3a7472795f6465706f7369745f6f725f61626f7274c47e01001752756e4e6174697665436f64653a3a7769746864726177fbe100000b5365745375627374617465730100001156616c696461746554785061796c6f6164203000001256657269667954785369676e617475726573000000000d577269746553756273746174655e220000230c09040c436f6d6d69744576656e7473dd6100000a436f6d6d69744c6f6773000000002f436f6d6d69745374617465557064617465733a3a476c6f62616c5669727475616c456432353531394163636f756e74d3ae0a0029436f6d6d69745374617465557064617465733a3a496e7465726e616c46756e6769626c655661756c740f940400220001210921012320220e071e860c6318c6318c6c4e1b40cc6318c6318cf7bca52eb54a6a86318c6318c60001230722014000012322220100010702000120074a5c220001210222000121022307a00100e05902182a026f01000000000000000000000000000000009058619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed7220000071e82cc6318c6318c659963ed8c6318c6318cf7e8f5ae8f4a96a6318c6318c6000123072202400001232222010001070000012007265c220001210222000121050add9b0000000000000752074107ff0a64000000000000002200005200012322220101012007245c20072087575b7dde8de219e3c7cd76433b95b8718051f81df61f0d22f3eb1b01a2f7f200012007125c2200012102220101220001220000220000071e0d906318c6318c659a6130cc6318c6318cf7a8ba5295eabf46318c6318c6000123072206440001232222004200012322220041000123222200010001232222004500012322220046000123222200071e51eea1329baee611cd3234eb34974147d067eabbe2cf30ab570f9c5dedfa00012307220400000123222200060001232222000500012322220041000123222200071e0d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c60001230722084400012322220000000123222200430001232222004200012322220041000123222200010001232222004500012322220046000123222200071e5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6000123072203000001232222000600012322220040000123222200071e58c798466a205d751ecdef0bff807ca2a2685098e68d7dd7e545946a084900012307220200000123222200400001232222010001070000012007255c2200012102220001a0000034456901e96e17020000000000000000000000000000220000071e0d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c600012307220744000123222200000001232222004200012322220041000123222200010001232222004500012322220046000123222200071e51cc7053305360b02f183b81b3d6266ad307fd195a70d9e95da55c68065a00012307220702000123222202010120070e5c0c0b6f776e65725f626164676500012007335c2200012102220101220001220b01c0021e51cc7053305360b02f183b81b3d6266ad307fd195a70d9e95da55c68065a220100010120070d5c0c0a6f776e65725f6b65797300012007375c2200012102220101220001228f01202201010120071dcc7053305360b02f183b81b3d6266ad307fd195a70d9e95da55c68065a2200000500012322220100010700000120075e5c220001210222000121022202012200012200012200012102809a4c6318c6318c6cb554820c6318c6318cf7a951d7a9e547c6318c6318c6c0021dcc7053305360b02f183b81b3d6266ad307fd195a70d9e95da55c68065a2202002200000600012322220101012007105c21022200000c087365637572696679000120075c5c22000121022201012200012202012200012200012200012102809a4c6318c6318c6cb554820c6318c6318cf7a951d7a9e547c6318c6318c6c0021dcc7053305360b02f183b81b3d6266ad307fd195a70d9e95da55c68065a220000400001232222010001070000012007115c22000121022200012101220000220000000001232222010001070000012007775c220001210221052102800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c60c074163636f756e742103090100000009000000000900000000220100200c0020220022000123222102030003090100000009000000000900000000010003090100000009000000000900000000420001232222004100012322220101012007205c805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6000120072e5c22000121022201012200019058c607945309c72a1758af94c389f1a3b92f74dca4610a1edfe664cc8e9f220000071e0d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c600012307220744000123222200000001232222004200012322220041000123222200010001232222004500012322220046000123222200071e9a4c6318c6318c6cb554820c6318c6318cf7a951d7a9e547c6318c6318c600012307220100000123222200071e0d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c600012307220744000123222200000001232222004200012322220041000123222200010001232222004500012322220046000123222200071e58c607945309c72a1758af94c389f1a3b92f74dca4610a1edfe664cc8e9f000123072203000001232222010001070000012007745c220001210221052102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c0d46756e6769626c655661756c742103090100000009000000000900000000220001805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6200c0020220022010001000123222200400001232222010001070000012007255c2200012102220001a000000c6d51c8f7aa06000000000000000000000000000000220000071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed7000123072201400001232222010001070000012007255c2200012102220001a0365b006b5404de0200000000000000000000000000000000220000210520800020800151cc7053305360b02f183b81b3d6266ad307fd195a70d9e95da55c68065a20800020800158c607945309c72a1758af94c389f1a3b92f74dca4610a1edfe664cc8e9f23202103071e58c798466a205d751ecdef0bff807ca2a2685098e68d7dd7e545946a084902805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6220001a00000f492ae370855f9ffffffffffffffffffffffffffffff071e58c607945309c72a1758af94c389f1a3b92f74dca4610a1edfe664cc8e9f02805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6220001a000000c6d51c8f7aa06000000000000000000000000000000071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed702805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6220001a0c0b304305404de020000000000000000000000000000000021012320a0002104a0e05902182a026f0100000000000000000000000000000000a0e05902182a026f0100000000000000000000000000000000a0c0b304305404de02000000000000000000000000000000002322a00022000120220300012007205c90f8f9f2823a45facc433974ca19774a2efcdc50feba01d6782f9790ffa234010000012007035c210020210702210222010220071e58c798466a205d751ecdef0bff807ca2a2685098e68d7dd7e545946a08492200000c0d57697468647261774576656e7420071c5c2101a000000c6d51c8f7aa0600000000000000000000000000000002210222010220071e51eea1329baee611cd3234eb34974147d067eabbe2cf30ab570f9c5dedfa2200000c0d57697468647261774576656e7420073c5c220002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a000000c6d51c8f7aa0600000000000000000000000000000002210222010220071e5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c62200000c125661756c744372656174696f6e4576656e742007245c210120071e58c607945309c72a1758af94c389f1a3b92f74dca4610a1edfe664cc8e9f02210222010220071e58c607945309c72a1758af94c389f1a3b92f74dca4610a1edfe664cc8e9f2200000c0c4465706f7369744576656e7420071c5c2101a000000c6d51c8f7aa0600000000000000000000000000000002210222010220071e51cc7053305360b02f183b81b3d6266ad307fd195a70d9e95da55c68065a2200000c0c4465706f7369744576656e7420073c5c220002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a000000c6d51c8f7aa0600000000000000000000000000000002210222010220071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed72200000c0c4465706f7369744576656e7420071c5c2101a0c0b304305404de020000000000000000000000000000000002210222010220071e5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c62200000c194275726e46756e6769626c655265736f757263654576656e7420071c5c2101a0c0b304305404de0200000000000000000000000000000000202100210223202306071e860c6318c6318c6c4e1b40cc6318c6318cf7bca52eb54a6a86318c6318c607230140222201000107020301210122000121012103800d906318c6318c6c4e1b40cc6318c6318cf7bfd5d45f48c686318c6318c6200720d8510877df1d820f4752b3c033baf656f62e0e612731718865d048b9d16300b32201010a0900000000000000071e82cc6318c6318c659963ed8c6318c6318cf7e8f5ae8f4a96a6318c6318c607230240222201000107000301210122000121012103800d906318c6318c659963ed8c6318c6318cf7be85a17d48bca6318c6318c6200720bd71c021e525c608eaf7291c8c0eb2519993241a8e8d6d58c62e3ae0565355922201010a03000000000000005222220101012007245c20072087575b7dde8de219e3c7cd76433b95b8718051f81df61f0d22f3eb1b01a2f7f20401210222000121012103800d906318c6318c659963ed8c6318c6318cf7be85a17d48bca6318c6318c6200720bd71c021e525c608eaf7291c8c0eb2519993241a8e8d6d58c62e3ae0565355922201010a000000000000000022000121012103800d906318c6318c659963ed8c6318c6318cf7be85a17d48bca6318c6318c6200720bd71c021e525c608eaf7291c8c0eb2519993241a8e8d6d58c62e3ae0565355922201010a0100000000000000071e58c798466a205d751ecdef0bff807ca2a2685098e68d7dd7e545946a084907230140222201000107000301210122000121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a0000000000000000071e51cc7053305360b02f183b81b3d6266ad307fd195a70d9e95da55c68065a07230602222202010120070e5c0c0b6f776e65725f62616467650401210222000121012103800d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c620072007bfe5891cd05394fa30c6a67fab9208642f39665ca903f9405aff6b6fefb36a220001070c22000121012103800d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c620072007bfe5891cd05394fa30c6a67fab9208642f39665ca903f9405aff6b6fefb36a2201010a0000000000000000010120070d5c0c0a6f776e65725f6b6579730401210222000121012103800d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c620072007bfe5891cd05394fa30c6a67fab9208642f39665ca903f9405aff6b6fefb36a220001070c22000121012103800d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c620072007bfe5891cd05394fa30c6a67fab9208642f39665ca903f9405aff6b6fefb36a2201010a000000000000000005222201000107000301210122000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a00000000000000000622220101012007105c21022200000c0873656375726966790401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a040000000000000040222201000107000301210122000121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a2201010a00000000000000000022220100010700000121012200004122220101012007205c805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c60401210222000121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a220001078522000121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a2201010a0300000000000000071e58c607945309c72a1758af94c389f1a3b92f74dca4610a1edfe664cc8e9f07230200222201000107000001210122000040222201000107000301210122000121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a0000000000000000071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed707230140222201000107000301210122000121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a0000000000000000232121070222010220071e58c798466a205d751ecdef0bff807ca2a2685098e68d7dd7e545946a08492200000c0d57697468647261774576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a18000000000000000222010220071e51eea1329baee611cd3234eb34974147d067eabbe2cf30ab570f9c5dedfa2200000c0d57697468647261774576656e740121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a2201010a27000000000000000222010220071e5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c62200000c125661756c744372656174696f6e4576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720ba27cc155884d6e1aa7a41346fd8c11f18cc99775653caef1fd3455d625fd1472201010a37000000000000000222010220071e58c607945309c72a1758af94c389f1a3b92f74dca4610a1edfe664cc8e9f2200000c0c4465706f7369744576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a19000000000000000222010220071e51cc7053305360b02f183b81b3d6266ad307fd195a70d9e95da55c68065a2200000c0c4465706f7369744576656e740121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a2201010a28000000000000000222010220071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed72200000c0c4465706f7369744576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a19000000000000000222010220071e5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c62200000c194275726e46756e6769626c655265736f757263654576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720ba27cc155884d6e1aa7a41346fd8c11f18cc99775653caef1fd3455d625fd1472201010a3a0000000000000022010121032021010822000121022102800d906318c6318c659a6130cc6318c6318cf7a8ba5295eabf46318c6318c60c145472616e73616374696f6e50726f636573736f720c0372756e0a00000000000000002201000a01000000000000000a000000000000000021022320220023202200210223202200232022002021040822010121022102800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c60c074163636f756e740c0877697468647261770a010000000000000022000120071e51eea1329baee611cd3234eb34974147d067eabbe2cf30ab570f9c5dedfa0a02000000000000000a000000000000000021022320220023202200210223202201071ef8f9f2823a45facc433974ca19774a2efcdc50feba01d6782f9790ffa2340002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a000000c6d51c8f7aa06000000000000000000000000000000232022002021010822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c0d46756e6769626c655661756c740c0474616b650a020000000000000022000120071e58c798466a205d751ecdef0bff807ca2a2685098e68d7dd7e545946a08490a03000000000000000a000000000000000021022320220023202200210223202201071ef8f9f2823a45facc433974ca19774a2efcdc50feba01d6782f9790ffa2340002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a000000c6d51c8f7aa0600000000000000000000000000000023202200202101082202000a030000000000000022000120071e58c798466a205d751ecdef0bff807ca2a2685098e68d7dd7e545946a08490a03000000000000000a000000000000000021022320220023202200210223202201071ef8f9f2823a45facc433974ca19774a2efcdc50feba01d6782f9790ffa2340002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a000000c6d51c8f7aa06000000000000000000000000000000232022002021000822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c07576f726b746f700c0b576f726b746f705f7075740a010000000000000022000120071ef8cd150618d8f38f8fe8c4b0a235c6aac5edb452215ec5de81fc268b75e20a02000000000000000a0000000000000000210223202201071ef8f9f2823a45facc433974ca19774a2efcdc50feba01d6782f9790ffa2340002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a000000c6d51c8f7aa0600000000000000000000000000000023202200210223202200232022002021000822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c07576f726b746f700c0c576f726b746f705f74616b650a010000000000000022000120071ef8cd150618d8f38f8fe8c4b0a235c6aac5edb452215ec5de81fc268b75e20a02000000000000000a010000000000000021022320220023202200210223202201071ef8f9f2823a45facc433974ca19774a2efcdc50feba01d6782f9790ffa2340002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a000000c6d51c8f7aa06000000000000000000000000000000232022002021000822010121022102800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c60c074163636f756e740c147472795f6465706f7369745f6f725f61626f72740a010000000000000022000120071e51cc7053305360b02f183b81b3d6266ad307fd195a70d9e95da55c68065a0a02000000000000000a0200000000000000210223202201071ef8f9f2823a45facc433974ca19774a2efcdc50feba01d6782f9790ffa2340002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a000000c6d51c8f7aa0600000000000000000000000000000023202200210223202200232022002021010822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c0d46756e6769626c655661756c740c037075740a020000000000000022000120071e58c607945309c72a1758af94c389f1a3b92f74dca4610a1edfe664cc8e9f0a03000000000000000a0200000000000000210223202201071ef8f9f2823a45facc433974ca19774a2efcdc50feba01d6782f9790ffa2340002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a000000c6d51c8f7aa060000000000000000000000000000002320220021022320220023202200202101082203000a030000000000000022000120071e58c607945309c72a1758af94c389f1a3b92f74dca4610a1edfe664cc8e9f0a03000000000000000a0200000000000000210223202201071ef8f9f2823a45facc433974ca19774a2efcdc50feba01d6782f9790ffa2340002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a000000c6d51c8f7aa060000000000000000000000000000002320220021022320220023202200202100230a2002000000000000000021010420071e51eea1329baee611cd3234eb34974147d067eabbe2cf30ab570f9c5dedfa20071e58c798466a205d751ecdef0bff807ca2a2685098e68d7dd7e545946a0849805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a00000f492ae370855f9ffffffffffffffffffffffffffffff020000000000000021010420071e51cc7053305360b02f183b81b3d6266ad307fd195a70d9e95da55c68065a20071e58c607945309c72a1758af94c389f1a3b92f74dca4610a1edfe664cc8e9f805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a000000c6d51c8f7aa060000000000000000000000000000002102a0000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000220000").unwrap();
        // successfully made TX: https://stokenet-dashboard.radixdlt.com/transaction/txid_tdx_2_15ufqjufp2lz64hkpg6lxxw45xmpv22jj30udd5putumgvgrxglqsyt7ge3/summary
        bag_of_bytes.try_into().unwrap()
    }
    fn sample_other() -> Self {
        /*
               🔮 manifestToSign: ~~~ MANIFEST ~~~
        CALL_METHOD

            Address("account_tdx_2_128h2zv5m4mnprnfjxn4nf96pglgx064mut8np26hp7w9mm064es2dn")

            "withdraw"

            Address("resource_tdx_2_1t5yynnmzvep4nctzjztem2ude9ghymwnmjmvklw3t35yn932vwa0cr")

            Decimal("23.123456789");

        TAKE_FROM_WORKTOP

            Address("resource_tdx_2_1t5yynnmzvep4nctzjztem2ude9ghymwnmjmvklw3t35yn932vwa0cr")

            Decimal("23.123456789")

            Bucket("bucket1");

        CALL_METHOD

            Address("account_tdx_2_128x8q5es2dstqtcc8wqm843xdtfs0lgetfcdn62a54wxspj6yhpxkf")

            "try_deposit_or_abort"

            Bucket("bucket1")

            Enum<0u8>();

        CALL_METHOD

            Address("account_tdx_2_128h2zv5m4mnprnfjxn4nf96pglgx064mut8np26hp7w9mm064es2dn")

            "withdraw"

            Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")

            Decimal("9874.6543");

        TAKE_FROM_WORKTOP

            Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")

            Decimal("9874.6543")

            Bucket("bucket2");

        CALL_METHOD

            Address("account_tdx_2_12y7e89yryqeghr6aq4mc9j46x56c5c9awc0rrypgnnxz08dv39tszq")

            "try_deposit_or_abort"

            Bucket("bucket2")

            Enum<0u8>();
        🔮 transactionPreviewRequest: TransactionPreviewRequest(manifest: "CALL_METHOD\n    Address(\"account_tdx_2_128h2zv5m4mnprnfjxn4nf96pglgx064mut8np26hp7w9mm064es2dn\")\n    \"withdraw\"\n    Address(\"resource_tdx_2_1t5yynnmzvep4nctzjztem2ude9ghymwnmjmvklw3t35yn932vwa0cr\")\n    Decimal(\"23.123456789\")\n;\nTAKE_FROM_WORKTOP\n    Address(\"resource_tdx_2_1t5yynnmzvep4nctzjztem2ude9ghymwnmjmvklw3t35yn932vwa0cr\")\n    Decimal(\"23.123456789\")\n    Bucket(\"bucket1\")\n;\nCALL_METHOD\n    Address(\"account_tdx_2_128x8q5es2dstqtcc8wqm843xdtfs0lgetfcdn62a54wxspj6yhpxkf\")\n    \"try_deposit_or_abort\"\n    Bucket(\"bucket1\")\n    Enum<0u8>()\n;\nCALL_METHOD\n    Address(\"account_tdx_2_128h2zv5m4mnprnfjxn4nf96pglgx064mut8np26hp7w9mm064es2dn\")\n    \"withdraw\"\n    Address(\"resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc\")\n    Decimal(\"9874.6543\")\n;\nTAKE_FROM_WORKTOP\n    Address(\"resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc\")\n    Decimal(\"9874.6543\")\n    Bucket(\"bucket2\")\n;\nCALL_METHOD\n    Address(\"account_tdx_2_12y7e89yryqeghr6aq4mc9j46x56c5c9awc0rrypgnnxz08dv39tszq\")\n    \"try_deposit_or_abort\"\n    Bucket(\"bucket2\")\n    Enum<0u8>()\n;\n", blobsHex: Optional([]), startEpochInclusive: 39919, endEpochExclusive: 39929, notaryPublicKey: Optional(Radix_Wallet_Dev.GatewayAPI.PublicKey.eddsaEd25519(Radix_Wallet_Dev.GatewayAPI.PublicKeyEddsaEd25519(keyType: Radix_Wallet_Dev.GatewayAPI.PublicKeyType.eddsaEd25519, keyHex: "d8fad1b91e9c104dc16d71ed31b7798f9f1c534c029b1d5ff87da4dae0e06739"))), notaryIsSignatory: Optional(false), tipPercentage: 0, nonce: 3258044523, signerPublicKeys: [Radix_Wallet_Dev.GatewayAPI.PublicKey.eddsaEd25519(Radix_Wallet_Dev.GatewayAPI.PublicKeyEddsaEd25519(keyType: Radix_Wallet_Dev.GatewayAPI.PublicKeyType.eddsaEd25519, keyHex: "6b5ce93949e6d01b36cf6a996e6f8fd250ba5fca40454e8228e4461643f06b98"))], flags: Radix_Wallet_Dev.GatewayAPI.TransactionPreviewRequestFlags(useFreeCredit: true, assumeAllSignatureProofs: false, skipEpochCheck: false))
        🔮 encodedReceipt: SEE BELOW
        🔮 analyzedManifestToReview: ExecutionSummary(accountWithdraws: ["account_tdx_2_128h2zv5m4mnprnfjxn4nf96pglgx064mut8np26hp7w9mm064es2dn": [EngineToolkit.ResourceIndicator.fungible(resourceAddress: EngineToolkit.Address, indicator: EngineToolkit.FungibleResourceIndicator.guaranteed(amount: EngineToolkit.Decimal)), EngineToolkit.ResourceIndicator.fungible(resourceAddress: EngineToolkit.Address, indicator: EngineToolkit.FungibleResourceIndicator.guaranteed(amount: EngineToolkit.Decimal))]], accountDeposits: ["account_tdx_2_12y7e89yryqeghr6aq4mc9j46x56c5c9awc0rrypgnnxz08dv39tszq": [EngineToolkit.ResourceIndicator.fungible(resourceAddress: EngineToolkit.Address, indicator: EngineToolkit.FungibleResourceIndicator.guaranteed(amount: EngineToolkit.Decimal))], "account_tdx_2_128x8q5es2dstqtcc8wqm843xdtfs0lgetfcdn62a54wxspj6yhpxkf": [EngineToolkit.ResourceIndicator.fungible(resourceAddress: EngineToolkit.Address, indicator: EngineToolkit.FungibleResourceIndicator.guaranteed(amount: EngineToolkit.Decimal))]], presentedProofs: [], newEntities: EngineToolkit.NewEntities(componentAddresses: [EngineToolkit.Address], resourceAddresses: [], packageAddresses: [], metadata: ["account_tdx_2_12y7e89yryqeghr6aq4mc9j46x56c5c9awc0rrypgnnxz08dv39tszq": ["owner_keys": Optional(EngineToolkit.MetadataValue.publicKeyHashArrayValue(value: [EngineToolkit.PublicKeyHash.ed25519(value: 29 bytes)])), "owner_badge": Optional(EngineToolkit.MetadataValue.nonFungibleLocalIdValue(value: EngineToolkit.NonFungibleLocalId.bytes(value: 30 bytes)))]]), encounteredEntities: [EngineToolkit.Address, EngineToolkit.Address, EngineToolkit.Address, EngineToolkit.Address, EngineToolkit.Address], accountsRequiringAuth: [EngineToolkit.Address], identitiesRequiringAuth: [], reservedInstructions: [], feeLocks: EngineToolkit.FeeLocks(lock: EngineToolkit.Decimal, contingentLock: EngineToolkit.Decimal), feeSummary: EngineToolkit.FeeSummary(executionCost: EngineToolkit.Decimal, finalizationCost: EngineToolkit.Decimal, storageExpansionCost: EngineToolkit.Decimal, royaltyCost: EngineToolkit.Decimal), detailedClassification: [EngineToolkit.DetailedManifestClass.transfer(isOneToOne: false), EngineToolkit.DetailedManifestClass.general], newlyCreatedNonFungibles: [])
               */
        let bag_of_bytes = BagOfBytes::from_hex("5c22000121062108a000743ba40b000000000000000000000000000000000000000900e1f5050900093d00a000743ba40b000000000000000000000000000000000000000980f0fa02a0aaaa829007e54be700000000000000000000000000000000a080cdc975bc56000000000000000000000000000000000000a080cdc975bc560000000000000000000000000000000000002102080000a0000000a1edccce1bc2d300000000000000000000000000002107098a734e000956211600a0008828952a4b910300000000000000000000000000000000a000f8eccd30a1010100000000000000000000000000000000a0000000000000000000000000000000000000000000000000a000f11bfc5c72a60300000000000000000000000000000000a00000000000000000000000000000000000000000000000002201012102230c09220b4166746572496e766f6b65500400000e416c6c6f636174654e6f64654964430d00000c4265666f7265496e766f6b65c20f00000d436c6f73655375627374617465f8fd00000a4372656174654e6f6465767700000844726f704e6f6465b7c3000009456d69744576656e74d8160000174d61726b537562737461746541735472616e7369656e746e0000000a4d6f76654d6f64756c65780500002b4f70656e53756273746174653a3a476c6f62616c46756e6769626c655265736f757263654d616e616765727ab004002e4f70656e53756273746174653a3a476c6f62616c4e6f6e46756e6769626c655265736f757263654d616e61676572bea600001b4f70656e53756273746174653a3a476c6f62616c5061636b616765ab561f00294f70656e53756273746174653a3a476c6f62616c5669727475616c456432353531394163636f756e74d7ac1200234f70656e53756273746174653a3a496e7465726e616c46756e6769626c655661756c7498ca0200264f70656e53756273746174653a3a496e7465726e616c47656e65726963436f6d706f6e656e74e4b601000750696e4e6f6465800100000a51756572794163746f72b80b00000c526561645375627374617465a23d03001b52756e4e6174697665436f64653a3a576f726b746f705f64726f70fe4500001a52756e4e6174697665436f64653a3a576f726b746f705f707574d2e200001b52756e4e6174697665436f64653a3a576f726b746f705f74616b655c8c00001552756e4e6174697665436f64653a3a637265617465106000003952756e4e6174697665436f64653a3a6372656174655f656d7074795f7661756c745f46756e6769626c655265736f757263654d616e61676572e41501001f52756e4e6174697665436f64653a3a6372656174655f776974685f646174614f6b00002852756e4e6174697665436f64653a3a6765745f616d6f756e745f46756e6769626c654275636b6574405801001c52756e4e6174697665436f64653a3a6f6e5f7669727475616c697a65d88600002052756e4e6174697665436f64653a3a7075745f46756e6769626c655661756c74d4bf00002152756e4e6174697665436f64653a3a74616b655f46756e6769626c655661756c74b24b01002352756e4e6174697665436f64653a3a7472795f6465706f7369745f6f725f61626f727488fd02001752756e4e6174697665436f64653a3a7769746864726177f6c301000b5365745375627374617465730100001156616c696461746554785061796c6f6164c05300001256657269667954785369676e617475726573000000000d57726974655375627374617465843d0000230c09040c436f6d6d69744576656e7473bac300000a436f6d6d69744c6f6773000000002f436f6d6d69745374617465557064617465733a3a476c6f62616c5669727475616c456432353531394163636f756e747e350c0029436f6d6d69745374617465557064617465733a3a496e7465726e616c46756e6769626c655661756c741e2809002200012109210123202212071e860c6318c6318c6c4e1b40cc6318c6318cf7bca52eb54a6a86318c6318c6000123072201400001232222010001070200012007635c220001210222000121022307a002024090da7a08eeda030000000000000000000000000000000000405ccc17ae570e02000000000000000000000000000000009058619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed7220000071e82cc6318c6318c659963ed8c6318c6318cf7e8f5ae8f4a96a6318c6318c6000123072202400001232222010001070000012007265c220001210222000121050add9b0000000000000752074107ff0a64000000000000002200005200012322220101012007245c200720c928e9091653fed0658d95ab6421ba5e4e6d43cb21153fd19f61cfbe12968efe00012007125c2200012102220101220001220000220000071e5d0849cf62664359e16290979dab8dc951726dd3dcb6cb7dd15c6849962a000123072203000001232222000600012322220040000123222200071e0d906318c6318c659a6130cc6318c6318cf7a8ba5295eabf46318c6318c6000123072206440001232222004200012322220041000123222200010001232222004500012322220046000123222200071e51eea1329baee611cd3234eb34974147d067eabbe2cf30ab570f9c5dedfa00012307220400000123222200060001232222000500012322220041000123222200071e0d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c60001230722084400012322220000000123222200430001232222004200012322220041000123222200010001232222004500012322220046000123222200071e584494962f42bae08be335f84678f2a3d8cba29cef12dcee1f3d93a163f800012307220200000123222200400001232222010001070000012007255c2200012102220001a0006e0f47dbdbe0c12b150000000000000000000000000000220000071e0d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c600012307220744000123222200000001232222004200012322220041000123222200010001232222004500012322220046000123222200071e51cc7053305360b02f183b81b3d6266ad307fd195a70d9e95da55c68065a0001230722040000012322220042000123222200400001232222004100012322220101012007205c805d0849cf62664359e16290979dab8dc951726dd3dcb6cb7dd15c6849962a000120072e5c22000121022201012200019058580c11142754271e9591ce3bc6bb7290461c9158bc0647efa2fec8b022220000071e58580c11142754271e9591ce3bc6bb7290461c9158bc0647efa2fec8b022000123072203000001232222010001070000012007745c220001210221052102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c0d46756e6769626c655661756c742103090100000009000000000900000000220001805d0849cf62664359e16290979dab8dc951726dd3dcb6cb7dd15c6849962a200c0020220022010001000123222200400001232222010001070000012007255c2200012102220001a0009270af6f05e74001000000000000000000000000000000220000071e5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6000123072203000001232222000600012322220040000123222200071e58c798466a205d751ecdef0bff807ca2a2685098e68d7dd7e545946a084900012307220200000123222200400001232222010001070000012007255c2200012102220001a0003ecdbad277fb1100000000000000000000000000000000220000071e513d93948320328b8f5d057782caba35358a60bd761e3190289ccc279dac00012307220702000123222202010120070e5c0c0b6f776e65725f626164676500012007335c2200012102220101220001220b01c0021e513d93948320328b8f5d057782caba35358a60bd761e3190289ccc279dac220100010120070d5c0c0a6f776e65725f6b65797300012007375c2200012102220101220001228f01202201010120071d3d93948320328b8f5d057782caba35358a60bd761e3190289ccc279dac2200000500012322220100010700000120075e5c220001210222000121022202012200012200012200012102809a4c6318c6318c6cb554820c6318c6318cf7a951d7a9e547c6318c6318c6c0021d3d93948320328b8f5d057782caba35358a60bd761e3190289ccc279dac2202002200000600012322220101012007105c21022200000c087365637572696679000120075c5c22000121022201012200012202012200012200012200012102809a4c6318c6318c6cb554820c6318c6318cf7a951d7a9e547c6318c6318c6c0021d3d93948320328b8f5d057782caba35358a60bd761e3190289ccc279dac220000400001232222010001070000012007115c22000121022200012101220000220000000001232222010001070000012007775c220001210221052102800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c60c074163636f756e742103090100000009000000000900000000220100200c0020220022000123222102030003090100000009000000000900000000010003090100000009000000000900000000420001232222004100012322220101012007205c805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6000120072e5c22000121022201012200019058e0f6444cadfd8b3a581661657737fdab9fcd9b0ff133897a1bf47e74c4220000071e0d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c600012307220744000123222200000001232222004200012322220041000123222200010001232222004500012322220046000123222200071e9a4c6318c6318c6cb554820c6318c6318cf7a951d7a9e547c6318c6318c600012307220100000123222200071e0d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c600012307220744000123222200000001232222004200012322220041000123222200010001232222004500012322220046000123222200071e58e0f6444cadfd8b3a581661657737fdab9fcd9b0ff133897a1bf47e74c4000123072203000001232222010001070000012007745c220001210221052102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c0d46756e6769626c655661756c742103090100000009000000000900000000220001805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6200c0020220022010001000123222200400001232222010001070000012007255c2200012102220001a000c09173b0675b4e17020000000000000000000000000000220000071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed7000123072201400001232222010001070000012007255c2200012102220001a04b32f6f66d8bd20b000000000000000000000000000000002200002105208000208001513d93948320328b8f5d057782caba35358a60bd761e3190289ccc279dac20800020800258580c11142754271e9591ce3bc6bb7290461c9158bc0647efa2fec8b02258e0f6444cadfd8b3a581661657737fdab9fcd9b0ff133897a1bf47e74c423202105071e584494962f42bae08be335f84678f2a3d8cba29cef12dcee1f3d93a163f802805d0849cf62664359e16290979dab8dc951726dd3dcb6cb7dd15c6849962a220001a0006e8f5090fa18bffeffffffffffffffffffffffffffffff071e58580c11142754271e9591ce3bc6bb7290461c9158bc0647efa2fec8b02202805d0849cf62664359e16290979dab8dc951726dd3dcb6cb7dd15c6849962a220001a0009270af6f05e74001000000000000000000000000000000071e58c798466a205d751ecdef0bff807ca2a2685098e68d7dd7e545946a084902805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6220001a000406e8c4f98a4b1e8fdffffffffffffffffffffffffffff071e58e0f6444cadfd8b3a581661657737fdab9fcd9b0ff133897a1bf47e74c402805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6220001a000c09173b0675b4e17020000000000000000000000000000071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed702805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6220001a080b8982f5caf1c040000000000000000000000000000000021012320a0002104a0405ccc17ae570e0200000000000000000000000000000000a0405ccc17ae570e0200000000000000000000000000000000a080b8982f5caf1c04000000000000000000000000000000002322a00022000120220600012007205c90f895aa1a51620ea63f102230aeb166778ff1d6767705e4c9400ba2fdfc85010000012007035c210000012007205c90f8d51c8f850b507a1038deef356de76d4173818fdb559d7fb47156a855ed010000012007035c210020210c02210222010220071e584494962f42bae08be335f84678f2a3d8cba29cef12dcee1f3d93a163f82200000c0d57697468647261774576656e7420071c5c2101a0009270af6f05e7400100000000000000000000000000000002210222010220071e51eea1329baee611cd3234eb34974147d067eabbe2cf30ab570f9c5dedfa2200000c0d57697468647261774576656e7420073c5c220002805d0849cf62664359e16290979dab8dc951726dd3dcb6cb7dd15c6849962aa0009270af6f05e7400100000000000000000000000000000002210222010220071e5d0849cf62664359e16290979dab8dc951726dd3dcb6cb7dd15c6849962a2200000c125661756c744372656174696f6e4576656e742007245c210120071e58580c11142754271e9591ce3bc6bb7290461c9158bc0647efa2fec8b02202210222010220071e58580c11142754271e9591ce3bc6bb7290461c9158bc0647efa2fec8b0222200000c0c4465706f7369744576656e7420071c5c2101a0009270af6f05e7400100000000000000000000000000000002210222010220071e51cc7053305360b02f183b81b3d6266ad307fd195a70d9e95da55c68065a2200000c0c4465706f7369744576656e7420073c5c220002805d0849cf62664359e16290979dab8dc951726dd3dcb6cb7dd15c6849962aa0009270af6f05e7400100000000000000000000000000000002210222010220071e58c798466a205d751ecdef0bff807ca2a2685098e68d7dd7e545946a08492200000c0d57697468647261774576656e7420071c5c2101a000c09173b0675b4e1702000000000000000000000000000002210222010220071e51eea1329baee611cd3234eb34974147d067eabbe2cf30ab570f9c5dedfa2200000c0d57697468647261774576656e7420073c5c220002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a000c09173b0675b4e1702000000000000000000000000000002210222010220071e5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c62200000c125661756c744372656174696f6e4576656e742007245c210120071e58e0f6444cadfd8b3a581661657737fdab9fcd9b0ff133897a1bf47e74c402210222010220071e58e0f6444cadfd8b3a581661657737fdab9fcd9b0ff133897a1bf47e74c42200000c0c4465706f7369744576656e7420071c5c2101a000c09173b0675b4e1702000000000000000000000000000002210222010220071e513d93948320328b8f5d057782caba35358a60bd761e3190289ccc279dac2200000c0c4465706f7369744576656e7420073c5c220002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a000c09173b0675b4e1702000000000000000000000000000002210222010220071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed72200000c0c4465706f7369744576656e7420071c5c2101a080b8982f5caf1c040000000000000000000000000000000002210222010220071e5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c62200000c194275726e46756e6769626c655265736f757263654576656e7420071c5c2101a080b8982f5caf1c0400000000000000000000000000000000202100210223202309071e860c6318c6318c6c4e1b40cc6318c6318cf7bca52eb54a6a86318c6318c607230140222201000107020301210122000121012103800d906318c6318c6c4e1b40cc6318c6318cf7bfd5d45f48c686318c6318c6200720d8510877df1d820f4752b3c033baf656f62e0e612731718865d048b9d16300b32201010a0900000000000000071e82cc6318c6318c659963ed8c6318c6318cf7e8f5ae8f4a96a6318c6318c607230240222201000107000301210122000121012103800d906318c6318c659963ed8c6318c6318cf7be85a17d48bca6318c6318c6200720bd71c021e525c608eaf7291c8c0eb2519993241a8e8d6d58c62e3ae0565355922201010a03000000000000005222220101012007245c200720c928e9091653fed0658d95ab6421ba5e4e6d43cb21153fd19f61cfbe12968efe0401210222000121012103800d906318c6318c659963ed8c6318c6318cf7be85a17d48bca6318c6318c6200720bd71c021e525c608eaf7291c8c0eb2519993241a8e8d6d58c62e3ae0565355922201010a000000000000000022000121012103800d906318c6318c659963ed8c6318c6318cf7be85a17d48bca6318c6318c6200720bd71c021e525c608eaf7291c8c0eb2519993241a8e8d6d58c62e3ae0565355922201010a0100000000000000071e584494962f42bae08be335f84678f2a3d8cba29cef12dcee1f3d93a163f807230140222201000107000301210122000121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a0000000000000000071e51cc7053305360b02f183b81b3d6266ad307fd195a70d9e95da55c68065a0723014122220101012007205c805d0849cf62664359e16290979dab8dc951726dd3dcb6cb7dd15c6849962a0401210222000121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a220001078522000121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a2201010a0300000000000000071e58580c11142754271e9591ce3bc6bb7290461c9158bc0647efa2fec8b02207230200222201000107000001210122000040222201000107000301210122000121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a0000000000000000071e58c798466a205d751ecdef0bff807ca2a2685098e68d7dd7e545946a084907230140222201000107000301210122000121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a0000000000000000071e513d93948320328b8f5d057782caba35358a60bd761e3190289ccc279dac07230602222202010120070e5c0c0b6f776e65725f62616467650401210222000121012103800d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c620072007bfe5891cd05394fa30c6a67fab9208642f39665ca903f9405aff6b6fefb36a220001070c22000121012103800d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c620072007bfe5891cd05394fa30c6a67fab9208642f39665ca903f9405aff6b6fefb36a2201010a0000000000000000010120070d5c0c0a6f776e65725f6b6579730401210222000121012103800d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c620072007bfe5891cd05394fa30c6a67fab9208642f39665ca903f9405aff6b6fefb36a220001070c22000121012103800d906318c6318c6dadbd5f4c6318c6318cf7d155d53de568a6318c6318c620072007bfe5891cd05394fa30c6a67fab9208642f39665ca903f9405aff6b6fefb36a2201010a000000000000000005222201000107000301210122000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a00000000000000000622220101012007105c21022200000c0873656375726966790401210222000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a030000000000000022000121012103800d906318c6318c6e8f9fcc0c6318c6318cf7aa2fad74a29e26318c6318c6200720a06c16caa26e2fbc01ba2b9fe564a3f02d8f426c4580fcafebdff5464fefbde82201010a040000000000000040222201000107000301210122000121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a2201010a00000000000000000022220100010700000121012200004122220101012007205c805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c60401210222000121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a220001078522000121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a2201010a0300000000000000071e58e0f6444cadfd8b3a581661657737fdab9fcd9b0ff133897a1bf47e74c407230200222201000107000001210122000040222201000107000301210122000121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a0000000000000000071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed707230140222201000107000301210122000121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a00000000000000002321210b0222010220071e584494962f42bae08be335f84678f2a3d8cba29cef12dcee1f3d93a163f82200000c0d57697468647261774576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a18000000000000000222010220071e51eea1329baee611cd3234eb34974147d067eabbe2cf30ab570f9c5dedfa2200000c0d57697468647261774576656e740121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a2201010a27000000000000000222010220071e5d0849cf62664359e16290979dab8dc951726dd3dcb6cb7dd15c6849962a2200000c125661756c744372656174696f6e4576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720ba27cc155884d6e1aa7a41346fd8c11f18cc99775653caef1fd3455d625fd1472201010a37000000000000000222010220071e58580c11142754271e9591ce3bc6bb7290461c9158bc0647efa2fec8b0222200000c0c4465706f7369744576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a19000000000000000222010220071e51cc7053305360b02f183b81b3d6266ad307fd195a70d9e95da55c68065a2200000c0c4465706f7369744576656e740121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a2201010a28000000000000000222010220071e58c798466a205d751ecdef0bff807ca2a2685098e68d7dd7e545946a08492200000c0d57697468647261774576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a18000000000000000222010220071e5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c62200000c125661756c744372656174696f6e4576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720ba27cc155884d6e1aa7a41346fd8c11f18cc99775653caef1fd3455d625fd1472201010a37000000000000000222010220071e58e0f6444cadfd8b3a581661657737fdab9fcd9b0ff133897a1bf47e74c42200000c0c4465706f7369744576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a19000000000000000222010220071e513d93948320328b8f5d057782caba35358a60bd761e3190289ccc279dac2200000c0c4465706f7369744576656e740121012103800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c6200720a54510264dbd13e03ea7d6e3112d5f3a88c9bddae66b9569d5de381ba9447a8a2201010a28000000000000000222010220071e58619833de031de3aad69cad02a22656e083e307fb617b28e1b275bd7ed72200000c0c4465706f7369744576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720462a3fea283117aab2b01c297812bdc0fa9060b29eb5e68b847f361bc12019332201010a19000000000000000222010220071e5da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c62200000c194275726e46756e6769626c655265736f757263654576656e740121012103800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c6200720ba27cc155884d6e1aa7a41346fd8c11f18cc99775653caef1fd3455d625fd1472201010a3a0000000000000022010121032021010822000121022102800d906318c6318c659a6130cc6318c6318cf7a8ba5295eabf46318c6318c60c145472616e73616374696f6e50726f636573736f720c0372756e0a00000000000000002201000a01000000000000000a000000000000000021022320220023202200210223202200232022002021080822010121022102800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c60c074163636f756e740c0877697468647261770a010000000000000022000120071e51eea1329baee611cd3234eb34974147d067eabbe2cf30ab570f9c5dedfa0a02000000000000000a000000000000000021022320220023202200210223202201071ef895aa1a51620ea63f102230aeb166778ff1d6767705e4c9400ba2fdfc850002805d0849cf62664359e16290979dab8dc951726dd3dcb6cb7dd15c6849962aa0009270af6f05e74001000000000000000000000000000000232022002021010822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c0d46756e6769626c655661756c740c0474616b650a020000000000000022000120071e584494962f42bae08be335f84678f2a3d8cba29cef12dcee1f3d93a163f80a03000000000000000a000000000000000021022320220023202200210223202201071ef895aa1a51620ea63f102230aeb166778ff1d6767705e4c9400ba2fdfc850002805d0849cf62664359e16290979dab8dc951726dd3dcb6cb7dd15c6849962aa0009270af6f05e7400100000000000000000000000000000023202200202101082202000a030000000000000022000120071e584494962f42bae08be335f84678f2a3d8cba29cef12dcee1f3d93a163f80a03000000000000000a000000000000000021022320220023202200210223202201071ef895aa1a51620ea63f102230aeb166778ff1d6767705e4c9400ba2fdfc850002805d0849cf62664359e16290979dab8dc951726dd3dcb6cb7dd15c6849962aa0009270af6f05e74001000000000000000000000000000000232022002021000822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c07576f726b746f700c0b576f726b746f705f7075740a010000000000000022000120071ef8d24cf7d13aba53f7031c49853c70d9a2548f7146729f3752a44b4c11d10a02000000000000000a0000000000000000210223202201071ef895aa1a51620ea63f102230aeb166778ff1d6767705e4c9400ba2fdfc850002805d0849cf62664359e16290979dab8dc951726dd3dcb6cb7dd15c6849962aa0009270af6f05e7400100000000000000000000000000000023202200210223202200232022002021000822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c07576f726b746f700c0c576f726b746f705f74616b650a010000000000000022000120071ef8d24cf7d13aba53f7031c49853c70d9a2548f7146729f3752a44b4c11d10a02000000000000000a010000000000000021022320220023202200210223202201071ef895aa1a51620ea63f102230aeb166778ff1d6767705e4c9400ba2fdfc850002805d0849cf62664359e16290979dab8dc951726dd3dcb6cb7dd15c6849962aa0009270af6f05e74001000000000000000000000000000000232022002021000822010121022102800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c60c074163636f756e740c147472795f6465706f7369745f6f725f61626f72740a010000000000000022000120071e51cc7053305360b02f183b81b3d6266ad307fd195a70d9e95da55c68065a0a02000000000000000a0200000000000000210223202201071ef895aa1a51620ea63f102230aeb166778ff1d6767705e4c9400ba2fdfc850002805d0849cf62664359e16290979dab8dc951726dd3dcb6cb7dd15c6849962aa0009270af6f05e7400100000000000000000000000000000023202200210223202200232022002021010822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c0d46756e6769626c655661756c740c037075740a020000000000000022000120071e58580c11142754271e9591ce3bc6bb7290461c9158bc0647efa2fec8b0220a03000000000000000a0200000000000000210223202201071ef895aa1a51620ea63f102230aeb166778ff1d6767705e4c9400ba2fdfc850002805d0849cf62664359e16290979dab8dc951726dd3dcb6cb7dd15c6849962aa0009270af6f05e740010000000000000000000000000000002320220021022320220023202200202101082203000a030000000000000022000120071e58580c11142754271e9591ce3bc6bb7290461c9158bc0647efa2fec8b0220a03000000000000000a0200000000000000210223202201071ef895aa1a51620ea63f102230aeb166778ff1d6767705e4c9400ba2fdfc850002805d0849cf62664359e16290979dab8dc951726dd3dcb6cb7dd15c6849962aa0009270af6f05e7400100000000000000000000000000000023202200210223202200232022002021000822010121022102800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c60c074163636f756e740c0877697468647261770a010000000000000022000120071e51eea1329baee611cd3234eb34974147d067eabbe2cf30ab570f9c5dedfa0a02000000000000000a030000000000000021022320220023202200210223202201071ef8d51c8f850b507a1038deef356de76d4173818fdb559d7fb47156a855ed0002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a000c09173b0675b4e17020000000000000000000000000000232022002021010822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c0d46756e6769626c655661756c740c0474616b650a020000000000000022000120071e58c798466a205d751ecdef0bff807ca2a2685098e68d7dd7e545946a08490a03000000000000000a030000000000000021022320220023202200210223202201071ef8d51c8f850b507a1038deef356de76d4173818fdb559d7fb47156a855ed0002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a000c09173b0675b4e1702000000000000000000000000000023202200202101082202000a030000000000000022000120071e58c798466a205d751ecdef0bff807ca2a2685098e68d7dd7e545946a08490a03000000000000000a030000000000000021022320220023202200210223202201071ef8d51c8f850b507a1038deef356de76d4173818fdb559d7fb47156a855ed0002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a000c09173b0675b4e17020000000000000000000000000000232022002021000822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c07576f726b746f700c0b576f726b746f705f7075740a010000000000000022000120071ef8d24cf7d13aba53f7031c49853c70d9a2548f7146729f3752a44b4c11d10a02000000000000000a0300000000000000210223202201071ef8d51c8f850b507a1038deef356de76d4173818fdb559d7fb47156a855ed0002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a000c09173b0675b4e1702000000000000000000000000000023202200210223202200232022002021000822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c07576f726b746f700c0c576f726b746f705f74616b650a010000000000000022000120071ef8d24cf7d13aba53f7031c49853c70d9a2548f7146729f3752a44b4c11d10a02000000000000000a040000000000000021022320220023202200210223202201071ef8d51c8f850b507a1038deef356de76d4173818fdb559d7fb47156a855ed0002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a000c09173b0675b4e17020000000000000000000000000000232022002021000822010121022102800d906318c6318c6ee313598c6318c6318cf7bcaa2e954a9626318c6318c60c074163636f756e740c147472795f6465706f7369745f6f725f61626f72740a010000000000000022000120071e513d93948320328b8f5d057782caba35358a60bd761e3190289ccc279dac0a02000000000000000a0500000000000000210223202201071ef8d51c8f850b507a1038deef356de76d4173818fdb559d7fb47156a855ed0002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a000c09173b0675b4e1702000000000000000000000000000023202200210223202200232022002021010822010121022102800d906318c6318c61e603c64c6318c6318cf7be913d63aafbc6318c6318c60c0d46756e6769626c655661756c740c037075740a020000000000000022000120071e58e0f6444cadfd8b3a581661657737fdab9fcd9b0ff133897a1bf47e74c40a03000000000000000a0500000000000000210223202201071ef8d51c8f850b507a1038deef356de76d4173818fdb559d7fb47156a855ed0002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a000c09173b0675b4e170200000000000000000000000000002320220021022320220023202200202101082203000a030000000000000022000120071e58e0f6444cadfd8b3a581661657737fdab9fcd9b0ff133897a1bf47e74c40a03000000000000000a0500000000000000210223202201071ef8d51c8f850b507a1038deef356de76d4173818fdb559d7fb47156a855ed0002805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a000c09173b0675b4e170200000000000000000000000000002320220021022320220023202200202100230a2004000000000000000021010420071e51eea1329baee611cd3234eb34974147d067eabbe2cf30ab570f9c5dedfa20071e584494962f42bae08be335f84678f2a3d8cba29cef12dcee1f3d93a163f8805d0849cf62664359e16290979dab8dc951726dd3dcb6cb7dd15c6849962aa0006e8f5090fa18bffeffffffffffffffffffffffffffffff020000000000000021010420071e51cc7053305360b02f183b81b3d6266ad307fd195a70d9e95da55c68065a20071e58580c11142754271e9591ce3bc6bb7290461c9158bc0647efa2fec8b022805d0849cf62664359e16290979dab8dc951726dd3dcb6cb7dd15c6849962aa0009270af6f05e74001000000000000000000000000000000030000000000000021010420071e51eea1329baee611cd3234eb34974147d067eabbe2cf30ab570f9c5dedfa20071e58c798466a205d751ecdef0bff807ca2a2685098e68d7dd7e545946a0849805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a000406e8c4f98a4b1e8fdffffffffffffffffffffffffffff050000000000000021010420071e513d93948320328b8f5d057782caba35358a60bd761e3190289ccc279dac20071e58e0f6444cadfd8b3a581661657737fdab9fcd9b0ff133897a1bf47e74c4805da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6a000c09173b0675b4e170200000000000000000000000000002102a0000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000220000").unwrap();
        // successfully made: https://stokenet-dashboard.radixdlt.com/transaction/txid_tdx_2_1z2gaxex0gqtn75tun6ukek0x2su38hj44zpp0mwy4m4zrwvwkf0qmk0cms/summary
        bag_of_bytes.try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionReceipt;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
