use crate::prelude::*;

decl_identified_vec_of!(
    /// An ordered set of ['MFAFactorInstance`]s on a specific network.
    MFAFactorInstance
);

impl MFAFactorInstances {
    pub fn max_index_component(&self) -> Option<HDPathComponent> {
        self.iter()
            .filter_map(|mfafi| {
                mfafi
                    .factor_instance
                    .badge
                    .as_virtual()
                    .map(|badge_source| {
                        badge_source
                            .as_hierarchical_deterministic()
                            .derivation_path
                            .index()
                    })
            })
            .max()
    }
}

impl HasSampleValues for MFAFactorInstances {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::sample_stokenet()
    }
}

impl HasSampleValuesOnNetworks for MFAFactorInstances {
    /// A sample used to facilitate unit tests.
    fn sample_mainnet() -> Self {
        Self::from_iter([
            MFAFactorInstance::sample_mainnet_account_securified_idx_0(),
            MFAFactorInstance::sample_mainnet_account_securified_idx_1(),
        ])
    }

    /// A sample used to facilitate unit tests.
    fn sample_stokenet() -> Self {
        Self::from_iter([
            MFAFactorInstance::sample_stokenet_account_securified_idx_0(),
            MFAFactorInstance::sample_stokenet_account_securified_idx_1(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn default_is_empty() {
        assert_eq!(MFAFactorInstances::default().len(), 0);
    }

    #[test]
    fn inequality() {
        assert_ne!(
            MFAFactorInstances::sample(),
            MFAFactorInstances::sample_other()
        );
    }

    #[test]
    fn equality() {
        assert_eq!(MFAFactorInstances::sample(), MFAFactorInstances::sample());
        assert_eq!(
            MFAFactorInstances::sample_other(),
            MFAFactorInstances::sample_other()
        );
    }

    #[test]
    fn duplicates_are_prevented() {
        assert_eq!(
            MFAFactorInstances::from_iter(
                [MFAFactorInstance::sample(), MFAFactorInstance::sample()]
                    .into_iter()
            )
            .len(),
            1
        )
    }

    #[test]
    fn with_one() {
        assert_eq!(
            MFAFactorInstances::just(MFAFactorInstance::sample()).len(),
            1
        )
    }

    #[test]
    fn get_all() {
        assert_eq!(MFAFactorInstances::sample().get_all().len(), 2);
    }

    #[test]
    fn get_by_public_key() {
        let mfa_factor_instance = MFAFactorInstance::sample();
        let public_key = mfa_factor_instance.factor_instance.id();
        let mfa_factor_instances =
            MFAFactorInstances::just(mfa_factor_instance.clone());
        assert_eq!(
            mfa_factor_instances.get_id(public_key),
            Some(&mfa_factor_instance)
        );
    }

    #[test]
    fn json_roundtrip_mainnet() {
        let sut = MFAFactorInstances::sample_mainnet();

        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            [
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
            "#,
        );
    }

    #[test]
    fn json_roundtrip_stokenet() {
        let sut = MFAFactorInstances::sample_stokenet();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            [
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
            "#,
        );
    }
}
