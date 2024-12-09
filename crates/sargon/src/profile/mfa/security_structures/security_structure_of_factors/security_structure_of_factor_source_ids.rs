use crate::prelude::*;

pub type SecurityStructureOfFactorSourceIds =
    AbstractSecurityStructure<FactorSourceID>;

impl HasSampleValues for SecurityStructureOfFactorSourceIds {
    fn sample() -> Self {
        let metadata = SecurityStructureMetadata::sample();
        Self::with_metadata(metadata, MatrixOfFactorSourceIds::sample())
    }

    fn sample_other() -> Self {
        let metadata = SecurityStructureMetadata::sample_other();
        Self::with_metadata(metadata, MatrixOfFactorSourceIds::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityStructureOfFactorSourceIds;

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
    fn assert_json_sample() {
        let sut = SUT::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
                        {
              "metadata": {
                "id": "ffffffff-ffff-ffff-ffff-ffffffffffff",
                "displayName": "Spending Account",
                "createdOn": "2023-09-11T16:05:56.000Z",
                "lastUpdatedOn": "2023-09-11T16:05:56.000Z"
              },
              "matrixOfFactors": {
                "primaryRole": {
                  "threshold": 2,
                  "thresholdFactors": [
                    {
                      "discriminator": "fromHash",
                      "fromHash": {
                        "kind": "device",
                        "body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
                      }
                    },
                    {
                      "discriminator": "fromHash",
                      "fromHash": {
                        "kind": "ledgerHQHardwareWallet",
                        "body": "ab59987eedd181fe98e512c1ba0f5ff059f11b5c7c56f15614dcc9fe03fec58b"
                      }
                    }
                  ],
                  "overrideFactors": []
                },
                "recoveryRole": {
                  "threshold": 0,
                  "thresholdFactors": [],
                  "overrideFactors": [
                    {
                      "discriminator": "fromHash",
                      "fromHash": {
                        "kind": "device",
                        "body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
                      }
                    },
                    {
                      "discriminator": "fromHash",
                      "fromHash": {
                        "kind": "ledgerHQHardwareWallet",
                        "body": "ab59987eedd181fe98e512c1ba0f5ff059f11b5c7c56f15614dcc9fe03fec58b"
                      }
                    }
                  ]
                },
                "confirmationRole": {
                  "threshold": 0,
                  "thresholdFactors": [],
                  "overrideFactors": [
                    {
                      "discriminator": "fromHash",
                      "fromHash": {
                        "kind": "password",
                        "body": "181ab662e19fac3ad9f08d5c673b286d4a5ed9cd3762356dc9831dc42427c1b9"
                      }
                    }
                  ]
                },
                "numberOfDaysUntilAutoConfirm": 14
              }
            }
            "#,
        );
    }

    #[test]
    fn assert_json_sample_other() {
        let sut = SUT::sample_other();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
              "metadata": {
                "id": "dededede-dede-dede-dede-dededededede",
                "displayName": "Savings Account",
                "createdOn": "2023-12-24T17:13:56.123Z",
                "lastUpdatedOn": "2023-12-24T17:13:56.123Z"
              },
              "matrixOfFactors": {
                "primaryRole": {
                  "threshold": 1,
                  "thresholdFactors": [
                    {
                    "discriminator": "fromHash",
                    "fromHash": {
                      "kind": "device",
                      "body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
                      }
                    }
                  ],
                  "overrideFactors": []
                },
                "recoveryRole": {
                  "threshold": 0,
                  "thresholdFactors": [],
                  "overrideFactors": [
                    {
                      "discriminator": "fromHash",
                      "fromHash": {
                        "kind": "ledgerHQHardwareWallet",
                        "body": "ab59987eedd181fe98e512c1ba0f5ff059f11b5c7c56f15614dcc9fe03fec58b"
                      }
                    }
                  ]
                },
                "confirmationRole": {
                  "threshold": 0,
                  "thresholdFactors": [],
                  "overrideFactors": [
                    {
                      "discriminator": "fromHash",
                      "fromHash": {
                        "kind": "ledgerHQHardwareWallet",
                        "body": "52ef052a0642a94279b296d6b3b17dedc035a7ae37b76c1d60f11f2725100077"
                      }
                    }
                  ]
                },
                "numberOfDaysUntilAutoConfirm": 14
              }
            }
          "#,
        );
    }
}
