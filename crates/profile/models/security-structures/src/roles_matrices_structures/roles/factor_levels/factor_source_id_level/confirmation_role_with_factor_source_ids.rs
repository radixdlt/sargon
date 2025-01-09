use crate::prelude::*;

pub type ConfirmationRoleWithFactorSourceIDs =
    RoleWithFactorSourceIds<{ ROLE_CONFIRMATION }>;
pub type ConfirmationRoleWithFactorSourceIds =
    ConfirmationRoleWithFactorSourceIDs;

impl HasSampleValues for ConfirmationRoleWithFactorSourceIds {
    /// Config MFA 1.1
    fn sample() -> Self {
        let mut builder = RoleBuilder::new();
        builder
            .add_factor_source(FactorSourceID::sample_password())
            .unwrap();
        builder.build().unwrap()
    }

    /// Config MFA 2.1
    fn sample_other() -> Self {
        let mut builder = RoleBuilder::new();
        builder
            .add_factor_source(FactorSourceID::sample_device())
            .unwrap();
        builder.build().unwrap()
    }
}
impl HasSampleValues for RecoveryRoleWithFactorSourceIds {
    /// Config MFA 1.1
    fn sample() -> Self {
        let mut builder = RoleBuilder::new();
        builder
            .add_factor_source(FactorSourceID::sample_device())
            .unwrap();

        builder
            .add_factor_source(FactorSourceID::sample_ledger())
            .unwrap();
        builder.build().unwrap()
    }

    /// Config MFA 3.3
    fn sample_other() -> Self {
        let mut builder = RoleBuilder::new();
        builder
            .add_factor_source(FactorSourceID::sample_ledger_other())
            .unwrap();

        builder.build().unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ConfirmationRoleWithFactorSourceIds;

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
                    "kind": "password",
                    "body": "181ab662e19fac3ad9f08d5c673b286d4a5ed9cd3762356dc9831dc42427c1b9"
                  }
                }
              ]
            }
            "#,
        );
    }
}
