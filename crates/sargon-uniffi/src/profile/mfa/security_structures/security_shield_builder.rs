#![allow(clippy::new_without_default)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    borrow::Borrow,
    sync::{Arc, RwLock},
};

use sargon::{IndexSet, MatrixBuilder};

use crate::prelude::*;

/// A builder of `SecurityStructureOfFactorSourceIds` a.k.a. `SecurityShield`,
/// which contains a MatrixOfFactorSourceIds - with primary, recovery, and
/// confirmation roles.
#[derive(Debug, uniffi::Object)]
pub struct SecurityShieldBuilder {
    wrapped: RwLock<sargon::SecurityShieldBuilder>,
}

#[uniffi::export]
impl SecurityShieldBuilder {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            wrapped: RwLock::new(sargon::SecurityShieldBuilder::new()),
        })
    }
}

impl SecurityShieldBuilder {
    fn get<R>(
        self: Arc<Self>,
        access: impl Fn(&sargon::SecurityShieldBuilder) -> R,
    ) -> R {
        let binding = self.wrapped.read().unwrap();
        access(&binding)
    }

    fn set(
        self: Arc<Self>,
        mut write: impl FnMut(
            &mut sargon::SecurityShieldBuilder,
        ) -> &sargon::SecurityShieldBuilder,
    ) {
        let mut binding = self.wrapped.write().expect("No poison");
        _ = write(&mut binding);
    }

    fn validation_for_addition_of_factor_source_by_calling(
        self: Arc<Self>,
        factor_sources: Vec<FactorSourceID>,
        call: impl Fn(
            &sargon::SecurityShieldBuilder,
            Vec<sargon::FactorSourceID>,
        )
            -> Vec<sargon::FactorSourceInRoleBuilderValidationStatus>,
    ) -> Vec<FactorSourceValidationStatus> {
        let input = factor_sources
            .clone()
            .into_iter()
            .map(Into::<sargon::FactorSourceID>::into)
            .collect_vec();

        self.get(|builder| {
            call(builder, input.clone())
                .into_iter()
                .map(Into::<FactorSourceValidationStatus>::into)
                .collect()
        })
    }
}

impl SecurityShieldBuilder {
    fn get_factors(
        self: Arc<Self>,
        access: impl Fn(
            &sargon::SecurityShieldBuilder,
        ) -> Vec<sargon::FactorSourceID>,
    ) -> Vec<FactorSourceID> {
        self.get(|builder| {
            let factors = access(builder);
            factors
                .into_iter()
                .map(crate::FactorSourceID::from)
                .collect::<Vec<_>>()
        })
    }
}
// ====================
// ==== GET / READ ====
// ====================
#[uniffi::export]
impl SecurityShieldBuilder {
    pub fn get_primary_threshold(self: Arc<Self>) -> u8 {
        self.get(|builder| builder.get_threshold())
    }

    pub fn get_number_of_days_until_auto_confirm(self: Arc<Self>) -> u16 {
        self.get(|builder| builder.get_number_of_days_until_auto_confirm())
    }

    pub fn get_name(self: Arc<Self>) -> String {
        self.get(|builder| builder.get_name())
    }

    pub fn get_primary_threshold_factors(
        self: Arc<Self>,
    ) -> Vec<FactorSourceID> {
        self.get_factors(|builder| builder.get_primary_threshold_factors())
    }

    pub fn get_primary_override_factors(
        self: Arc<Self>,
    ) -> Vec<FactorSourceID> {
        self.get_factors(|builder| builder.get_primary_override_factors())
    }

    pub fn get_recovery_factors(self: Arc<Self>) -> Vec<FactorSourceID> {
        self.get_factors(|builder| builder.get_recovery_factors())
    }

    pub fn get_confirmation_factors(self: Arc<Self>) -> Vec<FactorSourceID> {
        self.get_factors(|builder| builder.get_confirmation_factors())
    }
}

// ====================
// ===== MUTATION =====
// ====================
#[uniffi::export]
impl SecurityShieldBuilder {
    pub fn set_name(self: Arc<Self>, name: String) {
        self.set(|builder| builder.set_name(&name));
    }

    pub fn remove_factor_from_all_roles(
        self: Arc<Self>,
        factor_source_id: FactorSourceID,
    ) {
        self.set(|builder| {
            builder
                .remove_factor_from_all_roles(factor_source_id.clone().into())
        })
    }

    pub fn remove_factor_from_primary(
        self: Arc<Self>,
        factor_source_id: FactorSourceID,
    ) {
        self.set(|builder| {
            builder.remove_factor_from_primary(factor_source_id.clone().into())
        })
    }

    pub fn remove_factor_from_recovery(
        self: Arc<Self>,
        factor_source_id: FactorSourceID,
    ) {
        self.set(|builder| {
            builder.remove_factor_from_recovery(factor_source_id.clone().into())
        })
    }

    pub fn remove_factor_from_confirmation(
        self: Arc<Self>,
        factor_source_id: FactorSourceID,
    ) {
        self.set(|builder| {
            builder.remove_factor_from_confirmation(
                factor_source_id.clone().into(),
            )
        })
    }

    pub fn set_threshold(self: Arc<Self>, threshold: u8) {
        self.set(|builder| builder.set_threshold(threshold))
    }

    pub fn set_number_of_days_until_auto_confirm(
        self: Arc<Self>,
        number_of_days: u16,
    ) {
        self.set(|builder| {
            builder.set_number_of_days_until_auto_confirm(number_of_days)
        })
    }

    /// Adds the factor source to the primary role threshold list.
    pub fn add_factor_source_to_primary_threshold(
        self: Arc<Self>,
        factor_source_id: FactorSourceID,
    ) {
        self.set(|builder| {
            builder.add_factor_source_to_primary_threshold(
                factor_source_id.clone().into(),
            )
        })
    }

    pub fn add_factor_source_to_primary_override(
        self: Arc<Self>,
        factor_source_id: FactorSourceID,
    ) {
        self.set(|builder| {
            builder.add_factor_source_to_primary_override(
                factor_source_id.clone().into(),
            )
        })
    }

    pub fn add_factor_source_to_recovery_override(
        self: Arc<Self>,
        factor_source_id: FactorSourceID,
    ) {
        self.set(|builder| {
            builder.add_factor_source_to_recovery_override(
                factor_source_id.clone().into(),
            )
        })
    }

    pub fn add_factor_source_to_confirmation_override(
        self: Arc<Self>,
        factor_source_id: FactorSourceID,
    ) {
        self.set(|builder| {
            builder.add_factor_source_to_confirmation_override(
                factor_source_id.clone().into(),
            )
        })
    }
}

#[uniffi::export]
impl SecurityShieldBuilder {
    pub fn addition_of_factor_source_of_kind_to_primary_threshold_is_fully_valid(
        self: Arc<Self>,
        factor_source_kind: FactorSourceKind,
    ) -> bool {
        self.get(|builder|
            builder.addition_of_factor_source_of_kind_to_primary_threshold_is_fully_valid(
                factor_source_kind.clone().into(),
            )
        )
    }

    pub fn addition_of_factor_source_of_kind_to_primary_threshold_is_valid_or_can_be(
        self: Arc<Self>,
        factor_source_kind: FactorSourceKind,
    ) -> bool {
        self.get(|builder|
            builder.addition_of_factor_source_of_kind_to_primary_threshold_is_valid_or_can_be(
                factor_source_kind.clone().into(),
            )
        )
    }

    pub fn addition_of_factor_source_of_kind_to_primary_override_is_fully_valid(
        self: Arc<Self>,
        factor_source_kind: FactorSourceKind,
    ) -> bool {
        self.get(|builder|
            builder.addition_of_factor_source_of_kind_to_primary_override_is_fully_valid(
                factor_source_kind.clone().into(),
            )
        )
    }

    pub fn addition_of_factor_source_of_kind_to_primary_override_is_valid_or_can_be(
        self: Arc<Self>,
        factor_source_kind: FactorSourceKind,
    ) -> bool {
        self.get(|builder|
            builder.addition_of_factor_source_of_kind_to_primary_override_is_valid_or_can_be(
                factor_source_kind.clone().into(),
            )
        )
    }

    pub fn addition_of_factor_source_of_kind_to_recovery_is_valid_or_can_be(
        self: Arc<Self>,
        factor_source_kind: FactorSourceKind,
    ) -> bool {
        self.get(|builder|
            builder.addition_of_factor_source_of_kind_to_recovery_is_valid_or_can_be(
                factor_source_kind.clone().into(),
            )
        )
    }

    pub fn addition_of_factor_source_of_kind_to_recovery_is_fully_valid(
        self: Arc<Self>,
        factor_source_kind: FactorSourceKind,
    ) -> bool {
        self.get(|builder| {
            builder
                .addition_of_factor_source_of_kind_to_recovery_is_fully_valid(
                    factor_source_kind.clone().into(),
                )
        })
    }

    pub fn addition_of_factor_source_of_kind_to_confirmation_is_valid_or_can_be(
        self: Arc<Self>,
        factor_source_kind: FactorSourceKind,
    ) -> bool {
        self.get(|builder|
            builder.addition_of_factor_source_of_kind_to_confirmation_is_valid_or_can_be(
                factor_source_kind.clone().into(),
            )
        )
    }

    pub fn addition_of_factor_source_of_kind_to_confirmation_is_fully_valid(
        self: Arc<Self>,
        factor_source_kind: FactorSourceKind,
    ) -> bool {
        self.get(|builder|
            builder.addition_of_factor_source_of_kind_to_confirmation_is_fully_valid(
                factor_source_kind.clone().into(),
            )
        )
    }
}

#[uniffi::export]
impl SecurityShieldBuilder {
    pub fn validation_for_addition_of_factor_source_to_primary_threshold_for_each(
        self: Arc<Self>,
        factor_sources: Vec<FactorSourceID>,
    ) -> Vec<FactorSourceValidationStatus> {
        self.validation_for_addition_of_factor_source_by_calling(
            factor_sources,
            |builder, input| {
                builder
                    .validation_for_addition_of_factor_source_to_primary_threshold_for_each(input)
            },
        )
    }

    pub fn validation_for_addition_of_factor_source_to_primary_override_for_each(
        self: Arc<Self>,
        factor_sources: Vec<FactorSourceID>,
    ) -> Vec<FactorSourceValidationStatus> {
        self.validation_for_addition_of_factor_source_by_calling(
            factor_sources,
            |builder, input| {
                builder.validation_for_addition_of_factor_source_to_primary_override_for_each(input)
            },
        )
    }

    pub fn validation_for_addition_of_factor_source_to_recovery_override_for_each(
        self: Arc<Self>,
        factor_sources: Vec<FactorSourceID>,
    ) -> Vec<FactorSourceValidationStatus> {
        self.validation_for_addition_of_factor_source_by_calling(
            factor_sources,
            |builder, input| {
                builder
                    .validation_for_addition_of_factor_source_to_recovery_override_for_each(input)
            },
        )
    }
    pub fn validation_for_addition_of_factor_source_to_confirmation_override_for_each(
        self: Arc<Self>,
        factor_sources: Vec<FactorSourceID>,
    ) -> Vec<FactorSourceValidationStatus> {
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

#[uniffi::export]
impl SecurityShieldBuilder {
    pub fn validate(
        self: Arc<Self>,
    ) -> Option<SecurityShieldBuilderInvalidReason> {
        self.get(|builder| builder.validate().map(|x| x.into()))
    }

    pub fn build(
        self: Arc<Self>,
    ) -> Result<
        SecurityStructureOfFactorSourceIDs,
        SecurityShieldBuilderInvalidReason,
    > {
        self.get(|builder| builder.build())
            .map(|shield| shield.into())
            .map_err(|x| x.into())
    }
}

impl FactorSourceID {
    pub fn new(inner: impl Borrow<sargon::FactorSourceID>) -> Self {
        Self::from(*inner.borrow())
    }
}

#[cfg(test)]
impl FactorSourceID {
    pub fn sample_device() -> Self {
        Self::new(sargon::FactorSourceID::sample_device())
    }

    pub fn sample_device_other() -> Self {
        Self::new(sargon::FactorSourceID::sample_device_other())
    }

    pub fn sample_ledger() -> Self {
        Self::new(sargon::FactorSourceID::sample_ledger())
    }

    pub fn sample_ledger_other() -> Self {
        Self::new(sargon::FactorSourceID::sample_ledger_other())
    }

    pub fn sample_arculus() -> Self {
        Self::new(sargon::FactorSourceID::sample_arculus())
    }

    pub fn sample_arculus_other() -> Self {
        Self::new(sargon::FactorSourceID::sample_arculus_other())
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

        assert_eq!(sut.clone().get_name(), "My Shield");
        sut.clone().set_name("S.H.I.E.L.D.".to_owned());

        assert_eq!(sut.clone().get_number_of_days_until_auto_confirm(), 14);
        sut.clone().set_number_of_days_until_auto_confirm(u16::MAX);
        assert_eq!(
            sut.clone().get_number_of_days_until_auto_confirm(),
            u16::MAX
        );
        // Primary
        // let sim_prim =
        //     sut.clone().validation_for_addition_of_factor_source_to_primary_override_for_each(vec![
        //         FactorSourceID::sample_arculus(),
        //     ]);

        // let sim_prim_threshold = sut.clone()
        //     .validation_for_addition_of_factor_source_to_primary_threshold_for_each(vec![
        //         FactorSourceID::sample_arculus(),
        //     ]);

        // let sim_kind_prim = sut.clone()
        //     .addition_of_factor_source_of_kind_to_primary_override_is_fully_valid(
        //         FactorSourceKind::Device,
        //     );

        // let sim_kind_prim_threshold = sut.clone()
        //     .addition_of_factor_source_of_kind_to_primary_threshold_is_fully_valid(
        //         FactorSourceKind::Device,
        //     );

        sut.clone().add_factor_source_to_primary_threshold(
            FactorSourceID::sample_device(),
        );
        assert_eq!(
            sut.clone().get_primary_threshold_factors(),
            vec![FactorSourceID::sample_device()]
        );
        sut.clone().set_threshold(1);
        assert_eq!(sut.clone().get_primary_threshold(), 1);
        sut.clone().add_factor_source_to_primary_override(
            FactorSourceID::sample_arculus(),
        );
        sut.clone().add_factor_source_to_primary_override(
            FactorSourceID::sample_arculus_other(),
        );

        assert_eq!(
            sut.clone().get_primary_override_factors(),
            vec![
                FactorSourceID::sample_arculus(),
                FactorSourceID::sample_arculus_other()
            ]
        );

        // Recovery
        // let sim_rec =
        //     sut.clone().validation_for_addition_of_factor_source_to_recovery_override_for_each(vec![
        //         FactorSourceID::sample_ledger(),
        //     ]);

        // let sim_kind_rec = sut
        //     .clone()
        //     .addition_of_factor_source_of_kind_to_recovery_is_fully_valid(
        //         FactorSourceKind::ArculusCard,
        //     );

        sut.clone().add_factor_source_to_recovery_override(
            FactorSourceID::sample_ledger(),
        );
        sut.clone().add_factor_source_to_recovery_override(
            FactorSourceID::sample_ledger_other(),
        );

        assert_eq!(
            sut.clone().get_recovery_factors(),
            vec![
                FactorSourceID::sample_ledger(),
                FactorSourceID::sample_ledger_other()
            ]
        );

        // Confirmation
        // let sim_conf = sut.clone()
        //     .validation_for_addition_of_factor_source_to_confirmation_override_for_each(vec![
        //         FactorSourceID::sample_device(),
        //     ]);

        // let sim_kind_conf = sut
        //     .clone()
        //     .addition_of_factor_source_of_kind_to_confirmation_is_fully_valid(
        //         FactorSourceKind::ArculusCard,
        //     );

        sut.clone().add_factor_source_to_confirmation_override(
            FactorSourceID::sample_device(),
        );

        assert_eq!(
            sut.clone().get_confirmation_factors(),
            vec![FactorSourceID::sample_device(),]
        );
        /*

        Reintroduce these asserts when we reintroduce the field `reason_if_invalid` inside `FactorSourceValidationStatus`

                assert_ne!(
                    sim_prim,
                    sut.clone().validation_for_addition_of_factor_source_to_primary_override_for_each(vec![
                        FactorSourceID::sample_arculus(),
                    ])
                );

                assert_ne!(
                    sim_prim_threshold,
                    sut.clone().validation_for_addition_of_factor_source_to_primary_threshold_for_each(vec![
                        FactorSourceID::sample_arculus()
                    ])
                );

                assert_ne!(
                    sim_rec,
                    sut.clone().validation_for_addition_of_factor_source_to_recovery_override_for_each(vec![
                        FactorSourceID::sample_ledger(),
                    ])
                );

                assert_ne!(
                    sim_conf,
                    sut.clone().validation_for_addition_of_factor_source_to_confirmation_override_for_each(vec![
                        FactorSourceID::sample_device(),
                    ])
                );

                assert_ne!(
                    sim_kind_prim,
                    sut.clone().addition_of_factor_source_of_kind_to_primary_override_is_fully_valid(
                        FactorSourceKind::Device,
                    )
                );

                assert_ne!(
                    sim_kind_prim_threshold,
                    sut.clone().addition_of_factor_source_of_kind_to_primary_threshold_is_fully_valid(
                        FactorSourceKind::Device,
                    )
                );

                assert_eq!(
                    sim_kind_rec,
                    sut.clone().addition_of_factor_source_of_kind_to_recovery_is_fully_valid(
                        FactorSourceKind::ArculusCard,
                    )
                );

                assert_eq!(
                    sim_kind_conf,
                    sut.clone().addition_of_factor_source_of_kind_to_confirmation_is_fully_valid(
                        FactorSourceKind::ArculusCard,
                    )
                );
         */
        sut.clone().remove_factor_from_all_roles(
            FactorSourceID::sample_arculus_other(),
        );
        sut.clone().remove_factor_from_all_roles(FactorSourceID::sample_ledger_other());

        let f = FactorSourceID::sample_ledger_other();
        let xs = sut.clone().get_primary_override_factors();
        sut.clone().add_factor_source_to_primary_override(f.clone());
        sut.clone().remove_factor_from_primary(f.clone());
        assert_eq!(xs, sut.clone().get_primary_override_factors());

        let xs = sut.clone().get_recovery_factors();
        sut.clone()
            .add_factor_source_to_recovery_override(f.clone());
        sut.clone().remove_factor_from_recovery(f.clone());
        assert_eq!(xs, sut.clone().get_recovery_factors());

        let xs = sut.clone().get_confirmation_factors();
        sut.clone()
            .add_factor_source_to_confirmation_override(f.clone());
        sut.clone().remove_factor_from_confirmation(f.clone());
        assert_eq!(xs, sut.clone().get_confirmation_factors());

        let v0 = sut.clone().validate();
        let v1 = sut.clone().validate(); // can call validate many times!
        assert_eq!(v0, v1);

        let shield0 = sut.clone().build().unwrap();
        let shield = sut.clone().build().unwrap(); // can call build many times!
        assert_eq!(shield0, shield);

        assert_eq!(shield.metadata.display_name.value, "S.H.I.E.L.D.");
        assert_eq!(
            shield.matrix_of_factors.primary_role.override_factors,
            vec![FactorSourceID::sample_arculus()]
        );
        assert_eq!(
            shield.matrix_of_factors.recovery_role.override_factors,
            vec![FactorSourceID::sample_ledger()]
        );
        assert_eq!(
            shield.matrix_of_factors.confirmation_role.override_factors,
            vec![FactorSourceID::sample_device()]
        );
    }
}
