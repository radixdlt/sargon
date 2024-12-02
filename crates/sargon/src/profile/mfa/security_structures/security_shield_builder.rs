use crate::prelude::*;

#[derive(Debug)]
pub struct SecurityShieldBuilder {
    matrix_builder: RwLock<MatrixBuilder>,
    name: RwLock<String>,
}

impl SecurityShieldBuilder {
    fn get<R>(&self, access: impl Fn(&MatrixBuilder) -> R) -> R {
        let binding = self.matrix_builder.read().unwrap();
        access(&binding)
    }

    fn with<R, E: Into<CommonError>>(
        &self,
        mut write: impl FnMut(&mut MatrixBuilder) -> Result<R, E>,
    ) -> Result<R, CommonError> {
        let mut binding = self.matrix_builder.write().expect("No poison");
        write(&mut binding).map_err(|e| Into::<CommonError>::into(e))
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
        // let input = &factor_sources
        //     .clone()
        //     .into_iter()
        //     .map(Into::<FactorSourceID>::into)
        //     .collect::<IndexSet<_>>();
        // self.with(|builder| {
        //     let xs = call(builder, input);
        //     Ok::<_, CommonError>(xs)
        // })
        // .expect("No poison")
        todo!()
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
    pub fn get_primary_threshold(&self) -> u8 {
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
    pub fn set_name(&self, name: String) -> &Self {
        *self.name.write().unwrap() = name;
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

    pub fn remove_factor(
        &self,
        factor_source_id: FactorSourceID,
    ) -> Result<(), CommonError> {
        self.with(|builder| {
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
}

pub trait IsValidOrCanBecomeValid {
    fn is_valid_or_can_be(&self) -> bool;
}
impl<T> IsValidOrCanBecomeValid for Result<T, RoleBuilderValidation> {
    fn is_valid_or_can_be(&self) -> bool {
        match self {
            Ok(_) => true,
            Err(RoleBuilderValidation::BasicViolation(_)) => false,
            Err(RoleBuilderValidation::ForeverInvalid(_)) => false,
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
}

impl SecurityShieldBuilder {
    pub fn validation_for_addition_of_factor_source_to_primary_threshold_for_each(
        &self,
        factor_sources: Vec<FactorSourceID>,
    ) -> Vec<FactorSourceInRoleBuilderValidationStatus> {
        self.validation_for_addition_of_factor_source_by_calling(
            factor_sources,
            |builder, input| {
                builder
                    .validation_for_addition_of_factor_source_to_primary_threshold_for_each(input)
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
            SecurityShieldBuilderInvalidReason::ShieldNameInvalid
        })?;

        let metadata = SecurityStructureMetadata::new(display_name);

        let shield = SecurityStructureOfFactorSourceIds {
            matrix_of_factors,
            metadata,
        };
        Ok(shield)
    }
}

pub trait AsShieldBuilderViolation {
    fn as_shield_validation(
        &self,
    ) -> Option<SecurityShieldBuilderInvalidReason>;
}

impl<T> AsShieldBuilderViolation for Result<T, MatrixBuilderValidation> {
    fn as_shield_validation(
        &self,
    ) -> Option<SecurityShieldBuilderInvalidReason> {
        match self {
            Result::Err(err) => err.as_shield_validation(),
            Result::Ok(_) => None,
        }
    }
}
impl AsShieldBuilderViolation for MatrixBuilderValidation {
    fn as_shield_validation(
        &self,
    ) -> Option<SecurityShieldBuilderInvalidReason> {
        Some(SecurityShieldBuilderInvalidReason::Unknown)
    }
}

impl AsShieldBuilderViolation for MatrixRolesInCombinationViolation {
    fn as_shield_validation(
        &self,
    ) -> Option<SecurityShieldBuilderInvalidReason> {
        match self {
            Self::Basic(val) => val.as_shield_validation(),
            Self::ForeverInvalid(val) => val.as_shield_validation(),
            Self::NotYetValid(val) => val.as_shield_validation(),
        }
    }
}

impl AsShieldBuilderViolation for MatrixRolesInCombinationBasicViolation {
    fn as_shield_validation(
        &self,
    ) -> Option<SecurityShieldBuilderInvalidReason> {
        Some(SecurityShieldBuilderInvalidReason::Unknown)
    }
}
impl AsShieldBuilderViolation for MatrixRolesInCombinationForeverInvalid {
    fn as_shield_validation(
        &self,
    ) -> Option<SecurityShieldBuilderInvalidReason> {
        Some(SecurityShieldBuilderInvalidReason::Unknown)
    }
}
impl AsShieldBuilderViolation for MatrixRolesInCombinationNotYetValid {
    fn as_shield_validation(
        &self,
    ) -> Option<SecurityShieldBuilderInvalidReason> {
        Some(SecurityShieldBuilderInvalidReason::Unknown)
    }
}

impl AsShieldBuilderViolation for (RoleKind, RoleBuilderValidation) {
    fn as_shield_validation(
        &self,
    ) -> Option<SecurityShieldBuilderInvalidReason> {
        Some(SecurityShieldBuilderInvalidReason::Unknown)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SecurityShieldBuilderInvalidReason {
    ShieldNameInvalid,
    Unknown,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityShieldBuilder;

    //     #[test]
    //     fn test() {
    //         let sut = SUT::new();

    //         assert_eq!(sut.get_name(), "My Shield");
    //         sut.set_name("S.H.I.E.L.D.".to_owned());

    //         assert_eq!(sut.get_number_of_days_until_auto_confirm(), 14);
    //         sut.set_number_of_days_until_auto_confirm(u16::MAX).unwrap();
    //         assert_eq!(sut.get_number_of_days_until_auto_confirm(), u16::MAX);

    //         // Primary
    //         let sim_prim =
    //             sut.validation_for_addition_of_factor_source_to_primary_override_for_each(vec![
    //                 FactorSourceID::sample_arculus(),
    //             ]);

    //         let sim_prim_threshold = sut
    //             .validation_for_addition_of_factor_source_to_primary_threshold_for_each(vec![
    //                 FactorSourceID::sample_arculus(),
    //             ]);

    //         let sim_kind_prim = sut
    //             .validation_for_addition_of_factor_source_of_kind_to_primary_override(
    //                 FactorSourceKind::Device,
    //             );

    //         let sim_kind_prim_threshold = sut
    //             .validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
    //                 FactorSourceKind::Device,
    //             );

    //         sut.add_factor_source_to_primary_threshold(
    //             FactorSourceID::sample_device(),
    //         )
    //         .unwrap();
    //         assert_eq!(
    //             sut.get_primary_threshold_factors(),
    //             vec![FactorSourceID::sample_device()]
    //         );
    //         _ = sut.set_threshold(1);
    //         assert_eq!(sut.get_primary_threshold(), 1);
    //         sut.add_factor_source_to_primary_override(
    //             FactorSourceID::sample_arculus(),
    //         )
    //         .unwrap();
    //         sut.add_factor_source_to_primary_override(
    //             FactorSourceID::sample_arculus_other(),
    //         )
    //         .unwrap();

    //         assert_eq!(
    //             sut.get_primary_override_factors(),
    //             vec![
    //                 FactorSourceID::sample_arculus(),
    //                 FactorSourceID::sample_arculus_other()
    //             ]
    //         );

    //         // Recovery
    //         let sim_rec =
    //             sut.validation_for_addition_of_factor_source_to_recovery_override_for_each(vec![
    //                 FactorSourceID::sample_ledger(),
    //             ]);

    //         let sim_kind_rec = sut
    //             .validation_for_addition_of_factor_source_of_kind_to_recovery_override(
    //                 FactorSourceKind::ArculusCard,
    //             );

    //         sut.add_factor_source_to_recovery_override(
    //             FactorSourceID::sample_ledger(),
    //         )
    //         .unwrap();
    //         sut.add_factor_source_to_recovery_override(
    //             FactorSourceID::sample_ledger_other(),
    //         )
    //         .unwrap();

    //         assert_eq!(
    //             sut.get_recovery_factors(),
    //             vec![
    //                 FactorSourceID::sample_ledger(),
    //                 FactorSourceID::sample_ledger_other()
    //             ]
    //         );

    //         // Confirmation
    //         let sim_conf = sut
    //             .validation_for_addition_of_factor_source_to_confirmation_override_for_each(vec![
    //                 FactorSourceID::sample_device(),
    //             ]);

    //         let sim_kind_conf = sut
    //             .validation_for_addition_of_factor_source_of_kind_to_confirmation_override(
    //                 FactorSourceKind::ArculusCard,
    //             );

    //         sut.add_factor_source_to_confirmation_override(
    //             FactorSourceID::sample_device(),
    //         )
    //         .unwrap();

    //         assert_eq!(
    //             sut.get_confirmation_factors(),
    //             vec![FactorSourceID::sample_device(),]
    //         );

    //         assert_ne!(
    //             sim_prim,
    //             sut.validation_for_addition_of_factor_source_to_primary_override_for_each(vec![
    //                 FactorSourceID::sample_arculus(),
    //             ])
    //         );

    //         assert_ne!(
    //             sim_prim_threshold,
    //             sut.validation_for_addition_of_factor_source_to_primary_threshold_for_each(vec![
    //                 FactorSourceID::sample_arculus()
    //             ])
    //         );

    //         assert_ne!(
    //             sim_rec,
    //             sut.validation_for_addition_of_factor_source_to_recovery_override_for_each(vec![
    //                 FactorSourceID::sample_ledger(),
    //             ])
    //         );

    //         assert_ne!(
    //             sim_conf,
    //             sut.validation_for_addition_of_factor_source_to_confirmation_override_for_each(vec![
    //                 FactorSourceID::sample_device(),
    //             ])
    //         );

    //         assert_ne!(
    //             sim_kind_prim,
    //             sut.validation_for_addition_of_factor_source_of_kind_to_primary_override(
    //                 FactorSourceKind::Device,
    //             )
    //         );

    //         assert_ne!(
    //             sim_kind_prim_threshold,
    //             sut.validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
    //                 FactorSourceKind::Device,
    //             )
    //         );

    //         assert_eq!(
    //             sim_kind_rec,
    //             sut.validation_for_addition_of_factor_source_of_kind_to_recovery_override(
    //                 FactorSourceKind::ArculusCard,
    //             )
    //         );

    //         assert_eq!(
    //             sim_kind_conf,
    //             sut.validation_for_addition_of_factor_source_of_kind_to_confirmation_override(
    //                 FactorSourceKind::ArculusCard,
    //             )
    //         );

    //         sut.remove_factor(FactorSourceID::sample_arculus_other())
    //             .unwrap();
    //         sut.remove_factor(FactorSourceID::sample_ledger_other())
    //             .unwrap();

    //         let v0 = sut.validate();
    //         let v1 = sut.validate(); // can call validate many times!
    //         assert_eq!(v0, v1);

    //         let shield = sut.build().unwrap(); // can build only once! (but can build after `validate`)
    //         assert_eq!(
    //             shield.matrix_builder.metadata.display_name.value,
    //             "S.H.I.E.L.D."
    //         );
    //         assert_eq!(
    //             shield
    //                 .matrix_builder
    //                 .matrix_of_factors
    //                 .primary()
    //                 .get_override_factors(),
    //             &vec![FactorSourceID::sample_arculus().into()]
    //         );
    //         assert_eq!(
    //             shield
    //                 .matrix_builder
    //                 .matrix_of_factors
    //                 .recovery()
    //                 .get_override_factors(),
    //             &vec![FactorSourceID::sample_ledger().into()]
    //         );
    //         assert_eq!(
    //             shield
    //                 .matrix_builder
    //                 .matrix_of_factors
    //                 .confirmation()
    //                 .get_override_factors(),
    //             &vec![FactorSourceID::sample_device().into()]
    //         );
    //     }
}
