#![allow(clippy::new_without_default)]
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::prelude::*;
#[cfg(test)]
use sargon::FactorSourceWithExtraSampleValues;
use std::{borrow::Borrow, sync::Arc};

/// A builder of `SecurityStructureOfFactorSourceIds` a.k.a. `SecurityShield`,
/// which contains a MatrixOfFactorSourceIds - with primary, recovery, and
/// confirmation roles.
#[derive(Debug, Hash, PartialEq, Clone, uniffi::Object)]
#[uniffi::export(Hash, Eq)]
pub struct SecurityShieldBuilder {
    wrapped: Arc<sargon::SecurityShieldBuilder>,
}

#[uniffi::export]
impl SecurityShieldBuilder {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            wrapped: Arc::new(sargon::SecurityShieldBuilder::lenient()),
        })
    }
}

impl SecurityShieldBuilder {
    fn get<R>(
        &self,
        access: impl Fn(&sargon::SecurityShieldBuilder) -> R,
    ) -> R {
        let binding = self.wrapped.clone();
        access(&binding)
    }

    fn set(
        self: Arc<Self>,
        write: impl Fn(
            &Arc<sargon::SecurityShieldBuilder>,
        ) -> &sargon::SecurityShieldBuilder,
    ) -> Arc<Self> {
        builder_arc_map(self, |builder| {
            _ = write(&builder.wrapped);
        })
    }

    fn validation_for_addition_of_factor_source_by_calling(
        &self,
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
        &self,
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
    pub fn get_primary_threshold(&self) -> Threshold {
        self.get(|builder| builder.get_threshold()).into()
    }

    pub fn get_primary_threshold_values(&self) -> Vec<Threshold> {
        self.get(|builder| builder.get_threshold_values())
            .into_iter()
            .map(|threshold| threshold.into())
            .collect()
    }

    pub fn get_time_until_timed_confirmation_is_callable(&self) -> TimePeriod {
        self.get(|builder| {
            builder.get_time_until_timed_confirmation_is_callable()
        })
        .into()
    }

    pub fn get_name(&self) -> String {
        self.get(|builder| builder.get_name())
    }

    pub fn get_authentication_signing_factor(&self) -> Option<FactorSourceID> {
        self.get(|builder| builder.get_authentication_signing_factor())
            .map(|x| x.into())
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
#[uniffi::export]
impl SecurityShieldBuilder {
    pub fn set_name(self: Arc<Self>, name: String) -> Arc<Self> {
        self.set(|builder| builder.set_name(&name))
    }

    pub fn set_authentication_signing_factor(
        self: Arc<Self>,
        new: Option<FactorSourceID>,
    ) -> Arc<Self> {
        self.set(|builder| {
            builder.set_authentication_signing_factor(
                new.clone().map(|x| x.into_internal()),
            )
        })
    }

    pub fn remove_factor_from_all_roles(
        self: Arc<Self>,
        factor_source_id: FactorSourceID,
    ) -> Arc<Self> {
        self.set(|builder| {
            builder
                .remove_factor_from_all_roles(factor_source_id.clone().into())
        })
    }

    pub fn remove_factor_from_primary(
        self: Arc<Self>,
        factor_source_id: FactorSourceID,
        factor_list_kind: FactorListKind,
    ) -> Arc<Self> {
        self.set(|builder| {
            builder.remove_factor_from_primary(
                factor_source_id.clone().into(),
                factor_list_kind.into(),
            )
        })
    }

    pub fn remove_all_factors_from_primary_override(
        self: Arc<Self>,
    ) -> Arc<Self> {
        self.set(|builder| builder.remove_all_factors_from_primary_override())
    }

    pub fn remove_factor_from_recovery(
        self: Arc<Self>,
        factor_source_id: FactorSourceID,
    ) -> Arc<Self> {
        self.set(|builder| {
            builder.remove_factor_from_recovery(factor_source_id.clone().into())
        })
    }

    pub fn remove_factor_from_confirmation(
        self: Arc<Self>,
        factor_source_id: FactorSourceID,
    ) -> Arc<Self> {
        self.set(|builder| {
            builder.remove_factor_from_confirmation(
                factor_source_id.clone().into(),
            )
        })
    }

    pub fn set_threshold(self: Arc<Self>, threshold: Threshold) -> Arc<Self> {
        self.set(|builder| builder.set_threshold(threshold.into()))
    }

    pub fn set_time_until_delayed_confirmation_is_callable(
        self: Arc<Self>,
        time_period: TimePeriod,
    ) -> Arc<Self> {
        self.set(|builder| {
            builder.set_time_until_delayed_confirmation_is_callable(
                time_period.into(),
            )
        })
    }

    /// Adds the factor source to the primary role threshold list.
    ///
    /// Also sets the threshold to 1 if this is the first factor set and if
    /// the threshold was 0.
    pub fn add_factor_source_to_primary_threshold(
        self: Arc<Self>,
        factor_source_id: FactorSourceID,
    ) -> Arc<Self> {
        self.set(|builder| {
            builder.add_factor_source_to_primary_threshold(
                factor_source_id.clone().into(),
            )
        })
    }

    pub fn add_factor_source_to_primary_override(
        self: Arc<Self>,
        factor_source_id: FactorSourceID,
    ) -> Arc<Self> {
        self.set(|builder| {
            builder.add_factor_source_to_primary_override(
                factor_source_id.clone().into(),
            )
        })
    }

    pub fn add_factor_source_to_recovery_override(
        self: Arc<Self>,
        factor_source_id: FactorSourceID,
    ) -> Arc<Self> {
        self.set(|builder| {
            builder.add_factor_source_to_recovery_override(
                factor_source_id.clone().into(),
            )
        })
    }

    pub fn add_factor_source_to_confirmation_override(
        self: Arc<Self>,
        factor_source_id: FactorSourceID,
    ) -> Arc<Self> {
        self.set(|builder| {
            builder.add_factor_source_to_confirmation_override(
                factor_source_id.clone().into(),
            )
        })
    }

    pub fn reset_recovery_and_confirmation_role_state(
        self: Arc<Self>,
    ) -> Arc<Self> {
        self.set(|builder| builder.reset_recovery_and_confirmation_role_state())
    }
}

#[uniffi::export]
impl SecurityShieldBuilder {
    /// "Statically" queries which FactorSourceKinds are disallowed for authentication signing.
    pub fn disallowed_factor_source_kinds_for_authentication_signing(
        &self,
    ) -> Vec<FactorSourceKind> {
        sargon::SecurityShieldBuilder::disallowed_factor_source_kinds_for_authentication_signing().into_iter().filter(|f| f.is_supported()).into_type()
    }

    /// "Statically" queries which FactorSourceKinds are allowed for authentication signing.
    pub fn allowed_factor_source_kinds_for_authentication_signing(
        &self,
    ) -> Vec<FactorSourceKind> {
        sargon::SecurityShieldBuilder::allowed_factor_source_kinds_for_authentication_signing().into_iter().filter(|f| f.is_supported()).into_type()
    }

    /// "Statically" queries if `factor_source_kind`` is allowed for authentication signing.
    pub fn is_allowed_factor_source_kind_for_authentication_signing(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> bool {
        sargon::SecurityShieldBuilder::is_allowed_factor_source_kind_for_authentication_signing(
                factor_source_kind.clone().into())
    }

    pub fn addition_of_factor_source_of_kind_to_primary_threshold_is_fully_valid(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> bool {
        self.get(|builder|
            builder.addition_of_factor_source_of_kind_to_primary_threshold_is_fully_valid(
                factor_source_kind.clone().into(),
            )
        )
    }

    pub fn addition_of_factor_source_of_kind_to_primary_threshold_is_valid_or_can_be(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> bool {
        self.get(|builder|
            builder.addition_of_factor_source_of_kind_to_primary_threshold_is_valid_or_can_be(
                factor_source_kind.clone().into(),
            )
        )
    }

    pub fn addition_of_factor_source_of_kind_to_primary_override_is_fully_valid(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> bool {
        self.get(|builder|
            builder.addition_of_factor_source_of_kind_to_primary_override_is_fully_valid(
                factor_source_kind.clone().into(),
            )
        )
    }

    pub fn addition_of_factor_source_of_kind_to_primary_override_is_valid_or_can_be(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> bool {
        self.get(|builder|
            builder.addition_of_factor_source_of_kind_to_primary_override_is_valid_or_can_be(
                factor_source_kind.clone().into(),
            )
        )
    }

    pub fn addition_of_factor_source_of_kind_to_recovery_is_valid_or_can_be(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> bool {
        self.get(|builder|
            builder.addition_of_factor_source_of_kind_to_recovery_is_valid_or_can_be(
                factor_source_kind.clone().into(),
            )
        )
    }

    pub fn addition_of_factor_source_of_kind_to_recovery_is_fully_valid(
        &self,
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
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> bool {
        self.get(|builder|
            builder.addition_of_factor_source_of_kind_to_confirmation_is_valid_or_can_be(
                factor_source_kind.clone().into(),
            )
        )
    }

    pub fn addition_of_factor_source_of_kind_to_confirmation_is_fully_valid(
        &self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
    pub fn auto_assign_factors_to_recovery_and_confirmation_based_on_primary(
        self: Arc<Self>,
        all_factors: Vec<FactorSource>,
    ) -> Arc<Self> {
        builder_arc_map(self, |builder| {
            let _ = builder.wrapped
                .auto_assign_factors_to_recovery_and_confirmation_based_on_primary(
                    all_factors.into_internal(),
                );
        })
    }
}

#[uniffi::export]
impl SecurityShieldBuilder {
    pub fn validate_role_in_isolation(
        &self,
        role: RoleKind,
    ) -> Option<SecurityShieldBuilderRuleViolation> {
        self.get(|builder| {
            builder
                .validate_role_in_isolation(role.into_internal())
                .map(|x| x.into())
        })
    }

    pub fn status(&self) -> SecurityShieldBuilderStatus {
        self.get(|builder| builder.status().into())
    }

    pub fn selected_primary_threshold_factors_status(
        &self,
    ) -> SelectedPrimaryThresholdFactorsStatus {
        self.get(|builder| {
            builder.selected_primary_threshold_factors_status().into()
        })
    }

    pub fn sorted_factor_sources_for_primary_threshold_selection(
        &self,
        factor_sources: Vec<FactorSource>,
    ) -> Vec<FactorSource> {
        self.get(|builder| {
            builder
                .sorted_factor_sources_for_primary_threshold_selection(
                    factor_sources.clone().into_internal(),
                )
                .into_type()
        })
    }

    pub fn build(
        &self,
    ) -> Result<
        SecurityStructureOfFactorSourceIDs,
        SecurityShieldBuilderRuleViolation,
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

    // pub fn sample_trusted_contact() -> Self {
    //     Self::new(sargon::FactorSourceID::sample_trusted_contact())
    // }

    pub fn sample_arculus() -> Self {
        Self::new(sargon::FactorSourceID::sample_arculus())
    }

    pub fn sample_arculus_other() -> Self {
        Self::new(sargon::FactorSourceID::sample_arculus_other())
    }

    pub fn sample_password() -> Self {
        Self::new(sargon::FactorSourceID::sample_password())
    }

    pub fn sample_password_other() -> Self {
        Self::new(sargon::FactorSourceID::sample_password_other())
    }

    pub fn sample_off_device() -> Self {
        Self::new(sargon::FactorSourceID::sample_off_device())
    }
}

impl FactorSource {
    pub fn new(inner: impl Borrow<sargon::FactorSource>) -> Self {
        Self::from(inner.borrow().clone())
    }
}

#[cfg(test)]
impl FactorSource {
    pub fn id(&self) -> FactorSourceID {
        use sargon::BaseBaseIsFactorSource;
        self.clone().into_internal().factor_source_id().into()
    }

    pub fn sample_device() -> Self {
        Self::new(sargon::FactorSource::sample_device())
    }
    pub fn sample_password() -> Self {
        Self::new(sargon::FactorSource::sample_password())
    }
    // pub fn sample_trusted_contact_frank() -> Self {
    //     Self::new(sargon::FactorSource::sample_trusted_contact_frank())
    // }
    pub fn sample_device_babylon() -> Self {
        Self::new(sargon::FactorSource::sample_device_babylon())
    }
    pub fn sample_device_babylon_other() -> Self {
        Self::new(sargon::FactorSource::sample_device_babylon_other())
    }
    pub fn sample_ledger() -> Self {
        Self::new(sargon::FactorSource::sample_ledger())
    }
    pub fn sample_arculus() -> Self {
        Self::new(sargon::FactorSource::sample_arculus())
    }
    pub fn sample_arculus_other() -> Self {
        Self::new(sargon::FactorSource::sample_arculus_other())
    }
    pub fn sample_ledger_other() -> Self {
        Self::new(sargon::FactorSource::sample_ledger_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityShieldBuilder;

    // TODO: We should uncomment every reference in tests to TrustedContact and SecurityQuestions once they are supported
    #[test]
    fn rola() {
        let sut = SUT::new();
        // UniFFI list doesn't include unsupported kinds (TrustedContact & SecurityQuestions)
        assert_eq!(sut.disallowed_factor_source_kinds_for_authentication_signing().len() + 2, sargon::SecurityShieldBuilder::disallowed_factor_source_kinds_for_authentication_signing().len());

        assert_eq!(sut.allowed_factor_source_kinds_for_authentication_signing().len(), sargon::SecurityShieldBuilder::allowed_factor_source_kinds_for_authentication_signing().len());

        assert!(
            sut.is_allowed_factor_source_kind_for_authentication_signing(
                FactorSourceKind::Device
            )
        );
        assert!(
            !sut.is_allowed_factor_source_kind_for_authentication_signing(
                FactorSourceKind::Password
            )
        );

        // assert!(
        //     !sut.is_allowed_factor_source_kind_for_authentication_signing(
        //         FactorSourceKind::TrustedContact
        //     )
        // );
        // assert!(
        //     !sut.is_allowed_factor_source_kind_for_authentication_signing(
        //         FactorSourceKind::SecurityQuestions
        //     )
        // );
    }

    #[test]
    fn test() {
        let mut sut = SUT::new();

        assert_eq!(sut.clone().get_name(), "My Shield");
        sut = sut.set_name("S.H.I.E.L.D.".to_owned());

        assert_eq!(
            time_period_to_days(
                &sut.clone().get_time_until_timed_confirmation_is_callable()
            ),
            14
        );
        sut = sut.set_time_until_delayed_confirmation_is_callable(
            new_time_period_with_days(u16::MAX),
        );
        assert_eq!(
            time_period_to_days(
                &sut.clone().get_time_until_timed_confirmation_is_callable()
            ),
            u16::MAX
        );
        // Primary
        let sim_prim =
            sut.clone().validation_for_addition_of_factor_source_to_primary_override_for_each(vec![
                FactorSourceID::sample_arculus(),
            ]);

        let sim_prim_threshold = sut
            .clone()
            .validation_for_addition_of_factor_source_to_primary_threshold_for_each(vec![
                FactorSourceID::sample_arculus(),
            ]);

        let sim_kind_prim = sut
            .clone()
            .addition_of_factor_source_of_kind_to_primary_override_is_fully_valid(
                FactorSourceKind::Device,
            );

        let sim_kind_prim_threshold = sut
            .clone()
            .addition_of_factor_source_of_kind_to_primary_threshold_is_fully_valid(
                FactorSourceKind::Device,
            );

        // Threshold increases when adding a factor, because it's being set to All by default
        sut = sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_device(),
        );
        assert_eq!(sut.clone().get_primary_threshold(), Threshold::All);

        sut = sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_password_other(),
        );
        assert_eq!(sut.clone().get_primary_threshold(), Threshold::All);
        assert_eq!(
            sut.clone().get_primary_threshold_values(),
            vec![Threshold::All, Threshold::Specific(1)]
        );

        sut = sut
            .remove_factor_from_primary(
                FactorSourceID::sample_device(),
                FactorListKind::Threshold,
            )
            .remove_factor_from_primary(
                FactorSourceID::sample_password_other(),
                FactorListKind::Threshold,
            );

        // Setting a specific threshold explicitly
        sut = sut.set_threshold(Threshold::Specific(1));

        sut = sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_device(),
        );
        assert_eq!(sut.clone().get_primary_threshold(), Threshold::Specific(1));

        sut = sut.add_factor_source_to_primary_threshold(
            // should NOT bump threshold
            FactorSourceID::sample_password_other(),
        );
        assert_eq!(sut.clone().get_primary_threshold(), Threshold::Specific(1));
        sut = sut.remove_factor_from_primary(
            FactorSourceID::sample_password_other(),
            FactorListKind::Threshold,
        );

        assert_eq!(
            sut.clone().get_primary_threshold_factors(),
            vec![FactorSourceID::sample_device()]
        );
        sut = sut
            .add_factor_source_to_primary_override(
                FactorSourceID::sample_arculus(),
            )
            .add_factor_source_to_primary_override(
                FactorSourceID::sample_arculus_other(),
            );

        assert_eq!(
            sut.clone().get_primary_override_factors(),
            vec![
                FactorSourceID::sample_arculus(),
                FactorSourceID::sample_arculus_other()
            ]
        );

        sut = sut.remove_all_factors_from_primary_override();
        assert!(sut.clone().get_primary_override_factors().is_empty());

        sut = sut
            .add_factor_source_to_primary_override(
                FactorSourceID::sample_arculus(),
            )
            .add_factor_source_to_primary_override(
                FactorSourceID::sample_arculus_other(),
            );

        // Recovery
        let sim_rec =
            sut.clone().validation_for_addition_of_factor_source_to_recovery_override_for_each(vec![
                FactorSourceID::sample_ledger(),
            ]);

        let sim_kind_rec = sut
            .clone()
            .addition_of_factor_source_of_kind_to_recovery_is_fully_valid(
                FactorSourceKind::ArculusCard,
            );

        sut = sut
            .add_factor_source_to_recovery_override(
                FactorSourceID::sample_ledger(),
            )
            .add_factor_source_to_recovery_override(
                FactorSourceID::sample_ledger_other(),
            );

        assert_eq!(
            sut.clone().get_recovery_factors(),
            vec![
                FactorSourceID::sample_ledger(),
                FactorSourceID::sample_ledger_other()
            ]
        );
        sut = sut.reset_recovery_and_confirmation_role_state();
        assert_eq!(sut.clone().get_recovery_factors(), vec![]);

        sut = sut
            .add_factor_source_to_recovery_override(
                FactorSourceID::sample_ledger(),
            )
            .add_factor_source_to_recovery_override(
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
        let sim_conf = sut
            .clone()
            .validation_for_addition_of_factor_source_to_confirmation_override_for_each(vec![
                FactorSourceID::sample_device(),
            ]);

        let sim_kind_conf = sut
            .clone()
            .addition_of_factor_source_of_kind_to_confirmation_is_fully_valid(
                FactorSourceKind::ArculusCard,
            );

        sut = sut.add_factor_source_to_confirmation_override(
            FactorSourceID::sample_device(),
        );

        assert_eq!(
            sut.clone().get_confirmation_factors(),
            vec![FactorSourceID::sample_device(),]
        );

        assert_ne!(
            sim_prim,
            sut.clone().validation_for_addition_of_factor_source_to_primary_override_for_each(
                vec![
                FactorSourceID::sample_arculus(),
            ])
        );

        assert_eq!( // we use lenient builder, so we say state has not changed
            sim_prim_threshold,
            sut.clone().validation_for_addition_of_factor_source_to_primary_threshold_for_each(
                vec![
                FactorSourceID::sample_arculus()
            ])
        );

        assert_ne!(
            sim_rec,
            sut.clone().validation_for_addition_of_factor_source_to_recovery_override_for_each(
                vec![
                    FactorSourceID::sample_ledger(),
                    ])
                );

        assert_ne!(
                    sim_conf,
                    sut.clone().validation_for_addition_of_factor_source_to_confirmation_override_for_each(
                vec![
                FactorSourceID::sample_device(),
            ])
        );

        assert_eq!(
            sim_kind_prim,
            sut.clone().addition_of_factor_source_of_kind_to_primary_override_is_fully_valid(
                FactorSourceKind::Device,
            )
        );

        assert_eq!(
            sim_kind_prim_threshold,
            sut.clone().addition_of_factor_source_of_kind_to_primary_threshold_is_fully_valid(
                FactorSourceKind::Device,
            )
        );

        assert_eq!(
            sim_kind_rec,
            sut.clone()
                .addition_of_factor_source_of_kind_to_recovery_is_fully_valid(
                    FactorSourceKind::ArculusCard,
                )
        );

        assert_eq!(
            sim_kind_conf,
            sut.clone().addition_of_factor_source_of_kind_to_confirmation_is_fully_valid(
                FactorSourceKind::ArculusCard,
            )
        );

        sut = sut
            .remove_factor_from_all_roles(
                FactorSourceID::sample_arculus_other()
            )
            .remove_factor_from_all_roles(
                FactorSourceID::sample_ledger_other(),
            );

        let f = FactorSourceID::sample_ledger_other();
        let xs = sut.clone().get_primary_override_factors();
        sut = sut
            .add_factor_source_to_primary_override(f.clone())
            .remove_factor_from_primary(f.clone(), FactorListKind::Override);
        assert_eq!(xs, sut.clone().get_primary_override_factors());

        let xs = sut.clone().get_recovery_factors();
        sut = sut
            .add_factor_source_to_recovery_override(f.clone())
            .remove_factor_from_recovery(f.clone());
        assert_eq!(xs, sut.clone().get_recovery_factors());

        let xs = sut.clone().get_confirmation_factors();
        sut = sut
            .add_factor_source_to_confirmation_override(f.clone())
            .remove_factor_from_confirmation(f.clone());
        assert_eq!(xs, sut.clone().get_confirmation_factors());

        assert_eq!(
            sut.status(),
            SecurityShieldBuilderStatus::Invalid {
                reason: SecurityShieldBuilderStatusInvalidReason {
                    is_primary_role_factor_list_empty: false,
                    is_recovery_role_factor_list_empty: false,
                    is_confirmation_role_factor_list_empty: false,
                    is_auth_signing_factor_missing: true
                }
            }
        );
        sut = sut.set_authentication_signing_factor(Some(
            FactorSourceID::sample_device_other(),
        ));
        assert_eq!(
            sut.get_authentication_signing_factor(),
            Some(FactorSourceID::sample_device_other())
        );

        let s0 = sut.status();
        let s1 = sut.status(); // can call status many times!
        assert_eq!(s0, s1);

        let shield0 = sut.clone().build().unwrap();
        let shield = sut.clone().build().unwrap(); // can call build many times!
        assert_eq!(shield0, shield);

        assert_eq!(
            shield.authentication_signing_factor,
            FactorSourceID::sample_device_other()
        );
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

    #[test]
    fn auto_assign() {
        let mut sut = SUT::new();
        let all_factors_in_profile = vec![
            FactorSource::sample_password(),
            // FactorSource::sample_trusted_contact_frank(),
            FactorSource::sample_device_babylon(),
            FactorSource::sample_device_babylon_other(),
            FactorSource::sample_ledger(),
            FactorSource::sample_arculus(),
            FactorSource::sample_arculus_other(),
            FactorSource::sample_ledger_other(),
        ];
        let name = "Auto Built";
        let days_until_timed_confirmation = TimePeriod {
            value: 237,
            unit: TimePeriodUnit::Days,
        };
        sut = sut
            .set_name(name.to_owned())
            .set_time_until_delayed_confirmation_is_callable(
                new_time_period_with_days(237),
            )
            .add_factor_source_to_primary_threshold(
                FactorSource::sample_device_babylon().id(),
            )
            .add_factor_source_to_primary_threshold(
                FactorSource::sample_ledger().id(),
            )
            .auto_assign_factors_to_recovery_and_confirmation_based_on_primary(
                all_factors_in_profile.clone(),
            )
            .set_threshold(Threshold::Specific(2));

        let shield = sut.clone().build().unwrap();

        assert_eq!(shield.metadata.display_name.value, name.to_owned());
        let matrix = shield.matrix_of_factors;
        assert_eq!(
            matrix.time_until_delayed_confirmation_is_callable,
            days_until_timed_confirmation
        );

        pretty_assertions::assert_eq!(
            matrix.primary_role,
            PrimaryRoleWithFactorSourceIDs {
                threshold: Threshold::Specific(2),
                threshold_factors: vec![
                    FactorSourceID::sample_device(),
                    FactorSourceID::sample_ledger()
                ],
                override_factors: Vec::new()
            }
        );

        pretty_assertions::assert_eq!(
            matrix.recovery_role,
            RecoveryRoleWithFactorSourceIDs {
                threshold: Threshold::All,
                threshold_factors: Vec::new(),
                override_factors: vec![
                    // FactorSourceID::sample_trusted_contact(),
                    FactorSourceID::sample_ledger(),
                    FactorSourceID::sample_arculus(), // will be sample_arculus_other once we include sample_trusted_contact
                    FactorSourceID::sample_ledger_other(),
                ]
            }
        );

        pretty_assertions::assert_eq!(
            matrix.confirmation_role,
            ConfirmationRoleWithFactorSourceIDs {
                threshold: Threshold::All,
                threshold_factors: Vec::new(),
                override_factors: vec![
                    FactorSourceID::sample_password(),
                    FactorSourceID::sample_arculus_other(), // will be sample_arculus once we include sample_trusted_contact
                    FactorSourceID::sample_device(),
                    FactorSourceID::sample_device_other(),
                ]
            }
        );
    }

    // #[test]
    // fn primary_override_validation_status_trusted_contact() {
    //     let sut = SUT::new();
    //     let res = sut.validation_for_addition_of_factor_source_to_primary_override_for_each(
    //         vec![FactorSourceID::sample_trusted_contact()],
    //     );
    //     pretty_assertions::assert_eq!(
    //         res,
    //         vec![
    //             FactorSourceValidationStatus {
    //                 role: RoleKind::Primary,
    //                 factor_source_id: FactorSourceID::sample_trusted_contact(),
    //                 reason_if_invalid: Some(FactorSourceValidationStatusReasonIfInvalid::NonBasic(
    //                     SecurityShieldBuilderRuleViolation::PrimaryCannotContainTrustedContact
    //                 ))
    //             }
    //         ]
    //     )
    // }
}
