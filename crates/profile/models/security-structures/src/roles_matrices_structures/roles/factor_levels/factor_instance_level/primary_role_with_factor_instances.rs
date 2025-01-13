use crate::prelude::*;

pub type PrimaryRoleWithFactorInstances =
    RoleWithFactorInstances<{ ROLE_PRIMARY }>;

impl HasSampleValues for PrimaryRoleWithFactorInstances {
    fn sample() -> Self {
        MatrixOfFactorInstances::sample().primary_role
    }

    fn sample_other() -> Self {
        MatrixOfFactorInstances::sample_other().primary_role
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PrimaryRoleWithFactorInstances;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    #[should_panic]
    fn primary_role_non_securified_threshold_instances_is_err() {
        let _ = SUT::with_factors(
                1,
                [
                    HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_unsecurified_at_index(0).into()
                ],
                []
            );
    }

    #[test]
    fn assert_json_sample() {
        let sut = SUT::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
                        {
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
            }
            "#,
        );
    }
}
