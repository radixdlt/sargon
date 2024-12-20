use crate::prelude::*;

/// The mode of the shield builder, either `Lenient` or `Strict`, this has
/// no effect on the validation or building of the shield, which is always
/// strict. It only affects the incremental changing of the state before
/// the builder tries to build the shield (or validate the builders state).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityShieldBuilderMode {
    /// The most strict mode of the shield builder, which forbids many
    /// mutations, e.g. adding same factor two primary threshold and override,
    /// or adding multiple device factor sources to the primary etc.
    Strict,

    /// The lenient mode of the shield builder, which allows many otherwise
    /// forbidden mutations. Host might wanna use this if they wanna defer
    /// validation of the shield to the last point.
    ///
    /// This does NOT mean that the shield will be lenient, only that the
    /// incremental changes to the builder are lenient.
    Lenient,
}

#[derive(Debug)]
pub struct SecurityShieldBuilder {
    /// If the builder is acting in lenient or strict mode. This does not
    /// have any affect on the validation or building of the shield, which is
    /// always strict. It only affects the incremental changing of the state.
    mode: SecurityShieldBuilderMode,

    /// The underlying matrix builder
    matrix_builder: RwLock<MatrixBuilder>,

    /// The "ROLA" factor, used to sign authentication requests, it is always single
    /// factor - therefore not part of the matrix_builder. When built it must be set,
    /// and adhere to validation of `Self::is_allowed_factor_source_kind_for_authentication_signing`.
    authentication_signing_factor: RwLock<Option<FactorSourceID>>,

    /// The name of the shield, defaults to some valid value
    name: RwLock<String>,

    // We eagerly set this, and we use it inside the `build` method, ensuring
    // that for the same *state* of `MatrixBuilder` we always have the same shield!
    shield_id: SecurityStructureID,

    // We eagerly set this, and we use it inside the `build` method, ensuring
    // that for the same *state* of `MatrixBuilder` we always have the same shield!
    created_on: Timestamp,
}

impl Default for SecurityShieldBuilder {
    fn default() -> Self {
        Self::lenient()
    }
}

impl SecurityShieldBuilder {
    pub fn new(mode: SecurityShieldBuilderMode) -> Self {
        let matrix_builder = MatrixBuilder::new();
        let name = RwLock::new("My Shield".to_owned());
        Self {
            mode,
            matrix_builder: RwLock::new(matrix_builder),
            name,
            authentication_signing_factor: RwLock::new(None),
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
        let input = &factor_sources.into_iter().collect::<IndexSet<_>>();
        self.get(|builder| call(builder, input))
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
            factors.to_vec()
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

    pub fn get_authentication_signing_factor(&self) -> Option<FactorSourceID> {
        *self.authentication_signing_factor.read().unwrap()
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

    /// Sets the ROLA (authentication signing) factor to `new` if and only if
    /// `new` is not Some(invalid), where invalid is defined by `allowed_factor_source_kinds_for_authentication_signing`,
    /// that is, it checks the `FactorSourceKind` of the factor, according to the
    /// rules defined in [doc][doc].
    ///
    /// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shield
    pub fn set_authentication_signing_factor(
        &self,
        new: impl Into<Option<FactorSourceID>>,
    ) -> &Self {
        let new = new.into();
        if let Some(new) = new.as_ref() {
            if !Self::is_allowed_factor_source_kind_for_authentication_signing(
                new.get_factor_source_kind(),
            ) {
                warn!("Invalid FactorSourceKind for ROLA");
                return self;
            }
        }
        *self.authentication_signing_factor.write().unwrap() = new;
        self
    }

    /// Adds the factor source to the primary role threshold list.
    ///
    /// Also sets the threshold to 1 this is the first factor set and if
    /// the threshold was 0.
    pub fn add_factor_source_to_primary_threshold(
        &self,
        factor_source_id: FactorSourceID,
    ) -> &Self {
        self.set(|builder| {
            let res = builder.add_factor_source_to_primary_threshold_with_mode(
                factor_source_id,
                self.mode,
            );
            debug!(
                "Add FactorSource to PrimaryRole (threshold) result: {:?}",
                res
            );
        })
    }

    pub fn add_factor_source_to_primary_override(
        &self,
        factor_source_id: FactorSourceID,
    ) -> &Self {
        self.set(|builder| {
            let res = builder.add_factor_source_to_primary_override_with_mode(
                factor_source_id,
                self.mode,
            );
            debug!(
                "Add FactorSource to PrimaryRole (override) result: {:?}",
                res
            );
        })
    }

    /// Removes the factor from all relevant roles
    pub fn remove_factor_from_all_roles(
        &self,
        factor_source_id: FactorSourceID,
    ) -> &Self {
        self.set(|builder| {
            builder.remove_factor_from_all_roles(&factor_source_id)
        })
    }

    /// Removes factor **only** from the primary role.
    pub fn remove_factor_from_primary(
        &self,
        factor_source_id: FactorSourceID,
    ) -> &Self {
        self.set(|builder| {
            builder.remove_factor_from_primary(&factor_source_id)
        })
    }

    /// Removes factor **only** from the recovery role.
    pub fn remove_factor_from_recovery(
        &self,
        factor_source_id: FactorSourceID,
    ) -> &Self {
        self.set(|builder| {
            builder.remove_factor_from_recovery(&factor_source_id)
        })
    }

    /// Removes factor **only** from the confirmation role.
    pub fn remove_factor_from_confirmation(
        &self,
        factor_source_id: FactorSourceID,
    ) -> &Self {
        self.set(|builder| {
            builder.remove_factor_from_confirmation(&factor_source_id)
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
            let res = builder.add_factor_source_to_recovery_override_with_mode(
                factor_source_id,
                self.mode,
            );
            debug!("Add FactorSource to RecoveryRole result: {:?}", res);
        })
    }

    pub fn add_factor_source_to_confirmation_override(
        &self,
        factor_source_id: FactorSourceID,
    ) -> &Self {
        self.set(|builder| {
            let res = builder
                .add_factor_source_to_confirmation_override_with_mode(
                    factor_source_id,
                    self.mode,
                );
            debug!("Add FactorSource to ConfirmationRole result: {:?}", res);
        })
    }

    pub fn reset_recovery_and_confirmation_role_state(&self) -> &Self {
        self.set(|builder| {
            builder.reset_recovery_and_confirmation_role_state();
        })
    }

    pub(crate) fn reset_factors_in_roles(&self) -> &Self {
        self.set(|builder| {
            builder.reset_factors_in_roles();
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
    pub fn disallowed_factor_source_kinds_for_authentication_signing(
    ) -> IndexSet<FactorSourceKind> {
        IndexSet::from_iter([
            FactorSourceKind::Password,
            FactorSourceKind::SecurityQuestions,
            FactorSourceKind::TrustedContact,
        ])
    }

    pub fn allowed_factor_source_kinds_for_authentication_signing(
    ) -> IndexSet<FactorSourceKind> {
        let all = FactorSourceKind::all();
        let disallowed =
            Self::disallowed_factor_source_kinds_for_authentication_signing();
        all.difference(&disallowed).cloned().collect()
    }

    pub fn is_allowed_factor_source_kind_for_authentication_signing(
        factor_source_kind: FactorSourceKind,
    ) -> bool {
        Self::allowed_factor_source_kinds_for_authentication_signing()
            .contains(&factor_source_kind)
    }

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
                builder.validation_for_addition_of_factor_source_to_primary_threshold_for_each_with_mode(input, self.mode)
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
                builder.validation_for_addition_of_factor_source_to_primary_override_for_each_with_mode(input, self.mode)
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
                    .validation_for_addition_of_factor_source_to_recovery_override_for_each_with_mode(input, self.mode)
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
                builder.validation_for_addition_of_factor_source_to_confirmation_override_for_each_with_mode(
                    input,
                    self.mode
                )
            },
        )
    }
}

impl SecurityShieldBuilder {
    /// `None` means valid!
    pub fn validate(&self) -> Option<SecurityShieldBuilderInvalidReason> {
        if DisplayName::new(self.get_name()).is_err() {
            return Some(SecurityShieldBuilderInvalidReason::ShieldNameInvalid);
        }

        if let Some(matrix_invalid_reason) = self.get(|builder| {
            let r = builder.validate();
            r.as_shield_validation()
        }) {
            return Some(matrix_invalid_reason);
        }

        if self.get_authentication_signing_factor().is_none() {
            return Some(
                SecurityShieldBuilderInvalidReason::MissingAuthSigningFactor,
            );
        }

        None
    }

    /// Validates **just** the primary role **in isolation**.
    pub fn validate_primary_role(
        &self,
    ) -> Option<SecurityShieldBuilderInvalidReason> {
        self.validate_role_in_isolation(RoleKind::Primary)
    }

    /// `None` means valid!
    pub fn validate_role_in_isolation(
        &self,
        role: RoleKind,
    ) -> Option<SecurityShieldBuilderInvalidReason> {
        self.get(|builder| {
            let validation = match role {
                RoleKind::Primary => {
                    builder.validate_primary_role_in_isolation()
                }
                RoleKind::Recovery => {
                    builder.validate_recovery_role_in_isolation()
                }
                RoleKind::Confirmation => {
                    builder.validate_confirmation_role_in_isolation()
                }
            };
            validation.as_shield_validation()
        })
    }

    pub fn selected_factor_sources_for_role_status(
        &self,
        role: RoleKind,
    ) -> SelectedFactorSourcesForRoleStatus {
        // Validate the role in isolation
        if let Some(reason) = self.validate_role_in_isolation(role) {
            return match reason {
                SecurityShieldBuilderInvalidReason::PrimaryRoleMustHaveAtLeastOneFactor
                | SecurityShieldBuilderInvalidReason::RecoveryRoleMustHaveAtLeastOneFactor
                | SecurityShieldBuilderInvalidReason::ConfirmationRoleMustHaveAtLeastOneFactor => {
                    SelectedFactorSourcesForRoleStatus::Insufficient
                }
                _ => SelectedFactorSourcesForRoleStatus::Invalid,
            };
        }

        // Check conditions for Primary role
        let primary_factors_len =
            self.get(|builder| builder.get_primary_threshold_factors().len());
        if role == RoleKind::Primary && primary_factors_len < 2 {
            SelectedFactorSourcesForRoleStatus::Suboptimal
        } else {
            SelectedFactorSourcesForRoleStatus::Optimal
        }
    }

    pub fn sorted_factor_sources_for_primary_threshold_selection(
        &self,
        factor_sources: Vec<FactorSource>,
    ) -> Vec<FactorSource> {
        let mut factor_sources: Vec<FactorSource> = factor_sources
            .into_iter()
            .filter(|fs| self.addition_of_factor_source_of_kind_to_primary_threshold_is_valid_or_can_be(fs.factor_source_kind()))
            .collect();
        factor_sources.sort_by_key(|fs| {
            fs.factor_source_kind()
                .display_order_for_primary_threshold_selection()
        });

        factor_sources
    }

    pub fn build(
        &self,
    ) -> Result<
        SecurityStructureOfFactorSourceIds,
        SecurityShieldBuilderInvalidReason,
    > {
        let authentication_signing_factor =
            self.get_authentication_signing_factor().ok_or(
                SecurityShieldBuilderInvalidReason::MissingAuthSigningFactor,
            )?;
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
            authentication_signing_factor,
        };
        Ok(shield)
    }
}

impl SecurityShieldBuilder {
    pub fn strict() -> Self {
        Self::new(SecurityShieldBuilderMode::Strict)
    }

    pub fn lenient() -> Self {
        Self::new(SecurityShieldBuilderMode::Lenient)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityShieldBuilder;

    #[test]
    fn default_is_lenient() {
        assert_eq!(SUT::default().mode, SecurityShieldBuilderMode::Lenient);
    }

    #[test]
    fn mode_of_lenient() {
        assert_eq!(SUT::lenient().mode, SecurityShieldBuilderMode::Lenient);
    }

    #[test]
    fn mode_of_strict() {
        assert_eq!(SUT::strict().mode, SecurityShieldBuilderMode::Strict);
    }

    #[test]
    fn add_factor_to_primary_threshold_does_not_change_already_set_threshold() {
        let sut = SUT::strict();
        sut.set_threshold(42);
        sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_device(),
        );
        assert_eq!(sut.get_threshold(), 42);
    }

    #[test]
    fn allowed_rola() {
        let allowed =
            SUT::allowed_factor_source_kinds_for_authentication_signing();
        assert_eq!(
            allowed,
            IndexSet::<FactorSourceKind>::from_iter([
                FactorSourceKind::LedgerHQHardwareWallet,
                FactorSourceKind::ArculusCard,
                FactorSourceKind::OffDeviceMnemonic,
                FactorSourceKind::Device,
            ])
        );
    }

    #[test]
    fn is_allowed_rola() {
        let disallowed =
            SUT::disallowed_factor_source_kinds_for_authentication_signing();
        assert!(disallowed.iter().all(|k| {
            !SUT::is_allowed_factor_source_kind_for_authentication_signing(*k)
        }));
    }

    #[test]
    fn test_invalid_rola_kind_does_not_change_rola() {
        let sut = SUT::strict();
        assert!(sut.get_authentication_signing_factor().is_none());
        let valid = FactorSourceID::sample_device();
        sut.set_authentication_signing_factor(valid);
        assert_eq!(sut.get_authentication_signing_factor().unwrap(), valid);

        let invalid_factors = vec![
            FactorSourceID::sample_password(),
            FactorSourceID::sample_security_questions(),
            FactorSourceID::sample_trusted_contact(),
        ];
        for invalid in invalid_factors {
            sut.set_authentication_signing_factor(invalid); // should not have changed anything
        }
        assert_eq!(sut.get_authentication_signing_factor().unwrap(), valid);
    }

    #[test]
    fn test() {
        let sut = SUT::default();

        let _ = sut
            .set_name("S.H.I.E.L.D.")
            .set_authentication_signing_factor(Some(
                FactorSourceID::sample_device(),
            ))
            // Primary
            .set_number_of_days_until_auto_confirm(42)
            .add_factor_source_to_primary_threshold(
                // also sets threshold -> 1
                FactorSourceID::sample_device(),
            )
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
            .remove_factor_from_primary(FactorSourceID::sample_arculus_other())
            .remove_factor_from_recovery(FactorSourceID::sample_ledger_other());

        let shield0 = sut.build().unwrap();
        let shield = sut.build().unwrap();
        pretty_assertions::assert_eq!(shield0, shield);

        assert_eq!(shield.metadata.display_name.value, "S.H.I.E.L.D.");
        assert_eq!(
            shield.matrix_of_factors.primary().get_override_factors(),
            &vec![FactorSourceID::sample_arculus()]
        );
        assert_eq!(shield.matrix_of_factors.primary().get_threshold(), 1);
        assert_eq!(
            shield.matrix_of_factors.recovery().get_override_factors(),
            &vec![FactorSourceID::sample_ledger()]
        );
        assert_eq!(
            shield
                .matrix_of_factors
                .confirmation()
                .get_override_factors(),
            &vec![FactorSourceID::sample_device()]
        );
    }

    fn test_addition_of_factor_source_of_kind_to_primary(
        list_kind: FactorListKind,
        is_fully_valid: impl Fn(&SUT, FactorSourceKind) -> bool,
        can_be: impl Fn(&SUT, FactorSourceKind) -> bool,
        add: impl Fn(&SUT, FactorSourceID) -> &SUT,
    ) {
        let sut_owned = SUT::strict();
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
        sut.remove_factor_from_all_roles(FactorSourceID::sample_device());

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
        let sut = SUT::strict();

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
        let sut = SUT::strict();

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
        let sut = SUT::strict();

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
        let sut = SUT::strict();

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
        let sut = SUT::strict();
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

    #[test]
    fn test_sorted_factor_sources_for_primary_threshold_selection() {
        let sut = SUT::strict();
        let factor_sources = FactorSource::sample_values_all();
        let expected = vec![
            FactorSource::sample_device_babylon(),
            FactorSource::sample_device_babylon_other(),
            FactorSource::sample_device_olympia(),
            FactorSource::sample_arculus(),
            FactorSource::sample_arculus_other(),
            FactorSource::sample_ledger(),
            FactorSource::sample_ledger_other(),
            FactorSource::sample_password(),
            FactorSource::sample_password_other(),
            FactorSource::sample_off_device(),
            FactorSource::sample_off_device_other(),
        ];
        assert_eq!(
            sut.sorted_factor_sources_for_primary_threshold_selection(
                factor_sources
            ),
            expected
        )
    }

    #[test]
    fn selected_factor_sources_for_role_status_is_optimal() {
        let sut = SUT::default();

        let _ = sut
            .set_threshold(2)
            .add_factor_source_to_primary_threshold(
                FactorSourceID::sample_password(),
            )
            .add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device(),
            );
        let status =
            sut.selected_factor_sources_for_role_status(RoleKind::Primary);

        pretty_assertions::assert_eq!(
            status,
            SelectedFactorSourcesForRoleStatus::Optimal
        );
    }

    #[test]
    fn selected_factor_sources_for_role_status_is_suboptimal() {
        let sut = SUT::default();

        let _ = sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_ledger(),
        );
        let status =
            sut.selected_factor_sources_for_role_status(RoleKind::Primary);

        pretty_assertions::assert_eq!(
            status,
            SelectedFactorSourcesForRoleStatus::Suboptimal
        );
    }
}

#[cfg(test)]
mod test_invalid {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityShieldBuilder;

    #[test]
    fn primary_role_must_have_at_least_one_factor() {
        let sut = SUT::strict();
        assert_eq!(
            sut.validate().unwrap(),
            SecurityShieldBuilderInvalidReason::PrimaryRoleMustHaveAtLeastOneFactor
        );
        assert_eq!(
            sut.validate_role_in_isolation(RoleKind::Primary).unwrap(),
            SecurityShieldBuilderInvalidReason::PrimaryRoleMustHaveAtLeastOneFactor
        );
    }

    #[test]
    fn primary_role_with_threshold_cannot_be_zero_with_factors() {
        let sut = SUT::strict();
        sut.add_factor_source_to_primary_threshold(
            // bumped threshold
            FactorSourceID::sample_device(),
        );
        assert_eq!(sut.get_threshold(), 1);
        sut.set_threshold(0);
        assert_eq!(
            sut.validate().unwrap(),
            SecurityShieldBuilderInvalidReason::PrimaryRoleWithThresholdFactorsCannotHaveAThresholdValueOfZero
        );
        assert_eq!(
            sut.validate_role_in_isolation(RoleKind::Primary).unwrap(),
            SecurityShieldBuilderInvalidReason::PrimaryRoleWithThresholdFactorsCannotHaveAThresholdValueOfZero
        );
    }

    #[test]
    fn recovery_role_must_have_at_least_one_factor() {
        let sut = SUT::strict();
        sut.add_factor_source_to_primary_override(
            FactorSourceID::sample_device(),
        );
        assert_eq!(
            sut.validate().unwrap(),
            SecurityShieldBuilderInvalidReason::RecoveryRoleMustHaveAtLeastOneFactor
        );
        assert_eq!(
            sut.validate_role_in_isolation(RoleKind::Recovery).unwrap(),
            SecurityShieldBuilderInvalidReason::RecoveryRoleMustHaveAtLeastOneFactor
        );
    }

    #[test]
    fn confirmation_role_must_have_at_least_one_factor() {
        let sut = SUT::strict();
        sut.add_factor_source_to_primary_override(
            FactorSourceID::sample_device(),
        );
        sut.add_factor_source_to_recovery_override(
            FactorSourceID::sample_ledger(),
        );
        assert_eq!(
            sut.validate().unwrap(),
            SecurityShieldBuilderInvalidReason::ConfirmationRoleMustHaveAtLeastOneFactor
        );
        assert_eq!(
            sut.validate_role_in_isolation(RoleKind::Confirmation).unwrap(),
            SecurityShieldBuilderInvalidReason::ConfirmationRoleMustHaveAtLeastOneFactor
        );
    }

    #[test]
    fn valid_is_none() {
        let sut = SUT::strict();
        sut.set_authentication_signing_factor(FactorSourceID::sample_device());
        sut.add_factor_source_to_primary_override(
            FactorSourceID::sample_device(),
        );
        sut.add_factor_source_to_recovery_override(
            FactorSourceID::sample_ledger(),
        );
        sut.add_factor_source_to_confirmation_override(
            FactorSourceID::sample_arculus(),
        );
        assert!(sut.validate().is_none());
    }

    fn valid() -> SUT {
        let sut = SUT::strict();
        sut.add_factor_source_to_primary_override(
            FactorSourceID::sample_device(),
        );
        sut.add_factor_source_to_recovery_override(
            FactorSourceID::sample_ledger(),
        );
        sut.add_factor_source_to_confirmation_override(
            FactorSourceID::sample_arculus(),
        );
        sut.set_authentication_signing_factor(Some(
            FactorSourceID::sample_device(),
        ));
        sut
    }

    #[test]
    fn shield_name_invalid_empty() {
        let sut = valid();
        sut.set_name("");
        assert_eq!(
            sut.validate().unwrap(),
            SecurityShieldBuilderInvalidReason::ShieldNameInvalid
        );
    }

    #[test]
    fn shield_name_truncated_if_too_long() {
        let sut = valid();
        sut.set_name(
            "This shield name's too long and it is going to get truncated",
        );
        let shield = sut.build().unwrap();
        assert_eq!(
            shield.metadata.display_name.value,
            "This shield name's too long an"
        );
    }

    #[test]
    fn number_of_auto_confirm_days_invalid() {
        let sut = valid();
        sut.set_number_of_days_until_auto_confirm(0);
        assert_eq!(
            sut.validate().unwrap(),
            SecurityShieldBuilderInvalidReason::NumberOfDaysUntilAutoConfirmMustBeGreaterThanZero
        );
    }

    #[test]
    fn recovery_and_confirmation_factors_overlap() {
        let sut = SUT::strict();
        sut.add_factor_source_to_primary_override(
            FactorSourceID::sample_device(),
        );

        let same = FactorSourceID::sample_ledger();
        sut.add_factor_source_to_recovery_override(same);
        sut.add_factor_source_to_confirmation_override(
            same, // same factor! not allowed
        );
        assert_eq!(
            sut.validate().unwrap(),
            SecurityShieldBuilderInvalidReason::RecoveryAndConfirmationFactorsOverlap
        );
    }

    #[test]
    fn single_factor_used_in_primary_must_not_be_used_in_any_other_role_in_recovery(
    ) {
        let sut = SUT::strict();
        let same = FactorSourceID::sample_ledger();
        sut.add_factor_source_to_primary_override(same);

        sut.add_factor_source_to_recovery_override(
            same, // same factor! not allowed
        );
        sut.add_factor_source_to_confirmation_override(
            FactorSourceID::sample_arculus(),
        );
        assert_eq!(
            sut.validate().unwrap(),
            SecurityShieldBuilderInvalidReason::SingleFactorUsedInPrimaryMustNotBeUsedInAnyOtherRole
        );
    }

    #[test]
    fn single_factor_used_in_primary_must_not_be_used_in_any_other_role_in_confirmation(
    ) {
        let sut = SUT::strict();
        let same = FactorSourceID::sample_ledger();
        sut.add_factor_source_to_primary_override(same);

        sut.add_factor_source_to_recovery_override(
            FactorSourceID::sample_arculus(),
        );
        sut.add_factor_source_to_confirmation_override(
            same, // same factor! not allowed
        );
        assert_eq!(
            sut.validate().unwrap(),
            SecurityShieldBuilderInvalidReason::SingleFactorUsedInPrimaryMustNotBeUsedInAnyOtherRole
        );
    }

    #[test]
    fn primary_role_with_password_in_threshold_list_must_threshold_greater_than_one(
    ) {
        let sut = SUT::strict();

        sut.add_factor_source_to_recovery_override(
            FactorSourceID::sample_ledger(),
        );
        sut.add_factor_source_to_confirmation_override(
            FactorSourceID::sample_arculus(),
        );

        sut.set_threshold(1);
        sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_password(),
        );
        sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_device(),
        );

        assert_eq!(
            sut.validate().unwrap(),
            SecurityShieldBuilderInvalidReason::PrimaryRoleWithPasswordInThresholdListMustThresholdGreaterThanOne
        );
        assert_eq!(
            sut.validate_role_in_isolation(RoleKind::Primary).unwrap(),
            SecurityShieldBuilderInvalidReason::PrimaryRoleWithPasswordInThresholdListMustThresholdGreaterThanOne
        );
    }

    #[test]
    fn primary_role_with_password_in_threshold_list_must_have_another_factor() {
        let sut = SUT::strict();

        sut.add_factor_source_to_recovery_override(
            FactorSourceID::sample_ledger(),
        );
        sut.add_factor_source_to_confirmation_override(
            FactorSourceID::sample_arculus(),
        );

        sut.set_threshold(1);
        sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_password(),
        );

        assert_eq!(
            sut.validate().unwrap(),
            SecurityShieldBuilderInvalidReason::PrimaryRoleWithPasswordInThresholdListMustHaveAnotherFactor
        );
        assert_eq!(
            sut.validate_role_in_isolation(RoleKind::Primary).unwrap(),
            SecurityShieldBuilderInvalidReason::PrimaryRoleWithPasswordInThresholdListMustHaveAnotherFactor
        );
    }

    #[test]
    fn two_different_password_only_not_valid_for_primary() {
        let sut = SUT::strict();

        sut.add_factor_source_to_recovery_override(
            FactorSourceID::sample_ledger(),
        );
        sut.add_factor_source_to_confirmation_override(
            FactorSourceID::sample_arculus(),
        );

        sut.set_threshold(2);
        sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_password(),
        );
        sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_password_other(),
        );

        assert_eq!(
            sut.validate().unwrap(),
            SecurityShieldBuilderInvalidReason::PrimaryRoleWithPasswordInThresholdListMustHaveAnotherFactor
        );
    }

    #[test]
    fn primary_role_with_password_in_override_does_not_get_added() {
        let sut = SUT::strict();

        sut.add_factor_source_to_recovery_override(
            FactorSourceID::sample_ledger(),
        );
        sut.add_factor_source_to_confirmation_override(
            FactorSourceID::sample_arculus(),
        );

        sut.add_factor_source_to_primary_override(
            FactorSourceID::sample_password(),
        );
        assert!(sut.get_primary_override_factors().is_empty()); // did not get added

        assert_eq!(
            sut.validate().unwrap(),
            SecurityShieldBuilderInvalidReason::PrimaryRoleMustHaveAtLeastOneFactor
        );
        assert_eq!(
            sut.validate_role_in_isolation(RoleKind::Primary).unwrap(),
            SecurityShieldBuilderInvalidReason::PrimaryRoleMustHaveAtLeastOneFactor
        );
    }

    #[test]
    fn template() {
        // use this to create more tests...
        let sut = valid();
        sut.set_name("");
        assert_eq!(
            sut.validate().unwrap(),
            SecurityShieldBuilderInvalidReason::ShieldNameInvalid
        );
    }

    #[test]
    fn selected_factor_sources_for_role_status_is_insufficient() {
        let sut = SUT::default();
        let status =
            sut.selected_factor_sources_for_role_status(RoleKind::Primary);

        pretty_assertions::assert_eq!(
            status,
            SelectedFactorSourcesForRoleStatus::Insufficient
        );
    }

    #[test]
    fn selected_factor_sources_for_role_status_is_invalid() {
        let sut = SUT::default();

        let _ = sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_password(),
        );
        let status =
            sut.selected_factor_sources_for_role_status(RoleKind::Primary);

        pretty_assertions::assert_eq!(
            status,
            SelectedFactorSourcesForRoleStatus::Invalid
        );
    }
}

#[cfg(test)]
mod lenient {
    use super::*;
    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityShieldBuilder;

    #[test]
    fn same_factor_in_primary_roles_both_lists() {
        let test = |sut: &SUT, expected_threshold_count: usize| {
            let x = FactorSourceID::sample_ledger();
            sut.add_factor_source_to_primary_override(x);

            sut.add_factor_source_to_primary_threshold(x);
            assert_eq!(
                sut.get_primary_threshold_factors().len(),
                expected_threshold_count
            );
        };
        let strict = SUT::strict();
        let lenient = SUT::lenient();

        test(&strict, 0);
        test(&lenient, 1);
    }

    #[test]
    fn primary_allows_two_device_factor_sources() {
        let test = |sut: &SUT, expected_threshold_count: usize| {
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device(),
            );
            sut.add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device_other(),
            );

            assert_eq!(
                sut.get_primary_threshold_factors().len(),
                expected_threshold_count
            );
        };
        let strict = SUT::strict();
        let lenient = SUT::lenient();

        test(&strict, 1);
        test(&lenient, 2);
    }

    #[test]
    fn same_factor_allowed_in_primary_and_then_in_recovery_and_confirmation() {
        let test = |sut: &SUT, expected_factors_in_non_primary: usize| {
            let x = FactorSourceID::sample_device();

            sut.add_factor_source_to_primary_override(x);

            sut.add_factor_source_to_recovery_override(x);
            sut.add_factor_source_to_confirmation_override(x);

            assert_eq!(
                sut.get_recovery_factors().len(),
                expected_factors_in_non_primary
            );

            assert_eq!(
                sut.get_confirmation_factors().len(),
                expected_factors_in_non_primary
            );
        };
        let strict = SUT::strict();
        let lenient = SUT::lenient();

        test(&lenient, 1);
        test(&strict, 1); // actually it was allowed for strict too...
    }

    #[test]
    fn same_factor_to_same_primary_list_not_allowed_even_for_lenient() {
        let test = |factor_list: FactorListKind| {
            let do_test = |sut: &SUT| {
                let x = FactorSourceID::sample_ledger();
                match factor_list {
                    FactorListKind::Override => {
                        sut.add_factor_source_to_primary_override(x);
                        sut.add_factor_source_to_primary_override(x);
                        assert_eq!(sut.get_primary_override_factors().len(), 1);
                    }
                    FactorListKind::Threshold => {
                        sut.add_factor_source_to_primary_threshold(x);
                        sut.add_factor_source_to_primary_threshold(x);
                        assert_eq!(
                            sut.get_primary_threshold_factors().len(),
                            1
                        );
                    }
                }
            };

            let strict = SUT::strict();
            let lenient = SUT::lenient();

            do_test(&strict);
            do_test(&lenient);
        };

        test(FactorListKind::Threshold);
        test(FactorListKind::Override);
    }
}
