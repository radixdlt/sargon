use crate::prelude::*;

pub type MatrixOfFactorSourceIds = AbstractMatrixBuilt<FactorSourceID>;

impl MatrixOfFactorSourceIds {
    pub(crate) fn _unvalidated_with_roles(
        primary: PrimaryRoleWithFactorSourceIds,
        recovery: RecoveryRoleWithFactorSourceIds,
        confirmation: ConfirmationRoleWithFactorSourceIds,
    ) -> Self {
        unsafe {
            Self::unbuilt_with_roles_and_days(
                primary,
                recovery,
                confirmation,
                Self::DEFAULT_TIME_UNTIL_DELAYED_CONFIRMATION_IS_CALLABLE,
            )
        }
    }
}

#[cfg(test)]
impl MatrixOfFactorSourceIds {
    pub(crate) fn with_roles_and_days(
        primary: PrimaryRoleWithFactorSourceIds,
        recovery: RecoveryRoleWithFactorSourceIds,
        confirmation: ConfirmationRoleWithFactorSourceIds,
        time_until_delayed_confirmation_is_callable: TimePeriod,
    ) -> Self {
        unsafe {
            Self::unbuilt_with_roles_and_days(
                primary,
                recovery,
                confirmation,
                time_until_delayed_confirmation_is_callable,
            )
        }
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
            Self::DEFAULT_TIME_UNTIL_DELAYED_CONFIRMATION_IS_CALLABLE,
        )
    }
}

impl From<MatrixOfFactorInstances> for MatrixOfFactorSourceIds {
    fn from(value: MatrixOfFactorInstances) -> Self {
        unsafe {
            Self::unbuilt_with_roles_and_days(
                PrimaryRoleWithFactorSourceIds::from(value.primary_role),
                RecoveryRoleWithFactorSourceIds::from(value.recovery_role),
                ConfirmationRoleWithFactorSourceIds::from(
                    value.confirmation_role,
                ),
                value.time_until_delayed_confirmation_is_callable,
            )
        }
    }
}

impl MatrixOfFactorSourceIds {
    fn sample_from_template(template: MatrixTemplate) -> Self {
        template
            .materialize_ids(*ALL_FACTOR_SOURCE_ID_SAMPLES_INC_NON_HD)
            .unwrap()
    }

    /// Config 1.1 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn sample_config_1_1() -> Self {
        Self::sample_from_template(MatrixTemplate::config_1_1())
    }

    /// Config 1.2 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn sample_config_1_2() -> Self {
        Self::sample_from_template(MatrixTemplate::config_1_2())
    }

    /// Config 1.3 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn sample_config_1_3() -> Self {
        Self::sample_from_template(MatrixTemplate::config_1_3())
    }

    /// Config 1.4 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn sample_config_1_4() -> Self {
        Self::sample_from_template(MatrixTemplate::config_1_4())
    }

    /// Config 1.5 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn sample_config_1_5() -> Self {
        Self::sample_from_template(MatrixTemplate::config_1_5())
    }

    /// Config 2.1 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn sample_config_2_1() -> Self {
        Self::sample_from_template(MatrixTemplate::config_2_1())
    }

    /// Config 2.2 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn sample_config_2_2() -> Self {
        Self::sample_from_template(MatrixTemplate::config_2_2())
    }

    /// Config 2.3 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn sample_config_2_3() -> Self {
        Self::sample_from_template(MatrixTemplate::config_2_3())
    }

    /// Config 2.4 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn sample_config_2_4() -> Self {
        Self::sample_from_template(MatrixTemplate::config_2_4())
    }

    /// Config 3.0 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn sample_config_3_0() -> Self {
        Self::sample_from_template(MatrixTemplate::config_3_0())
    }

    /// Config 4.0 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn sample_config_4_0() -> Self {
        Self::sample_from_template(MatrixTemplate::config_4_0())
    }

    /// Config 5.1 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn sample_config_5_1() -> Self {
        Self::sample_from_template(MatrixTemplate::config_5_1())
    }

    /// Config 5.2 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn sample_config_5_2() -> Self {
        Self::sample_from_template(MatrixTemplate::config_5_2())
    }

    /// Config 6.0 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn sample_config_6_0() -> Self {
        Self::sample_from_template(MatrixTemplate::config_6_0())
    }

    /// Config 7.0 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn sample_config_7_0() -> Self {
        Self::sample_from_template(MatrixTemplate::config_7_0())
    }

    /// Config 8.0 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn sample_config_8_0() -> Self {
        Self::sample_from_template(MatrixTemplate::config_8_0())
    }

    /// Config 9.0 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn sample_config_9_0() -> Self {
        Self::sample_from_template(MatrixTemplate::config_9_0())
    }
}

impl HasSampleValues for MatrixOfFactorSourceIds {
    fn sample() -> Self {
        Self::sample_config_1_1()
    }

    fn sample_other() -> Self {
        Self::sample_config_2_4()
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
        assert_ne!(SUT::sample(), SUT::sample_config_1_2());
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
                SUT::sample_config_1_1(),
                SUT::sample_config_1_2(),
                SUT::sample_config_1_3(),
                SUT::sample_config_1_4(),
                SUT::sample_config_1_5(),
                SUT::sample_config_2_1(),
                SUT::sample_config_2_2(),
                SUT::sample_config_2_3(),
                SUT::sample_config_2_4(),
                SUT::sample_config_3_0(),
                SUT::sample_config_4_0(),
                SUT::sample_config_5_1(),
                SUT::sample_config_5_2(),
                SUT::sample_config_6_0(),
                SUT::sample_config_7_0(),
                SUT::sample_config_8_0(),
                SUT::sample_config_9_0(),
                // Duplicates should be removed
                SUT::sample_config_1_1(),
                SUT::sample_config_1_2(),
                SUT::sample_config_1_3(),
                SUT::sample_config_1_4(),
                SUT::sample_config_1_5(),
                SUT::sample_config_2_1(),
                SUT::sample_config_2_2(),
                SUT::sample_config_2_3(),
                SUT::sample_config_2_4(),
                SUT::sample_config_3_0(),
                SUT::sample_config_4_0(),
                SUT::sample_config_5_1(),
                SUT::sample_config_5_2(),
                SUT::sample_config_6_0(),
                SUT::sample_config_7_0(),
                SUT::sample_config_8_0(),
                SUT::sample_config_9_0(),
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
                "threshold": "all",
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
                "threshold": "all",
                "thresholdFactors": [],
                "overrideFactors": [
                  {
                    "discriminator": "fromHash",
                    "fromHash": {
                      "kind": "arculusCard",
                      "body": "12f36554769cd96614776e6dbd5629825b8e87366eec5e515de32bb1ea153820"
                    }
                  }
                ]
              },
              "confirmationRole": {
                "threshold": "all",
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
              "timeUntilDelayedConfirmationIsCallable": {
            	"value": 2,
            	"unit": "weeks"
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
              "primaryRole": {
                "threshold": "all",
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
                "threshold": "all",
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
                "threshold": "all",
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
              "timeUntilDelayedConfirmationIsCallable": {
            	"value": 2,
            	"unit": "weeks"
              }
            }
            "#,
        );
    }
}
