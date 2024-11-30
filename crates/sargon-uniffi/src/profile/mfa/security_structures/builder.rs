#![allow(clippy::new_without_default)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::{Arc, RwLock};

use sargon::IndexSet;

use crate::prelude::*;

#[derive(Debug, uniffi::Object)]
pub struct SecurityShieldBuilder {
    wrapped: RwLock<Option<MatrixBuilder>>,
    name: RwLock<String>,
}

#[derive(Debug, PartialEq, Eq, Hash, uniffi::Object)]
#[uniffi::export(Debug, Eq, Hash)]
pub struct SecurityStructureOfFactorSourceIds {
    pub wrapped: rules::SecurityStructureOfFactorSourceIds,
}

impl SecurityShieldBuilder {
    fn get<R>(
        &self,
        with_non_consumed_builder: impl Fn(&MatrixBuilder) -> R,
    ) -> R {
        let binding = self.wrapped.read().unwrap();

        let Some(builder) = binding.as_ref() else {
            unreachable!("Already built, should not have happened.")
        };
        with_non_consumed_builder(builder)
    }

    fn with<R, E: Into<CommonError>>(
        &self,
        mut with_non_consumed_builder: impl FnMut(
            &mut MatrixBuilder,
        ) -> Result<R, E>,
    ) -> Result<R, CommonError> {
        let guard = self.wrapped.write();

        let mut binding =
            guard.map_err(|_| CommonError::MatrixBuilderRwLockPoisoned)?;

        let Some(builder) = binding.as_mut() else {
            return Err(CommonError::AlreadyBuilt);
        };
        with_non_consumed_builder(builder)
            .map_err(|e| Into::<CommonError>::into(e))
    }

    fn validation_for_addition_of_factor_source_by_calling(
        &self,
        factor_sources: Vec<Arc<FactorSourceID>>,
        call: impl Fn(
            &MatrixBuilder,
            &IndexSet<sargon::FactorSourceID>,
        )
            -> IndexSet<rules::FactorSourceInRoleBuilderValidationStatus>,
    ) -> Result<Vec<Arc<FactorSourceValidationStatus>>, CommonError> {
        let input = &factor_sources
            .clone()
            .into_iter()
            .map(|x| x.inner)
            .collect::<IndexSet<_>>();
        self.with(|builder| {
            let xs = call(builder, input);

            let xs = xs
                .into_iter()
                .map(Into::<FactorSourceValidationStatus>::into)
                .map(Arc::new)
                .collect();

            Ok::<_, CommonError>(xs)
        })
    }
}

#[uniffi::export]
impl SecurityShieldBuilder {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            wrapped: RwLock::new(Some(MatrixBuilder::new())),
            name: RwLock::new("My Shield".to_owned()),
        })
    }
}

impl SecurityShieldBuilder {
    fn get_factors(
        &self,
        access: impl Fn(&MatrixBuilder) -> &Vec<sargon::FactorSourceID>,
    ) -> Vec<Arc<FactorSourceID>> {
        self.get(|builder| {
            let factors = access(builder);
            factors.iter().map(FactorSourceID::new).collect::<Vec<_>>()
        })
    }
}

// ====================
// ==== GET / READ ====
// ====================
#[uniffi::export]
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

    pub fn get_primary_threshold_factors(&self) -> Vec<Arc<FactorSourceID>> {
        self.get_factors(|builder| builder.get_primary_threshold_factors())
    }

    pub fn get_primary_override_factors(&self) -> Vec<Arc<FactorSourceID>> {
        self.get_factors(|builder| builder.get_primary_override_factors())
    }

    pub fn get_recovery_factors(&self) -> Vec<Arc<FactorSourceID>> {
        self.get_factors(|builder| builder.get_recovery_factors())
    }

    pub fn get_confirmation_factors(&self) -> Vec<Arc<FactorSourceID>> {
        self.get_factors(|builder| builder.get_confirmation_factors())
    }
}

// ====================
// ===== MUTATION =====
// ====================
#[uniffi::export]
impl SecurityShieldBuilder {
    pub fn set_name(&self, name: String) {
        *self.name.write().unwrap() = name
    }

    /// Adds the factor source to the primary role threshold list.
    pub fn add_factor_source_to_primary_threshold(
        &self,
        factor_source_id: Arc<FactorSourceID>,
    ) -> Result<(), CommonError> {
        self.with(|builder| {
            builder
                .add_factor_source_to_primary_threshold(factor_source_id.inner)
        })
    }

    pub fn add_factor_source_to_primary_override(
        &self,
        factor_source_id: Arc<FactorSourceID>,
    ) -> Result<(), CommonError> {
        self.with(|builder| {
            builder
                .add_factor_source_to_primary_override(factor_source_id.inner)
        })
    }

    pub fn remove_factor(
        &self,
        factor_source_id: Arc<FactorSourceID>,
    ) -> Result<(), CommonError> {
        self.with(|builder| builder.remove_factor(&factor_source_id.inner))
    }

    pub fn set_threshold(&self, threshold: u8) -> Result<(), CommonError> {
        self.with(|builder| builder.set_threshold(threshold))
    }

    pub fn set_number_of_days_until_auto_confirm(
        &self,
        number_of_days: u16,
    ) -> Result<(), CommonError> {
        self.with(|builder| {
            builder.set_number_of_days_until_auto_confirm(number_of_days)
        })
    }

    pub fn add_factor_source_to_recovery_override(
        &self,
        factor_source_id: Arc<FactorSourceID>,
    ) -> Result<(), CommonError> {
        self.with(|builder| {
            builder
                .add_factor_source_to_recovery_override(factor_source_id.inner)
        })
    }

    pub fn add_factor_source_to_confirmation_override(
        &self,
        factor_source_id: Arc<FactorSourceID>,
    ) -> Result<(), CommonError> {
        self.with(|builder| {
            builder.add_factor_source_to_confirmation_override(
                factor_source_id.inner,
            )
        })
    }

    pub fn validation_for_addition_of_factor_source_of_kind_to_confirmation_override(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> Result<(), CommonError> {
        self.with(|builder| {
            builder.validation_for_addition_of_factor_source_of_kind_to_confirmation_override(
                factor_source_kind.into(),
            )
        })
    }

    pub fn validation_for_addition_of_factor_source_of_kind_to_recovery_override(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> Result<(), CommonError> {
        self.with(|builder| {
            builder.validation_for_addition_of_factor_source_of_kind_to_recovery_override(
                factor_source_kind.into(),
            )
        })
    }

    pub fn validation_for_addition_of_factor_source_of_kind_to_primary_override(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> Result<(), CommonError> {
        self.with(|builder| {
            builder.validation_for_addition_of_factor_source_of_kind_to_primary_override(
                factor_source_kind.into(),
            )
        })
    }

    pub fn validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> Result<(), CommonError> {
        self.with(|builder| {
            builder.validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
                factor_source_kind.into(),
            )
        })
    }

    pub fn validation_for_addition_of_factor_source_to_primary_threshold_for_each(
        &self,
        factor_sources: Vec<Arc<FactorSourceID>>,
    ) -> Result<Vec<Arc<FactorSourceValidationStatus>>, CommonError> {
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
        factor_sources: Vec<Arc<FactorSourceID>>,
    ) -> Result<Vec<Arc<FactorSourceValidationStatus>>, CommonError> {
        self.validation_for_addition_of_factor_source_by_calling(
            factor_sources,
            |builder, input| {
                builder.validation_for_addition_of_factor_source_to_primary_override_for_each(input)
            },
        )
    }

    pub fn validation_for_addition_of_factor_source_to_recovery_override_for_each(
        &self,
        factor_sources: Vec<Arc<FactorSourceID>>,
    ) -> Result<Vec<Arc<FactorSourceValidationStatus>>, CommonError> {
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
        factor_sources: Vec<Arc<FactorSourceID>>,
    ) -> Result<Vec<Arc<FactorSourceValidationStatus>>, CommonError> {
        self.validation_for_addition_of_factor_source_by_calling(
            factor_sources,
            |builder, input| {
                builder.validation_for_addition_of_factor_source_to_confirmation_override_for_each(
                    input,
                )
            },
        )
    }

    pub fn build(
        self: Arc<Self>,
    ) -> Result<SecurityStructureOfFactorSourceIds, CommonError> {
        let mut binding = self
            .wrapped
            .write()
            .map_err(|_| CommonError::MatrixBuilderRwLockPoisoned)?;
        let builder = binding.take().ok_or(CommonError::AlreadyBuilt)?;
        let wrapped_matrix = builder
            .build()
            .map_err(|e| CommonError::BuildError(format!("{:?}", e)))?;

        let name = self.get_name();
        let display_name = sargon::DisplayName::new(name)
            .map_err(|e| CommonError::Sargon(format!("{:?}", e)))?;
        let wrapped_shield = rules::SecurityStructureOfFactorSourceIds::new(
            display_name,
            wrapped_matrix,
        );

        let shield = SecurityStructureOfFactorSourceIds {
            wrapped: wrapped_shield,
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

        assert_eq!(sut.get_name(), "My Shield");
        sut.set_name("S.H.I.E.L.D.".to_owned());

        assert_eq!(sut.get_number_of_days_until_auto_confirm(), 14);
        sut.set_number_of_days_until_auto_confirm(u16::MAX).unwrap();
        assert_eq!(sut.get_number_of_days_until_auto_confirm(), u16::MAX);

        // Primary
        let sim_prim =
            sut.validation_for_addition_of_factor_source_to_primary_override_for_each(vec![
                FactorSourceID::sample_arculus(),
            ]);

        let sim_prim_threshold = sut
            .validation_for_addition_of_factor_source_to_primary_threshold_for_each(vec![
                FactorSourceID::sample_arculus(),
            ]);

        let sim_kind_prim = sut
            .validation_for_addition_of_factor_source_of_kind_to_primary_override(
                FactorSourceKind::Device,
            );

        let sim_kind_prim_threshold = sut
            .validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
                FactorSourceKind::Device,
            );

        sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_device(),
        )
        .unwrap();
        assert_eq!(
            sut.get_primary_threshold_factors(),
            vec![FactorSourceID::sample_device()]
        );
        _ = sut.set_threshold(1);
        assert_eq!(sut.get_primary_threshold(), 1);
        sut.add_factor_source_to_primary_override(
            FactorSourceID::sample_arculus(),
        )
        .unwrap();
        sut.add_factor_source_to_primary_override(
            FactorSourceID::sample_arculus_other(),
        )
        .unwrap();

        assert_eq!(
            sut.get_primary_override_factors(),
            vec![
                FactorSourceID::sample_arculus(),
                FactorSourceID::sample_arculus_other()
            ]
        );

        // Recovery
        let sim_rec =
            sut.validation_for_addition_of_factor_source_to_recovery_override_for_each(vec![
                FactorSourceID::sample_ledger(),
            ]);

        let sim_kind_rec = sut
            .validation_for_addition_of_factor_source_of_kind_to_recovery_override(
                FactorSourceKind::ArculusCard,
            );

        sut.add_factor_source_to_recovery_override(
            FactorSourceID::sample_ledger(),
        )
        .unwrap();
        sut.add_factor_source_to_recovery_override(
            FactorSourceID::sample_ledger_other(),
        )
        .unwrap();

        assert_eq!(
            sut.get_recovery_factors(),
            vec![
                FactorSourceID::sample_ledger(),
                FactorSourceID::sample_ledger_other()
            ]
        );

        // Confirmation
        let sim_conf = sut
            .validation_for_addition_of_factor_source_to_confirmation_override_for_each(vec![
                FactorSourceID::sample_device(),
            ]);

        let sim_kind_conf = sut
            .validation_for_addition_of_factor_source_of_kind_to_confirmation_override(
                FactorSourceKind::ArculusCard,
            );

        sut.add_factor_source_to_confirmation_override(
            FactorSourceID::sample_device(),
        )
        .unwrap();

        assert_eq!(
            sut.get_confirmation_factors(),
            vec![FactorSourceID::sample_device(),]
        );

        assert_ne!(
            sim_prim,
            sut.validation_for_addition_of_factor_source_to_primary_override_for_each(vec![
                FactorSourceID::sample_arculus(),
            ])
        );

        assert_ne!(
            sim_prim_threshold,
            sut.validation_for_addition_of_factor_source_to_primary_threshold_for_each(vec![
                FactorSourceID::sample_arculus()
            ])
        );

        assert_ne!(
            sim_rec,
            sut.validation_for_addition_of_factor_source_to_recovery_override_for_each(vec![
                FactorSourceID::sample_ledger(),
            ])
        );

        assert_ne!(
            sim_conf,
            sut.validation_for_addition_of_factor_source_to_confirmation_override_for_each(vec![
                FactorSourceID::sample_device(),
            ])
        );

        assert_ne!(
            sim_kind_prim,
            sut.validation_for_addition_of_factor_source_of_kind_to_primary_override(
                FactorSourceKind::Device,
            )
        );

        assert_ne!(
            sim_kind_prim_threshold,
            sut.validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
                FactorSourceKind::Device,
            )
        );

        assert_eq!(
            sim_kind_rec,
            sut.validation_for_addition_of_factor_source_of_kind_to_recovery_override(
                FactorSourceKind::ArculusCard,
            )
        );

        assert_eq!(
            sim_kind_conf,
            sut.validation_for_addition_of_factor_source_of_kind_to_confirmation_override(
                FactorSourceKind::ArculusCard,
            )
        );

        sut.remove_factor(FactorSourceID::sample_arculus_other())
            .unwrap();
        sut.remove_factor(FactorSourceID::sample_ledger_other())
            .unwrap();

        let shield = sut.build().unwrap();
        assert_eq!(shield.wrapped.metadata.display_name.value, "S.H.I.E.L.D.");
        assert_eq!(
            shield
                .wrapped
                .matrix_of_factors
                .primary()
                .get_override_factors(),
            &vec![FactorSourceID::sample_arculus().inner]
        );
        assert_eq!(
            shield
                .wrapped
                .matrix_of_factors
                .recovery()
                .get_override_factors(),
            &vec![FactorSourceID::sample_ledger().inner]
        );
        assert_eq!(
            shield
                .wrapped
                .matrix_of_factors
                .confirmation()
                .get_override_factors(),
            &vec![FactorSourceID::sample_device().inner]
        );
    }
}
