use crate::prelude::*;

pub type RecoveryRoleWithFactorSourceIDs =
    RoleWithFactorSourceIds<{ ROLE_RECOVERY }>;
pub type RecoveryRoleWithFactorSourceIds = RecoveryRoleWithFactorSourceIDs;

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RecoveryRoleWithFactorSourceIds;

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
    fn get_all_factors() {
        let sut = SUT::sample();
        let factors = sut.all_factors();
        assert_eq!(
            factors.len(),
            sut.get_override_factors().len()
                + sut.get_threshold_factors().len()
        );
    }

    #[test]
    fn assert_json() {
        let sut = SUT::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
              "threshold": {
                "specific": 0
              },
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
            }
            "#,
        );
    }
}
