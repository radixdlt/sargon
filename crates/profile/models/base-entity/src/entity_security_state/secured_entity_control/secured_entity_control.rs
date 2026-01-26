#[cfg(test)]
use transaction_models::SignatureWithPublicKey;

use crate::prelude::*;

/// Advanced security control of an entity which has been "securified",
/// meaning an MFA security structure (`SecurityStructureOfFactorSources`)
/// which user has created has been applied to it.
#[derive(
    Serialize, Deserialize, Clone, PartialEq, Eq, Hash, derive_more::Debug,
)]
#[serde(rename_all = "camelCase")]
pub struct SecuredEntityControl {
    /// Virtual Entity Creation (Factor)Instance
    ///
    /// Optional since if we recovered this SecuredEntityControl part of
    /// account recovery scan we might not know the veci
    pub veci: Option<HierarchicalDeterministicFactorInstance>,

    pub access_controller_address: AccessControllerAddress,

    /// The believed-to-be-current security structure of FactorInstances which
    /// secures this entity.
    pub security_structure: SecurityStructureOfFactorInstances,

    /// A provisional new security structure configuration which user
    /// is about to change to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisional_securified_config: Option<ProvisionalSecurifiedConfig>,
}

impl SecuredEntityControl {
    pub fn access_controller_address(&self) -> AccessControllerAddress {
        self.access_controller_address
    }

    pub fn commit_provisional(&mut self) -> Result<()> {
        let provisional = self
            .get_provisional()
            .ok_or(CommonError::EntityHasNoProvisionalSecurityConfigSet)?;
        self.security_structure =
            provisional.get_security_structure_of_factor_instances();
        self.set_provisional(None);
        Ok(())
    }
}

impl HasProvisionalSecurifiedConfig for SecuredEntityControl {
    fn get_provisional(&self) -> Option<ProvisionalSecurifiedConfig> {
        self.provisional_securified_config.clone()
    }

    fn set_provisional(
        &mut self,
        provisional_securified_config: impl Into<
            Option<ProvisionalSecurifiedConfig>,
        >,
    ) {
        self.provisional_securified_config =
            provisional_securified_config.into();
    }
}

pub trait HasProvisionalSecurifiedConfig {
    fn get_provisional(&self) -> Option<ProvisionalSecurifiedConfig>;

    fn set_provisional(
        &mut self,
        provisional_securified_config: impl Into<
            Option<ProvisionalSecurifiedConfig>,
        >,
    );
}

impl HasFactorInstances for SecuredEntityControl {
    fn unique_all_factor_instances(&self) -> IndexSet<FactorInstance> {
        self.security_structure.unique_all_factor_instances()
    }

    fn unique_tx_signing_factor_instances(&self) -> IndexSet<FactorInstance> {
        self.security_structure
            .matrix_of_factors
            .unique_tx_signing_factor_instances()
    }
}

impl SecuredEntityControl {
    /// # Panics
    /// Panics if veci is securified, i.e. the FactorInstances is in fact
    /// not a "VECI".
    pub fn new(
        veci: impl Into<Option<HierarchicalDeterministicFactorInstance>>,
        access_controller_address: AccessControllerAddress,
        security_structure: SecurityStructureOfFactorInstances,
    ) -> Result<Self> {
        let veci = veci.into();
        if let Some(veci) = veci.clone() {
            if veci.is_securified() {
                panic!("Got securified factor instance as veci, this is a programmer error!")
            }
        };
        Ok(Self {
            veci,
            access_controller_address,
            security_structure,
            provisional_securified_config: None,
        })
    }

    pub fn authentication_signing_factor_instance(
        &self,
    ) -> HierarchicalDeterministicFactorInstance {
        self.security_structure
            .authentication_signing_factor_instance
            .clone()
    }
}

impl SecuredEntityControl {
    pub fn veci(&self) -> Option<HierarchicalDeterministicFactorInstance> {
        self.veci.clone()
    }
}

impl HasSampleValues for SecuredEntityControl {
    fn sample() -> Self {
        let mut sample = Self::new(
            HierarchicalDeterministicFactorInstance::sample(),
            AccessControllerAddress::sample(),
            SecurityStructureOfFactorInstances::sample(),
        )
        .unwrap();
        sample.provisional_securified_config =
            Some(ProvisionalSecurifiedConfig::sample_other());
        sample
    }

    fn sample_other() -> Self {
        Self::new(HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_unsecurified_at_index(0), AccessControllerAddress::sample_other(), SecurityStructureOfFactorInstances::sample_other()).unwrap()
    }
}

#[cfg(debug_assertions)]
impl SecuredEntityControl {
    pub fn sign_with_first_recovery_role_factor(
        &self,
        hash: &Hash,
    ) -> SignatureWithPublicKey {
        let factor_instance = self.security_structure.matrix_of_factors.recovery().get_override_factors().first().expect("Security structure should have at least one factor in recovery role");
        factor_instance.sign_hash(hash)
    }

    pub fn sign_with_first_confirmation_role_factor(
        &self,
        hash: &Hash,
    ) -> SignatureWithPublicKey {
        let factor_instance = self.security_structure.matrix_of_factors.confirmation().get_override_factors().first().expect("Security structure should have at least one factor in recovery role");
        factor_instance.sign_hash(hash)
    }

    pub fn sign_with_primary_role_factors(
        &self,
        hash: &Hash,
    ) -> Vec<SignatureWithPublicKey> {
        self.security_structure
            .matrix_of_factors
            .primary()
            .get_threshold_factors()
            .iter()
            .map(|instance| instance.sign_hash(hash))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecuredEntityControl;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn unique_all_factor_instances_includes_rola() {
        let sut = SUT::sample();
        assert!(sut
            .unique_all_factor_instances()
            .into_iter()
            .map(|f| f.try_as_hd_factor_instances().unwrap())
            .any(|f| f.derivation_path().get_key_kind()
                == CAP26KeyKind::AuthenticationSigning));
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    #[should_panic(
        expected = "Got securified factor instance as veci, this is a programmer error!"
    )]
    fn test_panics_if_veci_is_in_securified_space() {
        _ = SUT::new(

            HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(0),
            AccessControllerAddress::sample(),
            SecurityStructureOfFactorInstances::sample(),
        );
    }

    #[test]
    fn json_roundtrip_sample() {
        let model = SUT::sample();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
             "veci": {
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
             },
            "accessControllerAddress": "accesscontroller_rdx1c0duj4lq0dc3cpl8qd420fpn5eckh8ljeysvjm894lyl5ja5yq6y5a",
            "securityStructure": {
               "securityStructureId": "ffffffff-ffff-ffff-ffff-ffffffffffff",
               "matrixOfFactors": {
                 "primaryRole": {
                   "threshold": "all",
                   "thresholdFactors": [
                     {
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
                     },
                     {
                       "factorSourceID": {
                         "discriminator": "fromHash",
                         "fromHash": {
                           "kind": "ledgerHQHardwareWallet",
                           "body": "ab59987eedd181fe98e512c1ba0f5ff059f11b5c7c56f15614dcc9fe03fec58b"
                         }
                       },
                       "badge": {
                         "discriminator": "virtualSource",
                         "virtualSource": {
                           "discriminator": "hierarchicalDeterministicPublicKey",
                           "hierarchicalDeterministicPublicKey": {
                             "publicKey": {
                               "curve": "curve25519",
                               "compressedData": "92cd6838cd4e7b0523ed93d498e093f71139ffd5d632578189b39a26005be56b"
                             },
                             "derivationPath": {
                               "scheme": "cap26",
                               "path": "m/44H/1022H/1H/525H/1460H/0S"
                             }
                           }
                         }
                       }
                     }
                   ],
                   "overrideFactors": []
                 },
                 "recoveryRole": {
                   "threshold": "all",
                   "thresholdFactors": [],
                   "overrideFactors": [
                     {
                       "factorSourceID": {
                         "discriminator": "fromHash",
                         "fromHash": {
                           "kind": "arculusCard",
                           "body": "12f36554769cd96614776e6dbd5629825b8e87366eec5e515de32bb1ea153820"
                         }
                       },
                       "badge": {
                         "discriminator": "virtualSource",
                         "virtualSource": {
                           "discriminator": "hierarchicalDeterministicPublicKey",
                           "hierarchicalDeterministicPublicKey": {
                             "publicKey": {
                               "curve": "curve25519",
                               "compressedData": "999bc2b17d012c3ce49da85b880029be5f9a9093247821f746ba73b6fc20e406"
                             },
                             "derivationPath": {
                               "scheme": "cap26",
                               "path": "m/44H/1022H/1H/525H/1460H/0S"
                             }
                           }
                         }
                       }
                     }
                   ]
                 },
                 "confirmationRole": {
                   "threshold": "all",
                   "thresholdFactors": [],
                   "overrideFactors": [
                     {
                       "factorSourceID": {
                         "discriminator": "fromHash",
                         "fromHash": {
                           "kind": "password",
                           "body": "181ab662e19fac3ad9f08d5c673b286d4a5ed9cd3762356dc9831dc42427c1b9"
                         }
                       },
                       "badge": {
                         "discriminator": "virtualSource",
                         "virtualSource": {
                           "discriminator": "hierarchicalDeterministicPublicKey",
                           "hierarchicalDeterministicPublicKey": {
                             "publicKey": {
                               "curve": "curve25519",
                               "compressedData": "4af49eb56b1af579aaf03f1760ec526f56e2297651f7a067f4b362f685417a81"
                             },
                             "derivationPath": {
                               "scheme": "cap26",
                               "path": "m/44H/1022H/1H/525H/1460H/0S"
                             }
                           }
                         }
                       }
                     }
                   ]
                 },
                 "timeUntilDelayedConfirmationIsCallable": {
                	"value": 2,
                	"unit": "weeks"
                 }
               },
               "authenticationSigningFactorInstance": {
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
                         "compressedData": "136b3a73595315517f921767bc49ae3ba43fc25d2e34e51fbff434a329176ee8"
                       },
                       "derivationPath": {
                         "scheme": "cap26",
                         "path": "m/44H/1022H/1H/525H/1678H/0S"
                       }
                     }
                   }
                 }
               }
             },
             "provisionalSecurifiedConfig": {
               "discriminator": "factorInstancesDerived",
               "value": {
                 "securityStructureId": "dededede-dede-dede-dede-dededededede",
                 "matrixOfFactors": {
                   "primaryRole": {
                     "threshold": "all",
                     "thresholdFactors": [
                       {
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
                                 "compressedData": "a40a1850ade79f5b24956b4abdb94624ba8189f68ad39fd2bb92ecdc2cbe17d2"
                               },
                               "derivationPath": {
                                 "scheme": "cap26",
                                 "path": "m/44H/1022H/1H/618H/1460H/0S"
                               }
                             }
                           }
                         }
                       }
                     ],
                     "overrideFactors": []
                   },
                   "recoveryRole": {
                     "threshold": "all",                   
                     "thresholdFactors": [],
                     "overrideFactors": [
                       {
                         "factorSourceID": {
                           "discriminator": "fromHash",
                           "fromHash": {
                             "kind": "ledgerHQHardwareWallet",
                             "body": "ab59987eedd181fe98e512c1ba0f5ff059f11b5c7c56f15614dcc9fe03fec58b"
                           }
                         },
                         "badge": {
                           "discriminator": "virtualSource",
                           "virtualSource": {
                             "discriminator": "hierarchicalDeterministicPublicKey",
                             "hierarchicalDeterministicPublicKey": {
                               "publicKey": {
                                 "curve": "curve25519",
                                 "compressedData": "6f7ac7d9031e321d1762431941b672f164ebb5a6dd2ded9b0c8da2b278143c74"
                               },
                               "derivationPath": {
                                 "scheme": "cap26",
                                 "path": "m/44H/1022H/1H/618H/1460H/0S"
                               }
                             }
                           }
                         }
                       }
                     ]
                   },
                   "confirmationRole": {
                     "threshold": "all",                   
                     "thresholdFactors": [],
                     "overrideFactors": [
                       {
                         "factorSourceID": {
                           "discriminator": "fromHash",
                           "fromHash": {
                             "kind": "ledgerHQHardwareWallet",
                             "body": "52ef052a0642a94279b296d6b3b17dedc035a7ae37b76c1d60f11f2725100077"
                           }
                         },
                         "badge": {
                           "discriminator": "virtualSource",
                           "virtualSource": {
                             "discriminator": "hierarchicalDeterministicPublicKey",
                             "hierarchicalDeterministicPublicKey": {
                               "publicKey": {
                                 "curve": "curve25519",
                                 "compressedData": "e867cd64b70cccad642f47ee4acff014b982870cf5218fbd56da79b0eb6e9fba"
                               },
                               "derivationPath": {
                                 "scheme": "cap26",
                                 "path": "m/44H/1022H/1H/618H/1460H/0S"
                               }
                             }
                           }
                         }
                       }
                     ]
                   },
                   "timeUntilDelayedConfirmationIsCallable": {
                      "value": 2,
                      "unit": "weeks"
                    }
                 },
                 "authenticationSigningFactorInstance": {
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
                           "compressedData": "d2343d84e7970224ad4f605782f78b096b750f03990c927492ba5308258c689a"
                         },
                         "derivationPath": {
                           "scheme": "cap26",
                           "path": "m/44H/1022H/1H/618H/1678H/0S"
                         }
                       }
                     }
                   }
                 }
               }
             }
            }
        "#,
        );
    }
}
