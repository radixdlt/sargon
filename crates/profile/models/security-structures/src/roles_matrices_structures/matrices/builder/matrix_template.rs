use crate::prelude::*;

/// A Matrix of FactorSourceTemplates, can be used to create template
/// "SecurityShields", mostly useful for coding/tests, but theoretically
/// we could UniFFI export these and use them in the hosts wallets, which would
/// pre-populate SecurityShield-builder flow screens - if hosts/Sargon manages
/// to assign each template "slot" with a concrete FactorSourceID, known as
/// materialization.
pub type MatrixTemplate = AbstractMatrixBuilt<FactorSourceTemplate>;

impl<const ROLE: u8> AbstractBuiltRoleWithFactor<ROLE, FactorSourceTemplate> {
    /// Tries to materialize a RoleWithFactorSourceIds from a RoleTemplate by
    /// assigning each template with a concrete FactorSourceID using the FactorSourceIdAssigner.
    pub(crate) fn assign(
        self,
        factor_source_id_assigner: &mut FactorSourceIdAssigner,
    ) -> Result<RoleWithFactorSourceIds<ROLE>, CommonError> {
        let mut fulfill =
            |xs: &Vec<FactorSourceTemplate>| -> Result<Vec<FactorSourceID>, CommonError> {
                xs.iter()
                    .map(|f| factor_source_id_assigner.next(f))
                    .collect::<Result<Vec<_>, CommonError>>()
            };
        Ok(RoleWithFactorSourceIds::with_factors_and_threshold(
            self.get_threshold(),
            fulfill(self.get_threshold_factors())?,
            fulfill(self.get_override_factors())?,
        ))
    }
}

/// A helper which assigns FactorSourceIDs to FactorSourceTemplates, used for
/// materializing a MatrixTemplate into a MatrixOfFactorSourceIds.
pub(crate) struct FactorSourceIdAssigner {
    factor_source_ids: Vec<FactorSourceID>,
    map: IndexMap<FactorSourceTemplate, FactorSourceID>,
}

impl FactorSourceIdAssigner {
    fn new(
        factor_source_ids: impl IntoIterator<Item = FactorSourceID>,
    ) -> Self {
        Self {
            factor_source_ids: factor_source_ids.into_iter().collect_vec(),
            map: IndexMap::new(),
        }
    }

    fn next(
        &mut self,
        template: &FactorSourceTemplate,
    ) -> Result<FactorSourceID, CommonError> {
        if let Some(existing) = self.map.get(template) {
            Ok(*existing)
        } else if let Some(index_of_next) = self
            .factor_source_ids
            .iter()
            .position(|f| f.get_factor_source_kind() == template.kind)
        {
            let next = self.factor_source_ids.remove(index_of_next);
            self.map.insert(template.clone(), next);
            Ok(next)
        } else {
            Err(CommonError::TooFewFactorInstancesDerived)
        }
    }
}

impl MatrixTemplate {
    pub fn materialize(
        self,
        factor_source_ids: impl IntoIterator<Item = FactorSource>,
    ) -> Result<MatrixOfFactorSourceIds, CommonError> {
        self.materialize_ids(
            factor_source_ids.into_iter().map(|f| f.factor_source_id()),
        )
    }

    /// Tries to materialize a MatrixOfFactorSourceIds from a MatrixTemplate by
    /// assigning each template with a concrete FactorSourceID using the `factor_source_ids`.`
    pub fn materialize_ids(
        self,
        factor_source_ids: impl IntoIterator<Item = FactorSourceID>,
    ) -> Result<MatrixOfFactorSourceIds, CommonError> {
        let time_until_delayed_confirmation_is_callable =
            self.time_until_delayed_confirmation_is_callable;

        let mut assigner = FactorSourceIdAssigner::new(factor_source_ids);

        let primary_role = self.primary_role.assign(&mut assigner)?;

        let recovery_role = self.recovery_role.assign(&mut assigner)?;

        let confirmation_role = self.confirmation_role.assign(&mut assigner)?;

        let matrix = unsafe {
            MatrixOfFactorSourceIds::unbuilt_with_roles_and_days(
                primary_role,
                recovery_role,
                confirmation_role,
                time_until_delayed_confirmation_is_callable,
            )
        };

        Ok(matrix)
    }
}

impl MatrixTemplate {
    fn new(
        primary_role: PrimaryRoleTemplate,
        recovery_role: RecoveryRoleTemplate,
        confirmation_role: ConfirmationRoleTemplate,
    ) -> Self {
        unsafe {
            Self::unbuilt_with_roles_and_days(primary_role, recovery_role, confirmation_role, MatrixOfFactorSourceIds::DEFAULT_TIME_UNTIL_DELAYED_CONFIRMATION_IS_CALLABLE)
        }
    }

    /// Config 1.1 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn config_1_1() -> Self {
        Self::new(
            PrimaryRoleTemplate::new([
                FactorSourceTemplate::device(),
                FactorSourceTemplate::ledger(),
            ]),
            RecoveryRoleTemplate::new([
                FactorSourceTemplate::arculus(),
            ]),
            ConfirmationRoleTemplate::new([FactorSourceTemplate::password()]),
        )
    }

    /// Config 1.2 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn config_1_2() -> Self {
        Self::new(
            PrimaryRoleTemplate::new([
                FactorSourceTemplate::ledger(),
                FactorSourceTemplate::password(),
            ]),
            RecoveryRoleTemplate::new([
                FactorSourceTemplate::device(),
                FactorSourceTemplate::ledger(),
            ]),
            ConfirmationRoleTemplate::new([FactorSourceTemplate::password()]),
        )
    }

    /// Config 1.3 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn config_1_3() -> Self {
        Self::new(
            PrimaryRoleTemplate::new([
                FactorSourceTemplate::device(),
                FactorSourceTemplate::password(),
            ]),
            RecoveryRoleTemplate::new([
                FactorSourceTemplate::device(),
                FactorSourceTemplate::ledger(),
            ]),
            ConfirmationRoleTemplate::new([FactorSourceTemplate::password()]),
        )
    }

    /// Config 1.4 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn config_1_4() -> Self {
        Self::new(
            PrimaryRoleTemplate::new([FactorSourceTemplate::device()]),
            RecoveryRoleTemplate::new([FactorSourceTemplate::ledger()]),
            ConfirmationRoleTemplate::new([FactorSourceTemplate::password()]),
        )
    }

    /// Config 1.5 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn config_1_5() -> Self {
        Self::new(
            PrimaryRoleTemplate::new([FactorSourceTemplate::ledger()]),
            RecoveryRoleTemplate::new([FactorSourceTemplate::device()]),
            ConfirmationRoleTemplate::new([FactorSourceTemplate::password()]),
        )
    }

    /// Config 2.1 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn config_2_1() -> Self {
        Self::new(
            PrimaryRoleTemplate::new([
                FactorSourceTemplate::device(),
                FactorSourceTemplate::ledger(),
            ]),
            RecoveryRoleTemplate::new([
                FactorSourceTemplate::ledger(),
                FactorSourceTemplate::ledger_other(),
            ]),
            ConfirmationRoleTemplate::new([FactorSourceTemplate::device()]),
        )
    }

    /// Config 2.2 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn config_2_2() -> Self {
        Self::new(
            PrimaryRoleTemplate::new([
                FactorSourceTemplate::ledger(),
                FactorSourceTemplate::ledger_other(),
            ]),
            RecoveryRoleTemplate::new([
                FactorSourceTemplate::ledger(),
                FactorSourceTemplate::ledger_other(),
            ]),
            ConfirmationRoleTemplate::new([FactorSourceTemplate::device()]),
        )
    }

    /// Config 2.3 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn config_2_3() -> Self {
        Self::new(
            PrimaryRoleTemplate::new([FactorSourceTemplate::ledger()]),
            RecoveryRoleTemplate::new([FactorSourceTemplate::ledger_other()]),
            ConfirmationRoleTemplate::new([FactorSourceTemplate::device()]),
        )
    }

    /// Config 2.4 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn config_2_4() -> Self {
        Self::new(
            PrimaryRoleTemplate::new([FactorSourceTemplate::device()]),
            RecoveryRoleTemplate::new([FactorSourceTemplate::ledger()]),
            ConfirmationRoleTemplate::new([
                FactorSourceTemplate::ledger_other(),
            ]),
        )
    }

    /// Config 3 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn config_3_0() -> Self {
        Self::new(
            PrimaryRoleTemplate::new([
                FactorSourceTemplate::device(),
                FactorSourceTemplate::ledger(),
            ]),
            RecoveryRoleTemplate::new([
                FactorSourceTemplate::ledger(),
                FactorSourceTemplate::ledger_other(),
            ]),
            ConfirmationRoleTemplate::new([
                FactorSourceTemplate::device(),
                FactorSourceTemplate::password(),
            ]),
        )
    }

    /// Config 4 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn config_4_0() -> Self {
        Self::new(
            PrimaryRoleTemplate::new([
                FactorSourceTemplate::device(),
                FactorSourceTemplate::ledger(),
            ]),
            RecoveryRoleTemplate::new([
                FactorSourceTemplate::device(),
                FactorSourceTemplate::ledger(),
            ]),
            ConfirmationRoleTemplate::new([
                FactorSourceTemplate::password(),
                FactorSourceTemplate::password_other(),
                FactorSourceTemplate::off_device_mnemonic(),
            ]),
        )
    }

    /// Config 5.1 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn config_5_1() -> Self {
        Self::new(
            PrimaryRoleTemplate::new([
                FactorSourceTemplate::device(),
                FactorSourceTemplate::password(),
            ]),
            RecoveryRoleTemplate::new(
                [FactorSourceTemplate::trusted_contact()],
            ),
            ConfirmationRoleTemplate::new([FactorSourceTemplate::password()]),
        )
    }

    /// Config 5.2 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn config_5_2() -> Self {
        Self::new(
            PrimaryRoleTemplate::new([
                FactorSourceTemplate::device(),
                FactorSourceTemplate::password(),
            ]),
            RecoveryRoleTemplate::new([
                FactorSourceTemplate::trusted_contact(),
                FactorSourceTemplate::trusted_contact_other(),
                FactorSourceTemplate::device(),
            ]),
            ConfirmationRoleTemplate::new([
                FactorSourceTemplate::password(),
                FactorSourceTemplate::password_other(),
                FactorSourceTemplate::off_device_mnemonic(),
            ]),
        )
    }

    /// Config 6.0 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn config_6_0() -> Self {
        Self::new(
            PrimaryRoleTemplate::new([FactorSourceTemplate::device()]),
            RecoveryRoleTemplate::new(
                [FactorSourceTemplate::trusted_contact()],
            ),
            ConfirmationRoleTemplate::new([
                FactorSourceTemplate::security_questions(),
            ]),
        )
    }

    /// Config 7.0 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn config_7_0() -> Self {
        Self::new(
            PrimaryRoleTemplate::new([
                FactorSourceTemplate::device(),
                FactorSourceTemplate::ledger(),
            ]),
            RecoveryRoleTemplate::new([
                FactorSourceTemplate::trusted_contact(),
                FactorSourceTemplate::ledger(),
            ]),
            ConfirmationRoleTemplate::new([FactorSourceTemplate::device()]),
        )
    }

    /// Config 8.0 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn config_8_0() -> Self {
        Self::new(
            PrimaryRoleTemplate::new([
                FactorSourceTemplate::device(),
                FactorSourceTemplate::ledger(),
            ]),
            RecoveryRoleTemplate::new([
                FactorSourceTemplate::ledger(),
                FactorSourceTemplate::device(),
            ]),
            ConfirmationRoleTemplate::new([
                FactorSourceTemplate::security_questions(),
            ]),
        )
    }

    /// Config 9.0 according to [this document][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields#Example-Security-Shield-Configurations
    pub fn config_9_0() -> Self {
        Self::new(
            PrimaryRoleTemplate::new([
                FactorSourceTemplate::device(),
                FactorSourceTemplate::ledger(),
            ]),
            RecoveryRoleTemplate::new([
                FactorSourceTemplate::trusted_contact(),
                FactorSourceTemplate::device(),
            ]),
            ConfirmationRoleTemplate::new([
                FactorSourceTemplate::security_questions(),
            ]),
        )
    }
}

#[cfg(test)]
mod test_templates {
    use super::*;

    fn test_template(
        template: MatrixTemplate,
        expected: MatrixOfFactorSourceIds,
    ) {
        let m = template
            .materialize_ids(*ALL_FACTOR_SOURCE_ID_SAMPLES_INC_NON_HD)
            .unwrap();
        pretty_assertions::assert_eq!(m, expected);
    }

    #[test]
    fn template_config_1_1() {
        test_template(
            MatrixTemplate::config_1_1(),
            MatrixOfFactorSourceIds::sample_config_1_1(),
        )
    }

    #[test]
    fn template_config_1_2() {
        test_template(
            MatrixTemplate::config_1_2(),
            MatrixOfFactorSourceIds::sample_config_1_2(),
        )
    }

    #[test]
    fn template_config_1_3() {
        test_template(
            MatrixTemplate::config_1_3(),
            MatrixOfFactorSourceIds::sample_config_1_3(),
        )
    }

    #[test]
    fn template_config_1_4() {
        test_template(
            MatrixTemplate::config_1_4(),
            MatrixOfFactorSourceIds::sample_config_1_4(),
        )
    }

    #[test]
    fn template_config_1_5() {
        test_template(
            MatrixTemplate::config_1_5(),
            MatrixOfFactorSourceIds::sample_config_1_5(),
        )
    }

    #[test]
    fn template_config_2_1() {
        test_template(
            MatrixTemplate::config_2_1(),
            MatrixOfFactorSourceIds::sample_config_2_1(),
        )
    }

    #[test]
    fn template_config_2_2() {
        test_template(
            MatrixTemplate::config_2_2(),
            MatrixOfFactorSourceIds::sample_config_2_2(),
        )
    }

    #[test]
    fn template_config_2_3() {
        test_template(
            MatrixTemplate::config_2_3(),
            MatrixOfFactorSourceIds::sample_config_2_3(),
        )
    }

    #[test]
    fn template_config_2_4() {
        test_template(
            MatrixTemplate::config_2_4(),
            MatrixOfFactorSourceIds::sample_config_2_4(),
        )
    }

    #[test]
    fn template_config_3_0() {
        test_template(
            MatrixTemplate::config_3_0(),
            MatrixOfFactorSourceIds::sample_config_3_0(),
        )
    }

    #[test]
    fn template_config_4_0() {
        test_template(
            MatrixTemplate::config_4_0(),
            MatrixOfFactorSourceIds::sample_config_4_0(),
        )
    }

    #[test]
    fn template_config_5_1() {
        test_template(
            MatrixTemplate::config_5_1(),
            MatrixOfFactorSourceIds::sample_config_5_1(),
        )
    }

    #[test]
    fn template_config_5_2() {
        test_template(
            MatrixTemplate::config_5_2(),
            MatrixOfFactorSourceIds::sample_config_5_2(),
        )
    }

    #[test]
    fn template_config_6_0() {
        test_template(
            MatrixTemplate::config_6_0(),
            MatrixOfFactorSourceIds::sample_config_6_0(),
        )
    }

    #[test]
    fn template_config_7_0() {
        test_template(
            MatrixTemplate::config_7_0(),
            MatrixOfFactorSourceIds::sample_config_7_0(),
        )
    }

    #[test]
    fn template_config_8_0() {
        test_template(
            MatrixTemplate::config_8_0(),
            MatrixOfFactorSourceIds::sample_config_8_0(),
        )
    }

    #[test]
    fn template_config_9_0() {
        test_template(
            MatrixTemplate::config_9_0(),
            MatrixOfFactorSourceIds::sample_config_9_0(),
        )
    }
}
