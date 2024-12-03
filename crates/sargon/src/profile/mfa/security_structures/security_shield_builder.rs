use crate::prelude::*;

#[derive(Debug)]
pub struct SecurityShieldBuilder {
    matrix_builder: RwLock<MatrixBuilder>,
    name: RwLock<String>,
    // We eagerly set this, and we use it inside the `build` method, ensuring
    // that for the same *state* of `MatrixBuilder` we always have the same shield!
    shield_id: SecurityStructureID,
    // We eagerly set this, and we use it inside the `build` method, ensuring
    // that for the same *state* of `MatrixBuilder` we always have the same shield!
    created_on: Timestamp,
}

impl SecurityShieldBuilder {
    pub fn new() -> Self {
        let matrix_builder = MatrixBuilder::new();
        let name = RwLock::new("My Shield".to_owned());
        Self {
            matrix_builder: RwLock::new(matrix_builder),
            name,
            shield_id: SecurityStructureID::from(id()),
            created_on: now(),
        }
    }
}

impl SecurityShieldBuilder {
    fn get<R>(&self, access: impl Fn(&MatrixBuilder) -> R) -> R {
        let binding = self.matrix_builder.read().unwrap();
        access(&binding)
    }

    // Ignores error and returns a ref to self
    fn set<R>(&self, mut write: impl FnMut(&mut MatrixBuilder) -> R) -> &Self {
        let mut binding = self.matrix_builder.write().expect("No poison");
        write(&mut binding);
        self
    }

    fn validation_for_addition_of_factor_source_by_calling(
        &self,
        factor_sources: Vec<FactorSourceID>,
        call: impl Fn(
            &MatrixBuilder,
            &IndexSet<FactorSourceID>,
        )
            -> IndexSet<FactorSourceInRoleBuilderValidationStatus>,
    ) -> Vec<FactorSourceInRoleBuilderValidationStatus> {
        let input = factor_sources
            .iter()
            .map(|x| x.clone().into())
            .collect::<IndexSet<_>>();
        self.get(|builder| call(builder, &input))
            .into_iter()
            .collect_vec()
    }
}

impl SecurityShieldBuilder {
    fn get_factors(
        &self,
        access: impl Fn(&MatrixBuilder) -> &Vec<FactorSourceID>,
    ) -> Vec<FactorSourceID> {
        self.get(|builder| {
            let factors = access(builder);
            factors
                .iter()
                .map(|x| crate::FactorSourceID::from(*x))
                .collect::<Vec<_>>()
        })
    }
}

// ====================
// ==== GET / READ ====
// ====================
impl SecurityShieldBuilder {
    pub fn get_threshold(&self) -> u8 {
        self.get(|builder| builder.get_threshold())
    }

    pub fn get_number_of_days_until_auto_confirm(&self) -> u16 {
        self.get(|builder| builder.get_number_of_days_until_auto_confirm())
    }

    pub fn get_name(&self) -> String {
        self.name.read().unwrap().clone()
    }

    pub fn get_primary_threshold_factors(&self) -> Vec<FactorSourceID> {
        self.get_factors(|builder| builder.get_primary_threshold_factors())
    }

    pub fn get_primary_override_factors(&self) -> Vec<FactorSourceID> {
        self.get_factors(|builder| builder.get_primary_override_factors())
    }

    pub fn get_recovery_factors(&self) -> Vec<FactorSourceID> {
        self.get_factors(|builder| builder.get_recovery_factors())
    }

    pub fn get_confirmation_factors(&self) -> Vec<FactorSourceID> {
        self.get_factors(|builder| builder.get_confirmation_factors())
    }
}

// ====================
// ===== MUTATION =====
// ====================
impl SecurityShieldBuilder {
    pub fn set_name(&self, name: impl AsRef<str>) -> &Self {
        *self.name.write().unwrap() = name.as_ref().to_owned();
        self
    }

    /// Adds the factor source to the primary role threshold list.
    pub fn add_factor_source_to_primary_threshold(
        &self,
        factor_source_id: FactorSourceID,
    ) -> &Self {
        self.set(|builder| {
            builder.add_factor_source_to_primary_threshold(
                factor_source_id.clone().into(),
            )
        })
    }

    pub fn add_factor_source_to_primary_override(
        &self,
        factor_source_id: FactorSourceID,
    ) -> &Self {
        self.set(|builder| {
            builder.add_factor_source_to_primary_override(
                factor_source_id.clone().into(),
            )
        })
    }

    pub fn remove_factor(&self, factor_source_id: FactorSourceID) -> &Self {
        self.set(|builder| {
            builder.remove_factor(&factor_source_id.clone().into())
        })
    }

    pub fn set_threshold(&self, threshold: u8) -> &Self {
        self.set(|builder| builder.set_threshold(threshold))
    }

    pub fn set_number_of_days_until_auto_confirm(
        &self,
        number_of_days: u16,
    ) -> &Self {
        self.set(|builder| {
            builder.set_number_of_days_until_auto_confirm(number_of_days)
        })
    }

    pub fn add_factor_source_to_recovery_override(
        &self,
        factor_source_id: FactorSourceID,
    ) -> &Self {
        self.set(|builder| {
            builder.add_factor_source_to_recovery_override(
                factor_source_id.clone().into(),
            )
        })
    }

    pub fn add_factor_source_to_confirmation_override(
        &self,
        factor_source_id: FactorSourceID,
    ) -> &Self {
        self.set(|builder| {
            builder.add_factor_source_to_confirmation_override(
                factor_source_id.clone().into(),
            )
        })
    }
}

impl SecurityShieldBuilder {
    fn _validation_for_addition_of_factor_source_of_kind_to_confirmation_override(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> RoleBuilderMutateResult {
        self.get(|builder| {
            builder.validation_for_addition_of_factor_source_of_kind_to_confirmation_override(
                factor_source_kind
            )
        })
    }

    fn _validation_for_addition_of_factor_source_of_kind_to_recovery_override(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> RoleBuilderMutateResult {
        self.get(|builder| {
            builder.validation_for_addition_of_factor_source_of_kind_to_recovery_override(
                factor_source_kind
            )
        })
    }

    fn _validation_for_addition_of_factor_source_of_kind_to_primary_override(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> RoleBuilderMutateResult {
        self.get(|builder| {
            builder.validation_for_addition_of_factor_source_of_kind_to_primary_override(
                factor_source_kind
            )
        })
    }

    fn _validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> RoleBuilderMutateResult {
        self.get(|builder| {
            builder.validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
                factor_source_kind
            )
        })
    }
}

impl SecurityShieldBuilder {
    /// Returns `true` for `Ok` and `Err(NotYetValid)`.
    pub fn addition_of_factor_source_of_kind_to_primary_threshold_is_valid_or_can_be(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> bool {
        self._validation_for_addition_of_factor_source_of_kind_to_primary_threshold(factor_source_kind).is_valid_or_can_be()
    }

    /// Returns `true` for `Ok` and `Err(NotYetValid)`.
    pub fn addition_of_factor_source_of_kind_to_primary_override_is_valid_or_can_be(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> bool {
        self._validation_for_addition_of_factor_source_of_kind_to_primary_override(factor_source_kind).is_valid_or_can_be()
    }

    /// Returns `true` for `Ok` and `Err(NotYetValid)`.
    pub fn addition_of_factor_source_of_kind_to_recovery_is_valid_or_can_be(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> bool {
        self._validation_for_addition_of_factor_source_of_kind_to_recovery_override(factor_source_kind).is_valid_or_can_be()
    }

    /// Returns `true` for `Ok` and `Err(NotYetValid)`.
    pub fn addition_of_factor_source_of_kind_to_confirmation_is_valid_or_can_be(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> bool {
        self._validation_for_addition_of_factor_source_of_kind_to_confirmation_override(factor_source_kind).is_valid_or_can_be()
    }
}

pub trait IsValidOrCanBecomeValid {
    fn is_valid_or_can_be(&self) -> bool;
}
impl<T> IsValidOrCanBecomeValid for Result<T, RoleBuilderValidation> {
    fn is_valid_or_can_be(&self) -> bool {
        match self {
            Ok(_) => true,
            Err(RoleBuilderValidation::BasicViolation(_))
            | Err(RoleBuilderValidation::ForeverInvalid(_)) => false,
            Err(RoleBuilderValidation::NotYetValid(_)) => true,
        }
    }
}

impl SecurityShieldBuilder {
    pub fn addition_of_factor_source_of_kind_to_primary_threshold_is_fully_valid(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> bool {
        self._validation_for_addition_of_factor_source_of_kind_to_primary_threshold(factor_source_kind)
        .is_ok()
    }

    pub fn addition_of_factor_source_of_kind_to_primary_override_is_fully_valid(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> bool {
        self._validation_for_addition_of_factor_source_of_kind_to_primary_override(factor_source_kind)
        .is_ok()
    }

    pub fn addition_of_factor_source_of_kind_to_recovery_is_fully_valid(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> bool {
        self._validation_for_addition_of_factor_source_of_kind_to_recovery_override(factor_source_kind)
        .is_ok()
    }

    pub fn addition_of_factor_source_of_kind_to_confirmation_is_fully_valid(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> bool {
        self._validation_for_addition_of_factor_source_of_kind_to_confirmation_override(factor_source_kind)
        .is_ok()
    }
}

impl SecurityShieldBuilder {
    pub fn validation_for_addition_of_factor_source_to_primary_threshold_for_each(
        &self,
        factor_sources: Vec<FactorSourceID>,
    ) -> Vec<FactorSourceInRoleBuilderValidationStatus> {
        self.validation_for_addition_of_factor_source_by_calling(
            factor_sources,
            |builder, input| {
                builder.validation_for_addition_of_factor_source_to_primary_threshold_for_each(input)
            },
        )
    }

    pub fn validation_for_addition_of_factor_source_to_primary_override_for_each(
        &self,
        factor_sources: Vec<FactorSourceID>,
    ) -> Vec<FactorSourceInRoleBuilderValidationStatus> {
        self.validation_for_addition_of_factor_source_by_calling(
            factor_sources,
            |builder, input| {
                builder.validation_for_addition_of_factor_source_to_primary_override_for_each(input)
            },
        )
    }

    pub fn validation_for_addition_of_factor_source_to_recovery_override_for_each(
        &self,
        factor_sources: Vec<FactorSourceID>,
    ) -> Vec<FactorSourceInRoleBuilderValidationStatus> {
        self.validation_for_addition_of_factor_source_by_calling(
            factor_sources,
            |builder, input| {
                builder
                    .validation_for_addition_of_factor_source_to_recovery_override_for_each(input)
            },
        )
    }

    pub fn validation_for_addition_of_factor_source_to_confirmation_override_for_each(
        &self,
        factor_sources: Vec<FactorSourceID>,
    ) -> Vec<FactorSourceInRoleBuilderValidationStatus> {
        self.validation_for_addition_of_factor_source_by_calling(
            factor_sources,
            |builder, input| {
                builder.validation_for_addition_of_factor_source_to_confirmation_override_for_each(
                    input,
                )
            },
        )
    }
}

impl SecurityShieldBuilder {
    /// `None` means valid!
    pub fn validate(&self) -> Option<SecurityShieldBuilderInvalidReason> {
        self.get(|builder| {
            let r = builder.validate();
            r.as_shield_validation()
        })
    }

    pub fn build(
        &self,
    ) -> Result<
        SecurityStructureOfFactorSourceIds,
        SecurityShieldBuilderInvalidReason,
    > {
        let matrix_result = self.get(|builder| builder.build());

        if let Some(validation_error) = matrix_result.as_shield_validation() {
            return Err(validation_error);
        };
        assert!(
            matrix_result.is_ok(),
            "Programmer error, bad implementation of `into_validation`"
        );
        let matrix_of_factors = matrix_result.unwrap();

        let name = self.get_name();
        let display_name = DisplayName::new(name).map_err(|e| {
            error!("Invalid DisplayName {:?}", e);
            SecurityShieldBuilderInvalidReason::ShieldNameInvalid
        })?;

        let metadata = SecurityStructureMetadata::with_details(
            self.shield_id,
            display_name,
            self.created_on,
            self.created_on,
        );

        let shield = SecurityStructureOfFactorSourceIds {
            matrix_of_factors,
            metadata,
        };
        Ok(shield)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityShieldBuilder;

    #[test]
    fn test() {
        let sut = SUT::new();

        let _ = sut
            .set_name("S.H.I.E.L.D.")
            // Primary
            .set_number_of_days_until_auto_confirm(42)
            .add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device(),
            )
            .set_threshold(1)
            .add_factor_source_to_primary_override(
                FactorSourceID::sample_arculus(),
            )
            .add_factor_source_to_primary_override(
                FactorSourceID::sample_arculus_other(),
            )
            // Recovery
            .add_factor_source_to_recovery_override(
                FactorSourceID::sample_ledger(),
            )
            .add_factor_source_to_recovery_override(
                FactorSourceID::sample_ledger_other(),
            )
            // Confirmation
            .add_factor_source_to_confirmation_override(
                FactorSourceID::sample_device(),
            )
            .remove_factor(FactorSourceID::sample_arculus_other())
            .remove_factor(FactorSourceID::sample_ledger_other());

        let shield0 = sut.build().unwrap();
        let shield = sut.build().unwrap();
        pretty_assertions::assert_eq!(shield0, shield);

        assert_eq!(shield.metadata.display_name.value, "S.H.I.E.L.D.");
        assert_eq!(
            shield.matrix_of_factors.primary().get_override_factors(),
            &vec![FactorSourceID::sample_arculus().into()]
        );
        assert_eq!(
            shield.matrix_of_factors.recovery().get_override_factors(),
            &vec![FactorSourceID::sample_ledger().into()]
        );
        assert_eq!(
            shield
                .matrix_of_factors
                .confirmation()
                .get_override_factors(),
            &vec![FactorSourceID::sample_device().into()]
        );
    }

    fn test_addition_of_factor_source_of_kind_to_primary(
        list_kind: FactorListKind,
        is_fully_valid: impl Fn(&SUT, FactorSourceKind) -> bool,
        can_be: impl Fn(&SUT, FactorSourceKind) -> bool,
        add: impl Fn(&SUT, FactorSourceID) -> &SUT,
    ) {
        let sut_owned = SUT::new();
        let sut = &sut_owned;
        assert!(can_be(sut, FactorSourceKind::Device));

        if list_kind == FactorListKind::Threshold {
            assert!(!is_fully_valid(sut, FactorSourceKind::Password)); // never alone
            assert!(can_be(sut, FactorSourceKind::Password)); // lenient

            // now lets adding a Device => subsequent calls to `is_fully_valid` will return false
            add(sut, FactorSourceID::sample_device());
            add(sut, FactorSourceID::sample_ledger());

            sut.set_threshold(2);
            assert!(is_fully_valid(sut, FactorSourceKind::Password)); // not alone any more!
            assert!(can_be(sut, FactorSourceKind::Password));
        } else {
            // now lets adding a Device => subsequent calls to `is_fully_valid` will return false
            add(sut, FactorSourceID::sample_device());
        }

        assert!(!is_fully_valid(sut, FactorSourceKind::Device));

        // TODO: Unsure about this, we do not count current state and query "can I add (another) Device?" as something
        // which can become valid. It would require deletion of current Device factor. Maybe we should change this?
        // Not sure... lets keep it as is for now! And lets see how UI integration "feels".
        assert!(!can_be(sut, FactorSourceKind::Device));

        // make it valid again
        sut.remove_factor(FactorSourceID::sample_device());

        assert!(is_fully_valid(sut, FactorSourceKind::Device));
        assert!(can_be(sut, FactorSourceKind::Device));
    }

    #[test]
    fn test_addition_of_factor_source_of_kind_to_primary_threshold() {
        test_addition_of_factor_source_of_kind_to_primary(
            FactorListKind::Threshold,
            SUT::addition_of_factor_source_of_kind_to_primary_threshold_is_fully_valid,
            SUT::addition_of_factor_source_of_kind_to_primary_threshold_is_valid_or_can_be,
            SUT::add_factor_source_to_primary_threshold,
        );
    }

    #[test]
    fn test_addition_of_factor_source_of_kind_to_primary_override() {
        test_addition_of_factor_source_of_kind_to_primary(
            FactorListKind::Override,
            SUT::addition_of_factor_source_of_kind_to_primary_override_is_fully_valid,
            SUT::addition_of_factor_source_of_kind_to_primary_override_is_valid_or_can_be,
            SUT::add_factor_source_to_primary_override,
        );
    }

    #[test]
    fn test_addition_of_factor_source_of_kind_to_recovery_is_fully_valid() {
        let sut = SUT::new();

        let result = sut
            .addition_of_factor_source_of_kind_to_recovery_is_fully_valid(
                FactorSourceKind::Device,
            );
        assert!(result);

        let result = sut
            .addition_of_factor_source_of_kind_to_recovery_is_fully_valid(
                FactorSourceKind::Password,
            );
        assert!(!result);
    }

    #[test]
    fn test_addition_of_factor_source_of_kind_to_confirmation_is_fully_valid() {
        let sut = SUT::new();

        let result = sut
            .addition_of_factor_source_of_kind_to_confirmation_is_fully_valid(
                FactorSourceKind::Device,
            );
        assert!(result);

        let result = sut
            .addition_of_factor_source_of_kind_to_confirmation_is_fully_valid(
                FactorSourceKind::TrustedContact,
            );
        assert!(!result);
    }

    #[test]
    fn test_validation_for_addition_of_factor_source_to_primary_threshold_for_each(
    ) {
        let sut = SUT::new();

        sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_device(),
        );

        let xs = sut.validation_for_addition_of_factor_source_to_primary_threshold_for_each(
            vec![
                FactorSourceID::sample_device(),
                FactorSourceID::sample_device_other(),
            ],
        );

        pretty_assertions::assert_eq!(
            xs.into_iter().collect::<Vec<_>>(),
            vec![
                FactorSourceInRoleBuilderValidationStatus::forever_invalid(
                    RoleKind::Primary,
                    FactorSourceID::sample_device(),
                    ForeverInvalidReason::FactorSourceAlreadyPresent
                ),
                FactorSourceInRoleBuilderValidationStatus::forever_invalid(
                    RoleKind::Primary,
                    FactorSourceID::sample_device_other(),
                    ForeverInvalidReason::PrimaryCannotHaveMultipleDevices
                ),
            ]
        );
    }

    #[test]
    fn test_validation_for_addition_of_factor_source_to_recovery_override_for_each(
    ) {
        let sut = SUT::new();

        let xs = sut.validation_for_addition_of_factor_source_to_recovery_override_for_each(
            vec![
                FactorSourceID::sample_password(),
                FactorSourceID::sample_password_other(),
                FactorSourceID::sample_security_questions(),
                FactorSourceID::sample_security_questions_other(),
            ],
        );
        pretty_assertions::assert_eq!(
            xs.into_iter().collect::<Vec<_>>(),
            [
                FactorSourceID::sample_password(),
                FactorSourceID::sample_password_other(),
                FactorSourceID::sample_security_questions(),
                FactorSourceID::sample_security_questions_other(),
            ]
            .into_iter()
            .map(
                |fsid| FactorSourceInRoleBuilderValidationStatus::forever_invalid(
                    RoleKind::Recovery,
                    fsid,
                    if fsid.get_factor_source_kind() == FactorSourceKind::SecurityQuestions {
                        ForeverInvalidReason::RecoveryRoleSecurityQuestionsNotSupported
                    } else {
                        ForeverInvalidReason::RecoveryRolePasswordNotSupported
                    }
                )
            )
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_validation_for_addition_of_factor_source_to_confirmation_override_for_each(
    ) {
        let sut = SUT::new();
        let xs = sut
            .validation_for_addition_of_factor_source_to_confirmation_override_for_each(
                vec![
                    FactorSourceID::sample_trusted_contact(),
                    FactorSourceID::sample_trusted_contact_other(),
                ],
            );
        pretty_assertions::assert_eq!(
            xs.into_iter().collect::<Vec<_>>(),
            [
                FactorSourceID::sample_trusted_contact(),
                FactorSourceID::sample_trusted_contact_other(),
            ]
            .into_iter()
            .map(
                |fsid| FactorSourceInRoleBuilderValidationStatus::forever_invalid(
                    RoleKind::Confirmation,
                    fsid,
                    ForeverInvalidReason::ConfirmationRoleTrustedContactNotSupported
                )
            )
            .collect::<Vec<_>>()
        );
    }
}
