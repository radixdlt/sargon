use crate::prelude::*;

pub type MatrixOfFactorSourceIds = AbstractMatrixBuilt<FactorSourceID>;

impl MatrixOfFactorSourceIds {
    pub(crate) fn _unvalidated_with_roles_and_days(
        primary: PrimaryRoleWithFactorSourceIds,
        recovery: RecoveryRoleWithFactorSourceIds,
        confirmation: ConfirmationRoleWithFactorSourceIds,
        number_of_days_until_auto_confirm: u16,
    ) -> Self {
        assert_eq!(primary.role(), RoleKind::Primary);
        assert_eq!(recovery.role(), RoleKind::Recovery);
        assert_eq!(confirmation.role(), RoleKind::Confirmation);
        Self {
            built: PhantomData,
            primary_role: primary,
            recovery_role: recovery,
            confirmation_role: confirmation,
            number_of_days_until_auto_confirm,
        }
    }

    pub(crate) fn _unvalidated_with_roles(
        primary: PrimaryRoleWithFactorSourceIds,
        recovery: RecoveryRoleWithFactorSourceIds,
        confirmation: ConfirmationRoleWithFactorSourceIds,
    ) -> Self {
        Self::_unvalidated_with_roles_and_days(
            primary,
            recovery,
            confirmation,
            Self::DEFAULT_NUMBER_OF_DAYS_UNTIL_AUTO_CONFIRM,
        )
    }
}

#[cfg(test)]
impl MatrixOfFactorSourceIds {
    pub(crate) fn with_roles_and_days(
        primary: PrimaryRoleWithFactorSourceIds,
        recovery: RecoveryRoleWithFactorSourceIds,
        confirmation: ConfirmationRoleWithFactorSourceIds,
        number_of_days_until_auto_confirm: u16,
    ) -> Self {
        Self::_unvalidated_with_roles_and_days(
            primary,
            recovery,
            confirmation,
            number_of_days_until_auto_confirm,
        )
    }

    pub(crate) fn with_roles(
        primary: PrimaryRoleWithFactorSourceIds,
        recovery: RecoveryRoleWithFactorSourceIds,
        confirmation: ConfirmationRoleWithFactorSourceIds,
    ) -> Self {
        Self::with_roles_and_days(
            primary,
            recovery,
            confirmation,
            Self::DEFAULT_NUMBER_OF_DAYS_UNTIL_AUTO_CONFIRM,
        )
    }
}

impl MatrixOfFactorSourceIds {
    fn sample_from_template(template: MatrixTemplate) -> Self {
        template
            .materialize_ids(*ALL_FACTOR_SOURCE_ID_SAMPLES_INC_NON_HD)
            .unwrap()
    }

    pub fn sample_config_11() -> Self {
        Self::sample_from_template(MatrixTemplate::config_11())
    }

    pub fn sample_config_12() -> Self {
        Self::sample_from_template(MatrixTemplate::config_12())
    }

    pub fn sample_config_13() -> Self {
        Self::sample_from_template(MatrixTemplate::config_13())
    }

    pub fn sample_config_14() -> Self {
        Self::sample_from_template(MatrixTemplate::config_14())
    }

    pub fn sample_config_15() -> Self {
        Self::sample_from_template(MatrixTemplate::config_15())
    }

    pub fn sample_config_21() -> Self {
        Self::sample_from_template(MatrixTemplate::config_21())
    }

    pub fn sample_config_22() -> Self {
        Self::sample_from_template(MatrixTemplate::config_22())
    }

    pub fn sample_config_23() -> Self {
        Self::sample_from_template(MatrixTemplate::config_23())
    }

    pub fn sample_config_24() -> Self {
        Self::sample_from_template(MatrixTemplate::config_24())
    }

    pub fn sample_config_30() -> Self {
        Self::sample_from_template(MatrixTemplate::config_30())
    }

    pub fn sample_config_40() -> Self {
        Self::sample_from_template(MatrixTemplate::config_40())
    }

    pub fn sample_config_51() -> Self {
        Self::sample_from_template(MatrixTemplate::config_51())
    }

    pub fn sample_config_52() -> Self {
        Self::sample_from_template(MatrixTemplate::config_52())
    }

    pub fn sample_config_60() -> Self {
        Self::sample_from_template(MatrixTemplate::config_60())
    }

    pub fn sample_config_70() -> Self {
        Self::sample_from_template(MatrixTemplate::config_70())
    }

    pub fn sample_config_80() -> Self {
        Self::sample_from_template(MatrixTemplate::config_80())
    }

    pub fn sample_config_90() -> Self {
        Self::sample_from_template(MatrixTemplate::config_90())
    }
}

impl HasSampleValues for MatrixOfFactorSourceIds {
    fn sample() -> Self {
        Self::sample_config_11()
    }

    fn sample_other() -> Self {
        Self::sample_config_24()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(clippy::upper_case_acronyms)]
    type SUT = MatrixOfFactorSourceIds;

    #[test]
    fn template() {}

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
        assert_ne!(SUT::sample(), SUT::sample_config_12());
        assert_ne!(SUT::sample().primary(), SUT::sample_other().primary());
        assert_ne!(SUT::sample().recovery(), SUT::sample_other().recovery());
        assert_ne!(
            SUT::sample().confirmation(),
            SUT::sample_other().confirmation()
        );
    }

    #[test]
    fn hash() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                SUT::sample_config_11(),
                SUT::sample_config_12(),
                SUT::sample_config_13(),
                SUT::sample_config_14(),
                SUT::sample_config_15(),
                SUT::sample_config_21(),
                SUT::sample_config_22(),
                SUT::sample_config_23(),
                SUT::sample_config_24(),
                SUT::sample_config_30(),
                SUT::sample_config_40(),
                SUT::sample_config_51(),
                SUT::sample_config_52(),
                SUT::sample_config_60(),
                SUT::sample_config_70(),
                SUT::sample_config_80(),
                SUT::sample_config_90(),
                // Duplicates should be removed
                SUT::sample_config_11(),
                SUT::sample_config_12(),
                SUT::sample_config_13(),
                SUT::sample_config_14(),
                SUT::sample_config_15(),
                SUT::sample_config_21(),
                SUT::sample_config_22(),
                SUT::sample_config_23(),
                SUT::sample_config_24(),
                SUT::sample_config_30(),
                SUT::sample_config_40(),
                SUT::sample_config_51(),
                SUT::sample_config_52(),
                SUT::sample_config_60(),
                SUT::sample_config_70(),
                SUT::sample_config_80(),
                SUT::sample_config_90(),
            ])
            .len(),
            17
        );
    }

    #[test]
    fn assert_json_sample() {
        let sut = SUT::sample();

        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
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
            "#,
        );
    }
}
