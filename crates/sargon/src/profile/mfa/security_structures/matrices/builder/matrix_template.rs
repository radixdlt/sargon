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
        Ok(RoleWithFactorSourceIds::with_factors(
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
            Err(CommonError::Unknown)
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
        let number_of_days_until_auto_confirm =
            self.number_of_days_until_auto_confirm;
        let mut assigner = FactorSourceIdAssigner::new(factor_source_ids);
        let primary_role = self.primary_role.assign(&mut assigner)?;
        let recovery_role = self.recovery_role.assign(&mut assigner)?;
        let confirmation_role = self.confirmation_role.assign(&mut assigner)?;

        Ok(MatrixOfFactorSourceIds {
            built: PhantomData,
            primary_role,
            recovery_role,
            confirmation_role,
            number_of_days_until_auto_confirm,
        })
    }
}

impl MatrixTemplate {
    fn new(
        primary_role: PrimaryRoleTemplate,
        recovery_role: RecoveryRoleTemplate,
        confirmation_role: ConfirmationRoleTemplate,
    ) -> Self {
        Self {
            built: PhantomData,
            primary_role,
            recovery_role,
            confirmation_role,
            number_of_days_until_auto_confirm:
                MatrixOfFactorSourceIds::DEFAULT_NUMBER_OF_DAYS_UNTIL_AUTO_CONFIRM,
        }
    }

    pub fn config_11() -> Self {
        Self::new(
            PrimaryRoleTemplate::new([
                FactorSourceTemplate::device(),
                FactorSourceTemplate::ledger(),
            ]),
            RecoveryRoleTemplate::new([
                FactorSourceTemplate::device(),
                FactorSourceTemplate::ledger(),
            ]),
            ConfirmationRoleTemplate::new([FactorSourceTemplate::password()]),
        )
    }

    pub fn config_12() -> Self {
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

    pub fn config_13() -> Self {
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

    pub fn config_14() -> Self {
        Self::new(
            PrimaryRoleTemplate::new([FactorSourceTemplate::device()]),
            RecoveryRoleTemplate::new([FactorSourceTemplate::ledger()]),
            ConfirmationRoleTemplate::new([FactorSourceTemplate::password()]),
        )
    }

    pub fn config_15() -> Self {
        Self::new(
            PrimaryRoleTemplate::new([FactorSourceTemplate::ledger()]),
            RecoveryRoleTemplate::new([FactorSourceTemplate::device()]),
            ConfirmationRoleTemplate::new([FactorSourceTemplate::password()]),
        )
    }

    pub fn config_21() -> Self {
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

    pub fn config_22() -> Self {
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

    pub fn config_23() -> Self {
        Self::new(
            PrimaryRoleTemplate::new([FactorSourceTemplate::ledger()]),
            RecoveryRoleTemplate::new([FactorSourceTemplate::ledger_other()]),
            ConfirmationRoleTemplate::new([FactorSourceTemplate::device()]),
        )
    }

    pub fn config_24() -> Self {
        Self::new(
            PrimaryRoleTemplate::new([FactorSourceTemplate::device()]),
            RecoveryRoleTemplate::new([FactorSourceTemplate::ledger()]),
            ConfirmationRoleTemplate::new([
                FactorSourceTemplate::ledger_other(),
            ]),
        )
    }

    pub fn config_30() -> Self {
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

    pub fn config_40() -> Self {
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

    pub fn config_51() -> Self {
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

    pub fn config_52() -> Self {
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

    pub fn config_60() -> Self {
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

    pub fn config_70() -> Self {
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

    pub fn config_80() -> Self {
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

    pub fn config_90() -> Self {
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
    fn template_config_11() {
        test_template(
            MatrixTemplate::config_11(),
            MatrixOfFactorSourceIds::sample_config_11(),
        )
    }

    #[test]
    fn template_config_12() {
        test_template(
            MatrixTemplate::config_12(),
            MatrixOfFactorSourceIds::sample_config_12(),
        )
    }

    #[test]
    fn template_config_13() {
        test_template(
            MatrixTemplate::config_13(),
            MatrixOfFactorSourceIds::sample_config_13(),
        )
    }

    #[test]
    fn template_config_14() {
        test_template(
            MatrixTemplate::config_14(),
            MatrixOfFactorSourceIds::sample_config_14(),
        )
    }

    #[test]
    fn template_config_15() {
        test_template(
            MatrixTemplate::config_15(),
            MatrixOfFactorSourceIds::sample_config_15(),
        )
    }

    #[test]
    fn template_config_21() {
        test_template(
            MatrixTemplate::config_21(),
            MatrixOfFactorSourceIds::sample_config_21(),
        )
    }

    #[test]
    fn template_config_22() {
        test_template(
            MatrixTemplate::config_22(),
            MatrixOfFactorSourceIds::sample_config_22(),
        )
    }

    #[test]
    fn template_config_23() {
        test_template(
            MatrixTemplate::config_23(),
            MatrixOfFactorSourceIds::sample_config_23(),
        )
    }

    #[test]
    fn template_config_24() {
        test_template(
            MatrixTemplate::config_24(),
            MatrixOfFactorSourceIds::sample_config_24(),
        )
    }

    #[test]
    fn template_config_30() {
        test_template(
            MatrixTemplate::config_30(),
            MatrixOfFactorSourceIds::sample_config_30(),
        )
    }

    #[test]
    fn template_config_40() {
        test_template(
            MatrixTemplate::config_40(),
            MatrixOfFactorSourceIds::sample_config_40(),
        )
    }

    #[test]
    fn template_config_51() {
        test_template(
            MatrixTemplate::config_51(),
            MatrixOfFactorSourceIds::sample_config_51(),
        )
    }

    #[test]
    fn template_config_52() {
        test_template(
            MatrixTemplate::config_52(),
            MatrixOfFactorSourceIds::sample_config_52(),
        )
    }

    #[test]
    fn template_config_60() {
        test_template(
            MatrixTemplate::config_60(),
            MatrixOfFactorSourceIds::sample_config_60(),
        )
    }

    #[test]
    fn template_config_70() {
        test_template(
            MatrixTemplate::config_70(),
            MatrixOfFactorSourceIds::sample_config_70(),
        )
    }

    #[test]
    fn template_config_80() {
        test_template(
            MatrixTemplate::config_80(),
            MatrixOfFactorSourceIds::sample_config_80(),
        )
    }

    #[test]
    fn template_config_90() {
        test_template(
            MatrixTemplate::config_90(),
            MatrixOfFactorSourceIds::sample_config_90(),
        )
    }
}
