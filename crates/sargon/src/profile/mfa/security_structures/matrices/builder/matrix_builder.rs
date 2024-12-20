#![allow(clippy::new_without_default)]

use crate::prelude::*;

pub type MatrixBuilderMutateResult = Result<(), MatrixBuilderValidation>;

pub type MatrixBuilderBuildResult =
    Result<MatrixOfFactorSourceIds, MatrixBuilderValidation>;

/// A builder of MatrixOfFactorSourceIds, consists of role builders:
/// * PrimaryRoleBuilder
/// * RecoveryRoleBuilder
/// * ConfirmationRoleBuilder
///
/// And `number_of_days_until_auto_confirm`.
pub type MatrixBuilder = AbstractMatrixBuilderOrBuilt<
    IS_MATRIX_BUILDER,
    IS_ROLE_BUILDER,
    FactorSourceID,
>;

// ==================
// ===== PUBLIC =====
// ==================
impl MatrixBuilder {
    pub fn new() -> Self {
        Self {
            primary_role: PrimaryRoleBuilder::new(),
            recovery_role: RecoveryRoleBuilder::new(),
            confirmation_role: ConfirmationRoleBuilder::new(),
            number_of_days_until_auto_confirm:
                Self::DEFAULT_NUMBER_OF_DAYS_UNTIL_AUTO_CONFIRM,
        }
    }

    /// Validates each role in isolation and all roles in combination.
    ///
    /// If valid it returns a "built" `MatrixOfFactorSourceIds`.
    pub fn build(&self) -> MatrixBuilderBuildResult {
        self.validate_combination()?;

        let primary = self
            .primary_role
            .build()
            .into_matrix_err(RoleKind::Primary)?;
        let recovery = self
            .recovery_role
            .build()
            .into_matrix_err(RoleKind::Recovery)?;
        let confirmation = self
            .confirmation_role
            .build()
            .into_matrix_err(RoleKind::Confirmation)?;

        let built = unsafe {
            // Looks a bit odd, but yeah here is in fact the only place we
            // do build! The ctor is named like that so that it is clear that
            // when used elsewhere, it is not guaranteed to have been properly
            // built.
            MatrixOfFactorSourceIds::unbuilt_with_roles_and_days(
                primary,
                recovery,
                confirmation,
                self.number_of_days_until_auto_confirm,
            )
        };
        Ok(built)
    }

    pub fn validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> RoleBuilderMutateResult {
        self.primary_role
            .validation_for_addition_of_factor_source_of_kind_to_threshold(
                factor_source_kind,
            )
    }

    pub fn validation_for_addition_of_factor_source_of_kind_to_primary_override(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> RoleBuilderMutateResult {
        self.primary_role
            .validation_for_addition_of_factor_source_of_kind_to_override(
                factor_source_kind,
            )
    }

    pub fn validation_for_addition_of_factor_source_to_primary_threshold_for_each(
        &self,
        factor_sources: &IndexSet<FactorSourceID>,
    ) -> IndexSet<FactorSourceInRoleBuilderValidationStatus> {
        self.validation_for_addition_of_factor_source_to_primary_threshold_for_each_with_mode(factor_sources, SecurityShieldBuilderMode::Strict)
    }

    pub fn validation_for_addition_of_factor_source_to_primary_threshold_for_each_with_mode(
        &self,
        factor_sources: &IndexSet<FactorSourceID>,
        mode: SecurityShieldBuilderMode,
    ) -> IndexSet<FactorSourceInRoleBuilderValidationStatus> {
        self.primary_role
            .validation_for_addition_of_factor_source_for_each_with_mode(
                FactorListKind::Threshold,
                factor_sources,
                mode,
            )
    }

    pub fn validation_for_addition_of_factor_source_to_primary_override_for_each(
        &self,
        factor_sources: &IndexSet<FactorSourceID>,
    ) -> IndexSet<FactorSourceInRoleBuilderValidationStatus> {
        self.validation_for_addition_of_factor_source_to_primary_override_for_each_with_mode(factor_sources, SecurityShieldBuilderMode::Strict)
    }

    pub fn validation_for_addition_of_factor_source_to_primary_override_for_each_with_mode(
        &self,
        factor_sources: &IndexSet<FactorSourceID>,
        mode: SecurityShieldBuilderMode,
    ) -> IndexSet<FactorSourceInRoleBuilderValidationStatus> {
        self.primary_role
            .validation_for_addition_of_factor_source_for_each_with_mode(
                FactorListKind::Override,
                factor_sources,
                mode,
            )
    }

    pub fn validation_for_addition_of_factor_source_of_kind_to_recovery_override(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> RoleBuilderMutateResult {
        self.validation_for_addition_of_factor_source_of_kind_to_recovery_override_with_mode(factor_source_kind, SecurityShieldBuilderMode::Strict)
    }

    pub fn validation_for_addition_of_factor_source_of_kind_to_recovery_override_with_mode(
        &self,
        factor_source_kind: FactorSourceKind,
        mode: SecurityShieldBuilderMode,
    ) -> RoleBuilderMutateResult {
        self.recovery_role
            .validation_for_addition_of_factor_source_of_kind_to_override_with_mode(
                factor_source_kind,
                mode
            )
    }

    pub fn validation_for_addition_of_factor_source_to_recovery_override_for_each(
        &self,
        factor_sources: &IndexSet<FactorSourceID>,
    ) -> IndexSet<FactorSourceInRoleBuilderValidationStatus> {
        self.validation_for_addition_of_factor_source_to_recovery_override_for_each_with_mode(factor_sources, SecurityShieldBuilderMode::Strict)
    }

    pub fn validation_for_addition_of_factor_source_to_recovery_override_for_each_with_mode(
        &self,
        factor_sources: &IndexSet<FactorSourceID>,
        mode: SecurityShieldBuilderMode,
    ) -> IndexSet<FactorSourceInRoleBuilderValidationStatus> {
        self.recovery_role
            .validation_for_addition_of_factor_source_for_each_with_mode(
                FactorListKind::Override,
                factor_sources,
                mode,
            )
    }

    pub fn validation_for_addition_of_factor_source_of_kind_to_confirmation_override(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> RoleBuilderMutateResult {
        self.validation_for_addition_of_factor_source_of_kind_to_confirmation_override_with_mode(factor_source_kind, SecurityShieldBuilderMode::Strict)
    }

    pub fn validation_for_addition_of_factor_source_of_kind_to_confirmation_override_with_mode(
        &self,
        factor_source_kind: FactorSourceKind,
        mode: SecurityShieldBuilderMode,
    ) -> RoleBuilderMutateResult {
        self.confirmation_role
            .validation_for_addition_of_factor_source_of_kind_to_override_with_mode(
                factor_source_kind,
                mode
            )
    }

    pub fn validation_for_addition_of_factor_source_to_confirmation_override_for_each(
        &self,
        factor_sources: &IndexSet<FactorSourceID>,
    ) -> IndexSet<FactorSourceInRoleBuilderValidationStatus> {
        self.validation_for_addition_of_factor_source_to_confirmation_override_for_each_with_mode(factor_sources, SecurityShieldBuilderMode::Strict)
    }

    pub fn validation_for_addition_of_factor_source_to_confirmation_override_for_each_with_mode(
        &self,
        factor_sources: &IndexSet<FactorSourceID>,
        mode: SecurityShieldBuilderMode,
    ) -> IndexSet<FactorSourceInRoleBuilderValidationStatus> {
        self.confirmation_role
            .validation_for_addition_of_factor_source_for_each_with_mode(
                FactorListKind::Override,
                factor_sources,
                mode,
            )
    }

    pub fn validate_primary_role_in_isolation(
        &self,
    ) -> MatrixBuilderMutateResult {
        self.primary_role
            .validate()
            .into_matrix_err(RoleKind::Primary)?;
        Ok(())
    }

    pub fn validate_recovery_role_in_isolation(
        &self,
    ) -> MatrixBuilderMutateResult {
        self.recovery_role
            .validate()
            .into_matrix_err(RoleKind::Recovery)?;
        Ok(())
    }

    pub fn validate_confirmation_role_in_isolation(
        &self,
    ) -> MatrixBuilderMutateResult {
        self.confirmation_role
            .validate()
            .into_matrix_err(RoleKind::Confirmation)?;
        Ok(())
    }

    fn validate_each_role_in_isolation(&self) -> MatrixBuilderMutateResult {
        self.validate_primary_role_in_isolation()?;
        self.validate_recovery_role_in_isolation()?;
        self.validate_confirmation_role_in_isolation()?;
        Ok(())
    }

    pub fn validate(&self) -> MatrixBuilderMutateResult {
        self.validate_each_role_in_isolation()?;
        self.validate_combination()?;
        Ok(())
    }

    /// Adds the factor source to the primary role threshold list.
    ///
    /// Also sets the threshold to 1 this is the first factor set and if
    /// the threshold was 0.
    pub fn add_factor_source_to_primary_threshold(
        &mut self,
        factor_source_id: FactorSourceID,
    ) -> MatrixBuilderMutateResult {
        self.add_factor_source_to_primary_threshold_with_mode(
            factor_source_id,
            SecurityShieldBuilderMode::Strict,
        )
    }

    pub fn add_factor_source_to_primary_threshold_with_mode(
        &mut self,
        factor_source_id: FactorSourceID,
        mode: SecurityShieldBuilderMode,
    ) -> MatrixBuilderMutateResult {
        self.primary_role
            .add_factor_source_to_threshold_with_mode(factor_source_id, mode)
            .into_matrix_err(RoleKind::Primary)
    }

    pub fn reset_recovery_and_confirmation_role_state(&mut self) {
        self.recovery_role.reset();
        self.confirmation_role.reset();
    }

    pub fn reset_factors_in_roles(&mut self) {
        self.reset_recovery_and_confirmation_role_state();
        self.primary_role.reset();
    }

    /// Adds the factor source to the primary role override list.
    pub fn add_factor_source_to_primary_override(
        &mut self,
        factor_source_id: FactorSourceID,
    ) -> MatrixBuilderMutateResult {
        self.add_factor_source_to_primary_override_with_mode(
            factor_source_id,
            SecurityShieldBuilderMode::Strict,
        )
    }

    /// Adds the factor source to the primary role override list.
    pub fn add_factor_source_to_primary_override_with_mode(
        &mut self,
        factor_source_id: FactorSourceID,
        mode: SecurityShieldBuilderMode,
    ) -> MatrixBuilderMutateResult {
        self.primary_role
            .add_factor_source_to_override_with_mode(factor_source_id, mode)
            .into_matrix_err(RoleKind::Primary)
    }

    pub fn add_factor_source_to_recovery_override(
        &mut self,
        factor_source_id: FactorSourceID,
    ) -> MatrixBuilderMutateResult {
        self.add_factor_source_to_recovery_override_with_mode(
            factor_source_id,
            SecurityShieldBuilderMode::Strict,
        )
    }

    pub fn add_factor_source_to_recovery_override_with_mode(
        &mut self,
        factor_source_id: FactorSourceID,
        mode: SecurityShieldBuilderMode,
    ) -> MatrixBuilderMutateResult {
        self.recovery_role
            .add_factor_source_to_override_with_mode(factor_source_id, mode)
            .into_matrix_err(RoleKind::Recovery)
    }

    pub fn add_factor_source_to_confirmation_override(
        &mut self,
        factor_source_id: FactorSourceID,
    ) -> MatrixBuilderMutateResult {
        self.add_factor_source_to_confirmation_override_with_mode(
            factor_source_id,
            SecurityShieldBuilderMode::Strict,
        )
    }

    pub fn add_factor_source_to_confirmation_override_with_mode(
        &mut self,
        factor_source_id: FactorSourceID,
        mode: SecurityShieldBuilderMode,
    ) -> MatrixBuilderMutateResult {
        self.confirmation_role
            .add_factor_source_to_override_with_mode(factor_source_id, mode)
            .into_matrix_err(RoleKind::Confirmation)
    }

    pub fn get_confirmation_factors(&self) -> &Vec<FactorSourceID> {
        self.confirmation_role.get_override_factors()
    }

    pub fn get_recovery_factors(&self) -> &Vec<FactorSourceID> {
        self.recovery_role.get_override_factors()
    }

    pub fn get_primary_threshold_factors(&self) -> &Vec<FactorSourceID> {
        self.primary_role.get_threshold_factors()
    }

    pub fn get_primary_override_factors(&self) -> &Vec<FactorSourceID> {
        self.primary_role.get_override_factors()
    }

    /// Sets the threshold on the primary role builder.
    pub fn set_threshold(
        &mut self,
        threshold: u8,
    ) -> MatrixBuilderMutateResult {
        self.primary_role
            .set_threshold(threshold)
            .into_matrix_err(RoleKind::Primary)
    }

    pub fn get_threshold(&self) -> u8 {
        self.primary_role.get_threshold()
    }

    pub fn set_number_of_days_until_auto_confirm(
        &mut self,
        number_of_days: u16,
    ) -> MatrixBuilderMutateResult {
        self.number_of_days_until_auto_confirm = number_of_days;

        self.validate_number_of_days_until_auto_confirm()
    }

    pub fn get_number_of_days_until_auto_confirm(&self) -> u16 {
        self.number_of_days_until_auto_confirm
    }

    fn remove_factor_from_role<const ROLE: u8>(
        role: &mut RoleBuilder<{ ROLE }>,
        factor_source_id: &FactorSourceID,
    ) -> MatrixBuilderMutateResult {
        if role.remove_factor_source(factor_source_id).is_ok() {
            Ok(())
        } else {
            MatrixBuilderMutateResult::Err(MatrixBuilderValidation::CombinationViolation(
                MatrixRolesInCombinationViolation::Basic(
                    MatrixRolesInCombinationBasicViolation::FactorSourceNotFoundInAnyRole,
                ),
            ))
        }
    }

    pub fn remove_factor_from_primary(
        &mut self,
        factor_source_id: &FactorSourceID,
    ) -> MatrixBuilderMutateResult {
        Self::remove_factor_from_role(&mut self.primary_role, factor_source_id)
    }

    pub fn remove_factor_from_recovery(
        &mut self,
        factor_source_id: &FactorSourceID,
    ) -> MatrixBuilderMutateResult {
        Self::remove_factor_from_role(&mut self.recovery_role, factor_source_id)
    }

    pub fn remove_factor_from_confirmation(
        &mut self,
        factor_source_id: &FactorSourceID,
    ) -> MatrixBuilderMutateResult {
        Self::remove_factor_from_role(
            &mut self.confirmation_role,
            factor_source_id,
        )
    }

    /// Removes `factor_source_id` from all three roles, if not found in any an error
    /// is thrown.
    ///
    /// # Throws
    /// If none of the three role builders contains the factor source id, `Err(BasicViolation::FactorSourceNotFound)` is thrown
    pub fn remove_factor_from_all_roles(
        &mut self,
        factor_source_id: &FactorSourceID,
    ) -> MatrixBuilderMutateResult {
        let fsid = factor_source_id;
        let r0 = self.remove_factor_from_primary(fsid);
        let r1 = self.remove_factor_from_recovery(fsid);
        let r2 = self.remove_factor_from_confirmation(fsid);
        r0.or(r1).or(r2)
    }
}

// ==================
// ==== PRIVATE =====
// ==================
impl MatrixBuilder {
    fn validate_if_primary_has_single_it_must_not_be_used_by_any_other_role(
        &self,
    ) -> MatrixBuilderMutateResult {
        let primary_has_single_factor =
            self.primary_role.all_factors().len() == 1;
        if primary_has_single_factor {
            let primary_factors = self.primary_role.all_factors();
            let primary_factor = primary_factors.first().unwrap();
            let recovery_set = HashSet::<_>::from_iter(
                self.recovery_role.get_override_factors(),
            );
            let confirmation_set = HashSet::<_>::from_iter(
                self.confirmation_role.get_override_factors(),
            );
            if recovery_set.contains(primary_factor)
                || confirmation_set.contains(primary_factor)
            {
                return Err(MatrixBuilderValidation::CombinationViolation(
                    MatrixRolesInCombinationViolation::NotYetValid(MatrixRolesInCombinationNotYetValid::SingleFactorUsedInPrimaryMustNotBeUsedInAnyOtherRole),
                ));
            }
        }
        Ok(())
    }

    fn validate_no_factor_may_be_used_in_both_recovery_and_confirmation(
        &self,
    ) -> MatrixBuilderMutateResult {
        let recovery_set =
            HashSet::<_>::from_iter(self.recovery_role.get_override_factors());
        let confirmation_set = HashSet::<_>::from_iter(
            self.confirmation_role.get_override_factors(),
        );
        let intersection = recovery_set
            .intersection(&confirmation_set)
            .collect::<HashSet<_>>();
        if intersection.is_empty() {
            Ok(())
        } else {
            Err(MatrixBuilderValidation::CombinationViolation(
                MatrixRolesInCombinationViolation::ForeverInvalid(
                    MatrixRolesInCombinationForeverInvalid::RecoveryAndConfirmationFactorsOverlap,
                ),
            ))
        }
    }

    fn validate_number_of_days_until_auto_confirm(
        &self,
    ) -> MatrixBuilderMutateResult {
        if self.number_of_days_until_auto_confirm == 0 {
            return Err(MatrixBuilderValidation::CombinationViolation(
                MatrixRolesInCombinationViolation::Basic(
                    MatrixRolesInCombinationBasicViolation::NumberOfDaysUntilAutoConfirmMustBeGreaterThanZero,
                ),
            ));
        }
        Ok(())
    }

    /// Security Shield Rules
    /// In addition to the factor/role rules above, the wallet must enforce certain rules for combinations of
    /// factors across the three roles. The construction method described in the next section will automatically
    /// always follow these rules. A user may however choose to manually add/remove factors from their Shield
    /// configuration and so the wallet must evaluate these rules and inform the user when the combination they
    /// have chosen cannot be used. The wallet should never allow a user to complete a Shield configuration that
    /// violates these rules.
    ///
    /// 1. If only one factor is used for `Primary`, that factor may not be used for either `Recovery` or `Confirmation`
    /// 2. No factor may be used (override) in both `Recovery` and `Confirmation`
    /// 3. No factor may be used in both the `Primary` threshold and `Primary` override
    /// 4. Number of days until auto confirm is greater than zero
    fn validate_combination(&self) -> MatrixBuilderMutateResult {
        self.validate_if_primary_has_single_it_must_not_be_used_by_any_other_role()?;
        self.validate_no_factor_may_be_used_in_both_recovery_and_confirmation(
        )?;

        // N.B. the third 3:
        // "3. No factor may be used in both the `Primary` threshold and `Primary` override"
        // is already enforced by the RoleBuilder

        self.validate_number_of_days_until_auto_confirm()?;
        Ok(())
    }
}
