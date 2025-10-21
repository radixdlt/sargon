use time_utils::now;

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

impl PartialEq for SecurityShieldBuilder {
    fn eq(&self, other: &Self) -> bool {
        let (matrix, name, authentication_signing_factor) = (
            self.matrix_builder
                .read()
                .expect("Failed to read matrix_builder"),
            self.name.read().expect("Failed to read name"),
            self.authentication_signing_factor
                .read()
                .expect("Failed to read authentication_signing_factor"),
        );
        let (other_matrix, other_name, other_authentication_signing_factor) = (
            other
                .matrix_builder
                .read()
                .expect("Failed to read other matrix_builder"),
            other.name.read().expect("Failed to read other name"),
            other
                .authentication_signing_factor
                .read()
                .expect("Failed to read other authentication_signing_factor"),
        );

        self.mode == other.mode
            && *matrix == *other_matrix
            && *name == *other_name
            && *authentication_signing_factor
                == *other_authentication_signing_factor
            && self.shield_id == other.shield_id
            && self.created_on == other.created_on
    }
}

impl Eq for SecurityShieldBuilder {}

impl Clone for SecurityShieldBuilder {
    fn clone(&self) -> Self {
        Self {
            mode: self.mode,
            matrix_builder: RwLock::new(
                self.matrix_builder
                    .read()
                    .expect("Failed to read matrix_builder")
                    .clone(),
            ),
            name: RwLock::new(
                self.name.read().expect("Failed to read name").clone(),
            ),
            authentication_signing_factor: RwLock::new(
                *self
                    .authentication_signing_factor
                    .read()
                    .expect("Failed to read authentication_signing_factor"),
            ),
            shield_id: self.shield_id,
            created_on: self.created_on,
        }
    }
}

impl std::hash::Hash for SecurityShieldBuilder {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let (matrix, name) = (
            self.matrix_builder
                .read()
                .expect("Failed to read matrix_builder"),
            self.name.read().expect("Failed to read name"),
        );
        matrix.hash(state);
        name.hash(state);
        self.shield_id.hash(state);
        self.created_on.hash(state);
    }
}

impl SecurityShieldBuilder {
    /// Maximum number of units (days, weeks, years) for the security structure recovery confirmation fallback period.
    pub const MAX_RECOVERY_CONFIRMATION_FALLBACK_PERIOD_UNITS: u16 = 9999;

    pub fn new(mode: SecurityShieldBuilderMode) -> Self {
        let matrix_builder = MatrixBuilder::new();
        let name = RwLock::new("My Shield".to_owned());
        Self {
            mode,
            matrix_builder: RwLock::new(matrix_builder),
            name,
            authentication_signing_factor: RwLock::new(None),
            shield_id: SecurityStructureID::from(Uuid::new_v4()),
            created_on: now(),
        }
    }

    pub fn with_details(
        mode: SecurityShieldBuilderMode,
        matrix_builder: RwLock<MatrixBuilder>,
        name: RwLock<String>,
        authentication_signing_factor: RwLock<Option<FactorSourceID>>,
        shield_id: SecurityStructureID,
        created_on: Timestamp,
    ) -> Self {
        Self {
            mode,
            matrix_builder,
            name,
            authentication_signing_factor,
            shield_id,
            created_on,
        }
    }

    pub fn with_security_structure_of_factor_sources(
        mode: SecurityShieldBuilderMode,
        security_structure_of_factor_sources: SecurityStructureOfFactorSources,
    ) -> Self {
        Self::with_security_structure_of_factor_source_ids(
            mode,
            security_structure_of_factor_sources.into(),
        )
    }

    pub fn with_security_structure_of_factor_source_ids(
        mode: SecurityShieldBuilderMode,
        security_structure_of_factor_source_ids: SecurityStructureOfFactorSourceIds,
    ) -> Self {
        Self::with_details(
            mode,
            RwLock::new(unsafe {
                MatrixBuilder::unbuilt_with_roles_and_days(
                    security_structure_of_factor_source_ids
                        .matrix_of_factors
                        .primary_role,
                    security_structure_of_factor_source_ids
                        .matrix_of_factors
                        .recovery_role,
                    security_structure_of_factor_source_ids
                        .matrix_of_factors
                        .confirmation_role,
                    security_structure_of_factor_source_ids
                        .matrix_of_factors
                        .time_until_delayed_confirmation_is_callable,
                )
            }),
            RwLock::new(
                security_structure_of_factor_source_ids
                    .metadata
                    .display_name
                    .value(),
            ),
            RwLock::new(Some(
                security_structure_of_factor_source_ids
                    .authentication_signing_factor,
            )),
            security_structure_of_factor_source_ids.metadata.id,
            security_structure_of_factor_source_ids.metadata.created_on,
        )
    }
}

impl HasSampleValues for SecurityShieldBuilder {
    fn sample() -> Self {
        let matrix_builder = MatrixBuilder::new();
        let name = RwLock::new("My Shield".to_owned());
        Self::with_details(
            SecurityShieldBuilderMode::Strict,
            RwLock::new(matrix_builder),
            name,
            RwLock::new(None),
            SecurityStructureID::sample(),
            Timestamp::sample(),
        )
    }

    fn sample_other() -> Self {
        let matrix_builder = MatrixBuilder::new();
        let name = RwLock::new("My Shield Other".to_owned());
        Self::with_details(
            SecurityShieldBuilderMode::Lenient,
            RwLock::new(matrix_builder),
            name,
            RwLock::new(None),
            SecurityStructureID::sample_other(),
            Timestamp::sample_other(),
        )
    }
}

impl SecurityShieldBuilder {
    pub fn sample_strict_with_auth_signing() -> Self {
        Self::with_details(
            SecurityShieldBuilderMode::Strict,
            RwLock::new(MatrixBuilder::new()),
            RwLock::new("My Shield".to_owned()),
            RwLock::new(Some(FactorSourceID::sample_ledger())),
            SecurityStructureID::from(Uuid::new_v4()),
            now(),
        )
    }

    pub fn sample_lenient_with_auth_signing() -> Self {
        Self::with_details(
            SecurityShieldBuilderMode::Lenient,
            RwLock::new(MatrixBuilder::new()),
            RwLock::new("My Shield".to_owned()),
            RwLock::new(Some(FactorSourceID::sample_ledger())),
            SecurityStructureID::from(Uuid::new_v4()),
            now(),
        )
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
    pub fn get_threshold(&self) -> Threshold {
        self.get(|builder| builder.get_threshold())
    }

    pub fn get_threshold_values(&self) -> Vec<Threshold> {
        self.get(|builder| {
            Threshold::values(
                builder.get_primary_threshold_factors().len() as u8
            )
        })
    }

    pub fn get_time_until_timed_confirmation_is_callable(&self) -> TimePeriod {
        self.get(|builder| {
            builder.get_time_until_delayed_confirmation_is_callable()
        })
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

    /// Removes factor **only** from the primary role based on the [FactorListKind].
    pub fn remove_factor_from_primary(
        &self,
        factor_source_id: FactorSourceID,
        factor_list_kind: FactorListKind,
    ) -> &Self {
        self.set(|builder| {
            builder
                .remove_factor_from_primary(&factor_source_id, factor_list_kind)
        })
    }

    /// Removes all factors from the override list of the primary role.
    pub fn remove_all_factors_from_primary_override(&self) -> &Self {
        self.set(|builder| builder.remove_all_factors_from_primary_override())
    }

    /// Removes factor **only** from the recovery role.
    pub fn remove_factor_from_recovery(
        &self,
        factor_source_id: FactorSourceID,
    ) -> &Self {
        self.set(|builder| {
            builder.remove_factor_from_recovery(
                &factor_source_id,
                FactorListKind::Override,
            )
        })
    }

    /// Removes factor **only** from the confirmation role.
    pub fn remove_factor_from_confirmation(
        &self,
        factor_source_id: FactorSourceID,
    ) -> &Self {
        self.set(|builder| {
            builder.remove_factor_from_confirmation(
                &factor_source_id,
                FactorListKind::Override,
            )
        })
    }

    pub fn set_threshold(&self, threshold: Threshold) -> &Self {
        self.set(|builder| builder.set_threshold(threshold))
    }

    pub fn set_time_until_delayed_confirmation_is_callable(
        &self,
        time_period: TimePeriod,
    ) -> &Self {
        self.set(|builder| {
            builder.set_time_until_delayed_confirmation_is_callable(
                time_period.days(),
            )
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
                factor_source_kind,
            )
        })
    }

    fn _validation_for_addition_of_factor_source_of_kind_to_recovery_override(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> RoleBuilderMutateResult {
        self.get(|builder| {
            builder.validation_for_addition_of_factor_source_of_kind_to_recovery_override(
                factor_source_kind,
            )
        })
    }

    fn _validation_for_addition_of_factor_source_of_kind_to_primary_override(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> RoleBuilderMutateResult {
        self.get(|builder| {
            builder.validation_for_addition_of_factor_source_of_kind_to_primary_override(
                factor_source_kind,
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
                    self.mode,
                )
            },
        )
    }
}

impl SecurityShieldBuilder {
    /// `None` means valid!
    pub fn validate(&self) -> Option<SecurityShieldBuilderRuleViolation> {
        if DisplayName::new(self.get_name()).is_err() {
            return Some(SecurityShieldBuilderRuleViolation::ShieldNameInvalid);
        }

        if self.get_authentication_signing_factor().is_none() {
            return Some(
                SecurityShieldBuilderRuleViolation::MissingAuthSigningFactor,
            );
        }

        if let Some(matrix_invalid_reason) = self.get(|builder| {
            let r = builder.validate();
            r.as_shield_validation()
        }) {
            return Some(matrix_invalid_reason);
        }

        None
    }

    /// Validates **just** the primary role **in isolation**.
    pub fn validate_primary_role(
        &self,
    ) -> Option<SecurityShieldBuilderRuleViolation> {
        self.validate_role_in_isolation(RoleKind::Primary)
    }

    /// `None` means valid!
    pub fn validate_role_in_isolation(
        &self,
        role: RoleKind,
    ) -> Option<SecurityShieldBuilderRuleViolation> {
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

    pub fn is_role_in_isolation_invalid(&self, role: RoleKind) -> bool {
        self.validate_role_in_isolation(role).is_some_and(|reason| {
            matches!(
                reason,
                SecurityShieldBuilderRuleViolation::MissingAuthSigningFactor |
                SecurityShieldBuilderRuleViolation::PrimaryRoleMustHaveAtLeastOneFactor |
                SecurityShieldBuilderRuleViolation::RecoveryRoleMustHaveAtLeastOneFactor |
                SecurityShieldBuilderRuleViolation::ConfirmationRoleMustHaveAtLeastOneFactor
            )
        })
    }

    pub fn status(&self) -> SecurityShieldBuilderStatus {
        let invalid_reason = SecurityShieldBuilderStatusInvalidReason::new(
            IsPrimaryRoleFactorListEmpty(
                self.is_role_in_isolation_invalid(RoleKind::Primary),
            ),
            IsRecoveryRoleFactorListEmpty(
                self.is_role_in_isolation_invalid(RoleKind::Recovery),
            ),
            IsConfirmationRoleFactorListEmpty(
                self.is_role_in_isolation_invalid(RoleKind::Confirmation),
            ),
            IsAuthSigningFactorMissing(
                self.get_authentication_signing_factor().is_none(),
            ),
        );

        if let Some(invalid_reason) = invalid_reason {
            SecurityShieldBuilderStatus::Invalid {
                reason: invalid_reason,
            }
        } else if let Some(rule_violation_reason) = self.validate() {
            SecurityShieldBuilderStatus::Weak {
                reason: rule_violation_reason,
            }
        } else {
            SecurityShieldBuilderStatus::Strong
        }
    }

    pub fn selected_primary_threshold_factors_status(
        &self,
    ) -> SelectedPrimaryThresholdFactorsStatus {
        let reason = self.get(|builder| {
            let validation =
                builder.validate_primary_threshold_factors_in_isolation();
            validation.as_shield_validation()
        });

        if let Some(reason) = reason {
            return match reason {
                SecurityShieldBuilderRuleViolation::PrimaryRoleMustHaveAtLeastOneFactor => {
                    SelectedPrimaryThresholdFactorsStatus::Insufficient
                }
                SecurityShieldBuilderRuleViolation::PrimaryRoleWithPasswordInThresholdListMustHaveAnotherFactor => {
                    SelectedPrimaryThresholdFactorsStatus::Invalid {
                        reason: SelectedPrimaryThresholdFactorsStatusInvalidReason::CannotBeUsedAlone {
                            factor_source_kind: FactorSourceKind::Password,
                        }
                    }
                },
                _ => {
                    SelectedPrimaryThresholdFactorsStatus::Invalid {
                        reason: SelectedPrimaryThresholdFactorsStatusInvalidReason::Other {
                            underlying: reason,
                        }
                    }
                }
            };
        }

        // Check conditions for Primary role
        let primary_factors_len =
            self.get(|builder| builder.get_primary_threshold_factors().len());

        if primary_factors_len < 2 {
            SelectedPrimaryThresholdFactorsStatus::Suboptimal
        } else {
            SelectedPrimaryThresholdFactorsStatus::Optimal
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
        SecurityShieldBuilderRuleViolation,
    > {
        let authentication_signing_factor =
            self.get_authentication_signing_factor().ok_or(
                SecurityShieldBuilderRuleViolation::MissingAuthSigningFactor,
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
        // The name can be empty when the builder is being used for a shield that is not saved as a template in profile
        // For example to update the shield that is already applied to an entity
        let display_name = if name.is_empty() {
            Ok(DisplayName::empty())
        } else {
            DisplayName::new(name).map_err(|e| {
                error!("Invalid DisplayName {:?}", e);
                SecurityShieldBuilderRuleViolation::ShieldNameInvalid
            })
        }?;

        let metadata = SecurityStructureMetadata::with_details(
            self.shield_id,
            display_name,
            self.created_on,
            self.created_on,
            SecurityStructureFlags::new(),
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
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    #[allow(clippy::mutable_key_type)]
    fn hash() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                SUT::sample(),
                SUT::sample(),
                SUT::sample_other(),
                SUT::sample_other(),
            ])
            .len(),
            2
        )
    }

    #[test]
    fn clone() {
        assert_eq!(SUT::sample(), SUT::sample().clone());
        assert_eq!(SUT::sample_other(), SUT::sample_other().clone());
    }

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
        sut.set_threshold(Threshold::Specific(42));
        sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_device(),
        );
        assert_eq!(sut.get_threshold(), Threshold::Specific(42));
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
    fn test_get_time_until_timed_confirmation_is_callable() {
        let sut = SUT::strict();
        assert_eq!(
            sut.get_time_until_timed_confirmation_is_callable(),
            TimePeriod::with_days(14)
        );
        sut.set_time_until_delayed_confirmation_is_callable(
            TimePeriod::with_days(42),
        );
        assert_eq!(
            sut.get_time_until_timed_confirmation_is_callable(),
            TimePeriod::with_days(42)
        );
    }

    #[test]
    fn test_threshold_values() {
        let sut = SUT::strict();
        sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_device(),
        );

        assert_eq!(sut.get_threshold_values(), vec![Threshold::All]);

        sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_ledger(),
        );

        assert_eq!(
            sut.get_threshold_values(),
            vec![Threshold::All, Threshold::Specific(1)]
        );
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
            .set_time_until_delayed_confirmation_is_callable(
                TimePeriod::with_days(42),
            )
            .add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device(),
            )
            .add_factor_source_to_primary_threshold(
                FactorSourceID::sample_password(),
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
            .add_factor_source_to_confirmation_override(
                FactorSourceID::sample_device_other(),
            )
            .remove_factor_from_primary(
                FactorSourceID::sample_password(),
                FactorListKind::Threshold,
            )
            .remove_factor_from_primary(
                FactorSourceID::sample_arculus_other(),
                FactorListKind::Override,
            )
            .remove_factor_from_recovery(FactorSourceID::sample_ledger_other())
            .remove_factor_from_confirmation(
                FactorSourceID::sample_device_other(),
            );

        let shield0 = sut.build().unwrap();
        let shield = sut.build().unwrap();
        pretty_assertions::assert_eq!(shield0, shield);

        assert_eq!(shield.metadata.display_name.value(), "S.H.I.E.L.D.");
        assert_eq!(
            shield.matrix_of_factors.primary().get_override_factors(),
            &vec![FactorSourceID::sample_arculus()]
        );
        assert_eq!(shield.matrix_of_factors.primary().get_threshold_value(), 1);
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

            // now lets adding a Device => subsequent calls to `is_fully_valid` will return true
            add(sut, FactorSourceID::sample_device());
            add(sut, FactorSourceID::sample_ledger());

            sut.set_threshold(Threshold::Specific(2));
            assert!(is_fully_valid(sut, FactorSourceKind::Password)); // not alone any more!
            assert!(can_be(sut, FactorSourceKind::Password));
        } else {
            // now lets adding a Device => subsequent calls to `is_fully_valid` will return true
            add(sut, FactorSourceID::sample_device());
        }

        assert!(is_fully_valid(sut, FactorSourceKind::Device));

        // Another device can be added
        assert!(can_be(sut, FactorSourceKind::Device));
        assert!(is_fully_valid(sut, FactorSourceKind::Device));
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
                FactorSourceInRoleBuilderValidationStatus::ok(
                    RoleKind::Primary,
                    FactorSourceID::sample_device_other(),
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
    fn selected_primary_threshold_factors_status_invalid() {
        let sut = SUT::default();

        let _ = sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_password(),
        );

        pretty_assertions::assert_eq!(
            sut.selected_primary_threshold_factors_status(),
            SelectedPrimaryThresholdFactorsStatus::Invalid {
                reason: SelectedPrimaryThresholdFactorsStatusInvalidReason::CannotBeUsedAlone {
                    factor_source_kind: FactorSourceKind::Password,
                }
            }
        );

        sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_device(),
        )
        .add_factor_source_to_primary_threshold(
            FactorSourceID::sample_device_other(),
        );

        pretty_assertions::assert_eq!(
            sut.selected_primary_threshold_factors_status(),
            SelectedPrimaryThresholdFactorsStatus::Invalid {
                reason: SelectedPrimaryThresholdFactorsStatusInvalidReason::Other {
                    underlying: SecurityShieldBuilderRuleViolation::PrimaryCannotHaveMultipleDevices
                }
            }
        );
    }

    #[test]
    fn selected_primary_threshold_factors_status() {
        let sut = SUT::sample_lenient_with_auth_signing();

        pretty_assertions::assert_eq!(
            sut.selected_primary_threshold_factors_status(),
            SelectedPrimaryThresholdFactorsStatus::Insufficient
        );

        let _ = sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_device(),
        );

        pretty_assertions::assert_eq!(
            sut.selected_primary_threshold_factors_status(),
            SelectedPrimaryThresholdFactorsStatus::Suboptimal
        );

        sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_password(),
        )
        .add_factor_source_to_primary_override(FactorSourceID::sample_device());

        pretty_assertions::assert_eq!(
            sut.selected_primary_threshold_factors_status(),
            SelectedPrimaryThresholdFactorsStatus::Optimal
        );
    }

    #[test]
    fn remove_all_factors_from_primary_override() {
        let sut = SUT::default();

        let _ = sut
            .add_factor_source_to_primary_override(
                FactorSourceID::sample_device(),
            )
            .add_factor_source_to_primary_override(
                FactorSourceID::sample_device_other(),
            );

        pretty_assertions::assert_eq!(
            sut.get_primary_override_factors(),
            vec![
                FactorSourceID::sample_device(),
                FactorSourceID::sample_device_other(),
            ]
        );

        sut.remove_all_factors_from_primary_override();

        assert!(sut.get_primary_override_factors().is_empty());
    }
}

#[cfg(test)]
mod test_invalid {
    use super::*;
    use crate::prelude::SelectedPrimaryThresholdFactorsStatusInvalidReason::Other;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityShieldBuilder;

    #[test]
    fn must_have_auth_signing_factor() {
        let sut = SUT::strict();
        assert_eq!(
            sut.validate().unwrap(),
            SecurityShieldBuilderRuleViolation::MissingAuthSigningFactor
        );
    }

    #[test]
    fn missing_auth_signing_takes_precedence_over_roles_validation() {
        let sut = SUT::strict();

        sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_device(),
        )
        .add_factor_source_to_recovery_override(FactorSourceID::sample_device())
        // This addition results in SingleFactorUsedInPrimaryMustNotBeUsedInAnyOtherRole matrix invalid reason
        .add_factor_source_to_confirmation_override(
            FactorSourceID::sample_device(),
        );

        assert_eq!(
            sut.validate().unwrap(),
            SecurityShieldBuilderRuleViolation::MissingAuthSigningFactor
        );
    }

    #[test]
    fn primary_role_must_have_at_least_one_factor() {
        let sut = SUT::sample_strict_with_auth_signing();
        assert_eq!(
            sut.validate().unwrap(),
            SecurityShieldBuilderRuleViolation::PrimaryRoleMustHaveAtLeastOneFactor
        );
        assert_eq!(
            sut.validate_role_in_isolation(RoleKind::Primary).unwrap(),
            SecurityShieldBuilderRuleViolation::PrimaryRoleMustHaveAtLeastOneFactor
        );
    }

    #[test]
    fn primary_role_with_threshold_cannot_be_zero_with_factors() {
        let sut = SUT::sample_strict_with_auth_signing();
        sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_device(),
        );
        assert_eq!(sut.get_threshold(), Threshold::All);
        sut.set_threshold(Threshold::zero());
        assert_eq!(
            sut.validate().unwrap(),
            SecurityShieldBuilderRuleViolation::PrimaryRoleWithThresholdFactorsCannotHaveAThresholdValueOfZero
        );
        assert_eq!(
            sut.validate_role_in_isolation(RoleKind::Primary).unwrap(),
            SecurityShieldBuilderRuleViolation::PrimaryRoleWithThresholdFactorsCannotHaveAThresholdValueOfZero
        );
    }

    #[test]
    fn recovery_role_must_have_at_least_one_factor() {
        let sut = SUT::sample_strict_with_auth_signing();
        sut.add_factor_source_to_primary_override(
            FactorSourceID::sample_device(),
        );
        assert_eq!(
            sut.validate().unwrap(),
            SecurityShieldBuilderRuleViolation::RecoveryRoleMustHaveAtLeastOneFactor
        );
        assert_eq!(
            sut.validate_role_in_isolation(RoleKind::Recovery).unwrap(),
            SecurityShieldBuilderRuleViolation::RecoveryRoleMustHaveAtLeastOneFactor
        );
    }

    #[test]
    fn confirmation_role_must_have_at_least_one_factor() {
        let sut = SUT::sample_strict_with_auth_signing();
        sut.add_factor_source_to_primary_override(
            FactorSourceID::sample_device(),
        );
        sut.add_factor_source_to_recovery_override(
            FactorSourceID::sample_ledger(),
        );
        assert_eq!(
            sut.validate().unwrap(),
            SecurityShieldBuilderRuleViolation::ConfirmationRoleMustHaveAtLeastOneFactor
        );
        assert_eq!(
            sut.validate_role_in_isolation(RoleKind::Confirmation).unwrap(),
            SecurityShieldBuilderRuleViolation::ConfirmationRoleMustHaveAtLeastOneFactor
        );
    }

    #[test]
    fn shield_must_have_authentication_signing_factor() {
        let sut = SUT::strict();
        sut.add_factor_source_to_primary_override(
            FactorSourceID::sample_device(),
        );
        sut.add_factor_source_to_recovery_override(
            FactorSourceID::sample_ledger(),
        );
        sut.add_factor_source_to_confirmation_override(
            FactorSourceID::sample_ledger_other(),
        );
        assert_eq!(
            sut.validate().unwrap(),
            SecurityShieldBuilderRuleViolation::MissingAuthSigningFactor
        );
        assert!(sut.validate_role_in_isolation(RoleKind::Primary).is_none());
        sut.set_authentication_signing_factor(Some(
            FactorSourceID::sample_device(),
        ));
        assert!(sut.validate().is_none());
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
            SecurityShieldBuilderRuleViolation::ShieldNameInvalid
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
            shield.metadata.display_name.value(),
            "This shield name's too long an"
        );
    }

    #[test]
    fn time_until_delayed_confirmation_is_callable_invalid() {
        let sut = valid();
        sut.set_time_until_delayed_confirmation_is_callable(
            TimePeriod::with_days(0),
        );
        assert_eq!(
            sut.validate().unwrap(),
            SecurityShieldBuilderRuleViolation::NumberOfDaysUntilTimeBasedConfirmationMustBeGreaterThanZero
        );
    }

    #[test]
    fn recovery_and_confirmation_factors_overlap() {
        let sut = SUT::sample_strict_with_auth_signing();
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
            SecurityShieldBuilderRuleViolation::RecoveryAndConfirmationFactorsOverlap
        );
    }

    #[test]
    fn factor_source_already_present() {
        let sut = SUT::sample_strict_with_auth_signing();
        sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_device(),
        )
        .add_factor_source_to_recovery_override(FactorSourceID::sample_ledger())
        .add_factor_source_to_confirmation_override(
            FactorSourceID::sample_ledger_other(),
        );

        let res = sut.validation_for_addition_of_factor_source_to_primary_threshold_for_each(
            vec![FactorSourceID::sample_device(),]
        );

        sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_device(),
        ); // did not get added

        assert_eq!(
            res,
            vec![FactorSourceInRoleBuilderValidationStatus::forever_invalid(
                RoleKind::Primary,
                FactorSourceID::sample_device(),
                ForeverInvalidReason::FactorSourceAlreadyPresent
            )]
        );

        assert!(sut.validate().is_none(),);
        assert_eq!(sut.status(), SecurityShieldBuilderStatus::Strong);
    }

    #[test]
    fn primary_cannot_have_password_in_override_list() {
        let sut = SUT::sample_strict_with_auth_signing();
        sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_device(),
        )
        .add_factor_source_to_recovery_override(FactorSourceID::sample_ledger())
        .add_factor_source_to_confirmation_override(
            FactorSourceID::sample_ledger_other(),
        );

        let is_valid_or_can_be = sut.addition_of_factor_source_of_kind_to_primary_override_is_valid_or_can_be(FactorSourceKind::Password);
        let validation_res = sut.validation_for_addition_of_factor_source_to_primary_override_for_each(
            vec![FactorSourceID::sample_password(),]
        );

        sut.add_factor_source_to_primary_override(
            FactorSourceID::sample_password(),
        ); // did not get added

        assert!(!is_valid_or_can_be);
        assert_eq!(
            validation_res,
            vec![FactorSourceInRoleBuilderValidationStatus::forever_invalid(
                RoleKind::Primary,
                FactorSourceID::sample_password(),
                ForeverInvalidReason::PrimaryCannotHavePasswordInOverrideList
            )]
        );

        assert!(sut.validate().is_none(),);
        assert_eq!(sut.status(), SecurityShieldBuilderStatus::Strong);
    }

    #[test]
    fn single_factor_used_in_primary_must_not_be_used_in_any_other_role_in_recovery(
    ) {
        let sut = SUT::sample_strict_with_auth_signing();
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
            SecurityShieldBuilderRuleViolation::SingleFactorUsedInPrimaryMustNotBeUsedInAnyOtherRole
        );
    }

    #[test]
    fn single_factor_used_in_primary_must_not_be_used_in_any_other_role_in_confirmation(
    ) {
        let sut = SUT::sample_strict_with_auth_signing();
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
            SecurityShieldBuilderRuleViolation::SingleFactorUsedInPrimaryMustNotBeUsedInAnyOtherRole
        );
    }

    #[test]
    fn primary_role_with_password_in_threshold_list_must_threshold_greater_than_one(
    ) {
        let sut = SUT::sample_strict_with_auth_signing();

        sut.add_factor_source_to_recovery_override(
            FactorSourceID::sample_ledger(),
        );
        sut.add_factor_source_to_confirmation_override(
            FactorSourceID::sample_arculus(),
        );

        sut.set_threshold(Threshold::Specific(1));
        sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_password(),
        );
        sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_device(),
        );

        assert_eq!(
            sut.validate().unwrap(),
            SecurityShieldBuilderRuleViolation::PrimaryRoleWithPasswordInThresholdListMustThresholdGreaterThanOne
        );
        assert_eq!(
            sut.validate_role_in_isolation(RoleKind::Primary).unwrap(),
            SecurityShieldBuilderRuleViolation::PrimaryRoleWithPasswordInThresholdListMustThresholdGreaterThanOne
        );
    }

    #[test]
    fn primary_role_with_password_in_threshold_list_must_have_another_factor() {
        let sut = SUT::sample_strict_with_auth_signing();

        sut.add_factor_source_to_recovery_override(
            FactorSourceID::sample_ledger(),
        );
        sut.add_factor_source_to_confirmation_override(
            FactorSourceID::sample_arculus(),
        );

        sut.set_threshold(Threshold::Specific(1));
        sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_password(),
        );

        assert_eq!(
            sut.validate().unwrap(),
            SecurityShieldBuilderRuleViolation::PrimaryRoleWithPasswordInThresholdListMustHaveAnotherFactor
        );
        assert_eq!(
            sut.validate_role_in_isolation(RoleKind::Primary).unwrap(),
            SecurityShieldBuilderRuleViolation::PrimaryRoleWithPasswordInThresholdListMustHaveAnotherFactor
        );
    }

    #[test]
    fn two_different_password_only_not_valid_for_primary() {
        let sut = SUT::sample_strict_with_auth_signing();

        sut.add_factor_source_to_recovery_override(
            FactorSourceID::sample_ledger(),
        );
        sut.add_factor_source_to_confirmation_override(
            FactorSourceID::sample_arculus(),
        );

        sut.set_threshold(Threshold::Specific(2));
        sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_password(),
        );
        sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_password_other(),
        );

        assert_eq!(
            sut.validate().unwrap(),
            SecurityShieldBuilderRuleViolation::PrimaryRoleWithPasswordInThresholdListMustHaveAnotherFactor
        );
    }

    #[test]
    fn primary_role_with_password_in_override_does_not_get_added() {
        let sut = SUT::sample_strict_with_auth_signing();

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
            SecurityShieldBuilderRuleViolation::PrimaryRoleMustHaveAtLeastOneFactor
        );
        assert_eq!(
            sut.validate_role_in_isolation(RoleKind::Primary).unwrap(),
            SecurityShieldBuilderRuleViolation::PrimaryRoleMustHaveAtLeastOneFactor
        );
    }

    #[test]
    fn template() {
        // use this to create more tests...
        let sut = valid();
        sut.set_name("");
        assert_eq!(
            sut.validate().unwrap(),
            SecurityShieldBuilderRuleViolation::ShieldNameInvalid
        );
    }

    #[test]
    fn selected_primary_threshold_factors_status_is_insufficient() {
        let sut = SUT::default();
        let status = sut.selected_primary_threshold_factors_status();

        pretty_assertions::assert_eq!(
            status,
            SelectedPrimaryThresholdFactorsStatus::Insufficient
        );
    }

    #[test]
    fn selected_primary_threshold_factors_status_is_invalid_when_only_password_is_selected(
    ) {
        let sut = SUT::default();

        let _ = sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_password(),
        );
        let status = sut.selected_primary_threshold_factors_status();

        pretty_assertions::assert_eq!(
            status,
            SelectedPrimaryThresholdFactorsStatus::Invalid {
                reason: SelectedPrimaryThresholdFactorsStatusInvalidReason::CannotBeUsedAlone {
                    factor_source_kind: FactorSourceKind::Password
                }
            }
        );
    }

    #[test]
    fn selected_primary_threshold_factors_status_is_invalid() {
        let sut = SUT::default();

        let _ = sut
            .add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device(),
            )
            .add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device_other(),
            );
        let _ = sut.set_threshold(Threshold::zero());
        let status = sut.selected_primary_threshold_factors_status();

        pretty_assertions::assert_eq!(
            status,
            SelectedPrimaryThresholdFactorsStatus::Invalid {
                reason: Other {
                    underlying: SecurityShieldBuilderRuleViolation::PrimaryCannotHaveMultipleDevices
                }
            }
        );
    }

    #[test]
    fn shield_status_strong() {
        let sut = SUT::default();

        let _ = sut
            .add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device(),
            )
            .add_factor_source_to_primary_threshold(
                FactorSourceID::sample_password(),
            )
            .set_authentication_signing_factor(Some(
                FactorSourceID::sample_device(),
            ))
            .add_factor_source_to_recovery_override(
                FactorSourceID::sample_ledger(),
            )
            .add_factor_source_to_confirmation_override(
                FactorSourceID::sample_arculus(),
            );

        let status = sut.status();

        pretty_assertions::assert_eq!(
            status,
            SecurityShieldBuilderStatus::Strong
        );
        assert!(sut.build().is_ok())
    }

    #[test]
    fn shield_status_weak() {
        let sut = SUT::default();

        let _ = sut
            .add_factor_source_to_primary_threshold(
                FactorSourceID::sample_password(),
            )
            .set_authentication_signing_factor(Some(
                FactorSourceID::sample_device(),
            ))
            .add_factor_source_to_recovery_override(
                FactorSourceID::sample_ledger(),
            )
            .add_factor_source_to_confirmation_override(
                FactorSourceID::sample_arculus(),
            );

        pretty_assertions::assert_eq!(
            sut.status(),
            SecurityShieldBuilderStatus::Weak {
                reason: SecurityShieldBuilderRuleViolation::PrimaryRoleWithPasswordInThresholdListMustHaveAnotherFactor
            }
        );

        let _ = sut
            .add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device(),
            )
            .remove_factor_from_recovery(FactorSourceID::sample_ledger())
            .add_factor_source_to_recovery_override(
                FactorSourceID::sample_arculus(),
            );

        pretty_assertions::assert_eq!(
            sut.status(),
            SecurityShieldBuilderStatus::Weak {
                reason: SecurityShieldBuilderRuleViolation::RecoveryAndConfirmationFactorsOverlap
            }
        );

        let _ = sut
            .remove_factor_from_recovery(FactorSourceID::sample_password())
            .add_factor_source_to_recovery_override(
                FactorSourceID::sample_ledger_other(),
            )
            .add_factor_source_to_confirmation_override(
                FactorSourceID::sample_ledger_other(),
            );

        pretty_assertions::assert_eq!(
            sut.status(),
            SecurityShieldBuilderStatus::Weak {
                reason: SecurityShieldBuilderRuleViolation::RecoveryAndConfirmationFactorsOverlap
            }
        );
        assert!(sut.build().is_ok())
    }

    #[test]
    fn shield_status_invalid() {
        let sut = SUT::default();

        pretty_assertions::assert_eq!(
            sut.status(),
            SecurityShieldBuilderStatus::Invalid {
                reason: SecurityShieldBuilderStatusInvalidReason::new(
                    IsPrimaryRoleFactorListEmpty(true),
                    IsRecoveryRoleFactorListEmpty(true),
                    IsConfirmationRoleFactorListEmpty(true),
                    IsAuthSigningFactorMissing(true),
                )
                .unwrap()
            }
        );

        let _ = sut.add_factor_source_to_primary_threshold(
            FactorSourceID::sample_device(),
        );

        pretty_assertions::assert_eq!(
            sut.status(),
            SecurityShieldBuilderStatus::Invalid {
                reason: SecurityShieldBuilderStatusInvalidReason::new(
                    IsPrimaryRoleFactorListEmpty(false),
                    IsRecoveryRoleFactorListEmpty(true),
                    IsConfirmationRoleFactorListEmpty(true),
                    IsAuthSigningFactorMissing(true),
                )
                .unwrap()
            }
        );

        let _ = sut.set_authentication_signing_factor(Some(
            FactorSourceID::sample_device(),
        ));

        pretty_assertions::assert_eq!(
            sut.status(),
            SecurityShieldBuilderStatus::Invalid {
                reason: SecurityShieldBuilderStatusInvalidReason::new(
                    IsPrimaryRoleFactorListEmpty(false),
                    IsRecoveryRoleFactorListEmpty(true),
                    IsConfirmationRoleFactorListEmpty(true),
                    IsAuthSigningFactorMissing(false),
                )
                .unwrap()
            }
        );

        let _ = sut.add_factor_source_to_recovery_override(
            FactorSourceID::sample_ledger(),
        );

        pretty_assertions::assert_eq!(
            sut.status(),
            SecurityShieldBuilderStatus::Invalid {
                reason: SecurityShieldBuilderStatusInvalidReason::new(
                    IsPrimaryRoleFactorListEmpty(false),
                    IsRecoveryRoleFactorListEmpty(false),
                    IsConfirmationRoleFactorListEmpty(true),
                    IsAuthSigningFactorMissing(false),
                )
                .unwrap()
            }
        );
        assert!(sut.build().is_err())
    }

    #[test]
    fn shield_status_weak_when_multiple_devices_in_primary() {
        let sut = SUT::default();

        let _ = sut
            .set_authentication_signing_factor(Some(
                FactorSourceID::sample_device(),
            ))
            .add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device(),
            )
            .add_factor_source_to_recovery_override(
                FactorSourceID::sample_device(),
            )
            .add_factor_source_to_confirmation_override(
                FactorSourceID::sample_arculus(),
            );

        assert!(
            sut.addition_of_factor_source_of_kind_to_primary_threshold_is_valid_or_can_be(
                FactorSourceKind::Device
            )
        );

        let _ = sut
            .add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device_other(),
            ) // did not get added
            .add_factor_source_to_primary_override(
                FactorSourceID::sample_device(),
            );

        pretty_assertions::assert_eq!(
            sut.validate(),
            Some(SecurityShieldBuilderRuleViolation::PrimaryCannotHaveMultipleDevices)
        );
        pretty_assertions::assert_eq!(
            sut.status(),
            SecurityShieldBuilderStatus::Weak {
                reason: SecurityShieldBuilderRuleViolation::PrimaryCannotHaveMultipleDevices
            }
        );
        assert!(sut.build().is_ok())
    }

    #[test]
    fn shield_status_weak_when_same_factor_in_primary() {
        let sut = SUT::default();

        let _ = sut
            .set_authentication_signing_factor(Some(
                FactorSourceID::sample_device(),
            ))
            .add_factor_source_to_primary_threshold(
                FactorSourceID::sample_ledger(),
            )
            .add_factor_source_to_recovery_override(
                FactorSourceID::sample_device(),
            )
            .add_factor_source_to_confirmation_override(
                FactorSourceID::sample_arculus(),
            );

        assert!(
            sut.addition_of_factor_source_of_kind_to_primary_threshold_is_valid_or_can_be(
                FactorSourceKind::LedgerHQHardwareWallet
            )
        );

        let _ = sut
            .add_factor_source_to_primary_threshold(
                FactorSourceID::sample_ledger(),
            ) // did not get added
            .add_factor_source_to_primary_override(
                FactorSourceID::sample_ledger(),
            );

        pretty_assertions::assert_eq!(
            sut.validate(),
            Some(
                SecurityShieldBuilderRuleViolation::FactorSourceAlreadyPresent
            )
        );
        pretty_assertions::assert_eq!(
            sut.status(),
            SecurityShieldBuilderStatus::Weak {
                reason: SecurityShieldBuilderRuleViolation::FactorSourceAlreadyPresent
            }
        );
        assert!(sut.build().is_ok())
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

        test(&strict, 1);
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

        test(&strict, 2);
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

    #[test]
    fn basic_validation() {
        let sut = SUT::lenient();

        let _ = sut
            .set_authentication_signing_factor(Some(
                FactorSourceID::sample_device(),
            ))
            .add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device(),
            )
            .add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device(),
            ); // did not get added, duplicates are not allowed

        pretty_assertions::assert_eq!(
            sut.get_primary_threshold_factors(),
            vec![FactorSourceID::sample_device()]
        );

        pretty_assertions::assert_eq!(
            sut._validation_for_addition_of_factor_source_of_kind_to_primary_threshold(
                FactorSourceKind::Device,
            ),
            Ok(())
        );

        let _ = sut
            .add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device_other(),
            )
            .add_factor_source_to_primary_override(
                FactorSourceID::sample_device_other(),
            ); // actually this is added because of the lenient mode

        pretty_assertions::assert_eq!(
            sut.get_primary_threshold_factors(),
            vec![
                FactorSourceID::sample_device(),
                FactorSourceID::sample_device_other()
            ]
        );

        pretty_assertions::assert_eq!(
            sut.get_primary_override_factors(),
            vec![FactorSourceID::sample_device_other()]
        );

        let _ = sut
            .add_factor_source_to_recovery_override(
                FactorSourceID::sample_ledger(),
            )
            .add_factor_source_to_confirmation_override(
                FactorSourceID::sample_arculus(),
            );

        pretty_assertions::assert_eq!(
            sut.status(),
            SecurityShieldBuilderStatus::Weak {
                reason: SecurityShieldBuilderRuleViolation::PrimaryCannotHaveMultipleDevices
            }
        );
        assert!(sut.build().is_ok());
    }

    #[test]
    fn primary_password_never_alone() {
        let sut = SUT::lenient();

        let _ = sut
            .set_authentication_signing_factor(FactorSourceID::sample_device())
            .add_factor_source_to_primary_override(
                FactorSourceID::sample_password(),
            ); // not allowed

        assert!(sut.get_primary_override_factors().is_empty(),);

        let _ = sut
            .add_factor_source_to_recovery_override(
                FactorSourceID::sample_arculus(),
            )
            .add_factor_source_to_confirmation_override(
                FactorSourceID::sample_arculus_other(),
            )
            .add_factor_source_to_primary_threshold(
                FactorSourceID::sample_password(),
            );

        assert_eq!(
            sut.status(),
            SecurityShieldBuilderStatus::Weak {
                reason: SecurityShieldBuilderRuleViolation::PrimaryRoleWithPasswordInThresholdListMustHaveAnotherFactor
            }
        );

        let _ = sut
            .add_factor_source_to_primary_threshold(
                FactorSourceID::sample_ledger(),
            )
            .set_threshold(Threshold::Specific(1));

        assert_eq!(
            sut.status(),
            SecurityShieldBuilderStatus::Weak {
                reason: SecurityShieldBuilderRuleViolation::PrimaryRoleWithPasswordInThresholdListMustThresholdGreaterThanOne
            }
        );

        let _ = sut.set_threshold(Threshold::Specific(2));

        assert_eq!(sut.status(), SecurityShieldBuilderStatus::Strong);

        let shield = sut.build().unwrap();
        assert!(shield
            .matrix_of_factors
            .primary_role
            .get_override_factors()
            .is_empty());
        assert_eq!(
            shield.matrix_of_factors.primary_role.get_threshold(),
            Threshold::Specific(2)
        );
        assert_eq!(
            shield
                .matrix_of_factors
                .primary_role
                .get_threshold_factors()
                .clone(),
            vec![
                FactorSourceID::sample_password(),
                FactorSourceID::sample_ledger()
            ]
        );
    }

    #[test]
    fn validation_for_addition() {
        let sut = SUT::lenient();

        assert_eq!(
            sut._validation_for_addition_of_factor_source_of_kind_to_primary_override(
                FactorSourceKind::Password,
            ),
            Err(
                RoleBuilderValidation::ForeverInvalid(
                    ForeverInvalidReason::PrimaryCannotHavePasswordInOverrideList
                )
            )
        );
    }

    #[test]
    fn with_security_structure_of_factor_source_ids_roundtrip() {
        let shield = SUT::default();
        let _ = shield
            .set_name("Shield")
            .set_authentication_signing_factor(Some(
                FactorSourceID::sample_device(),
            ))
            .set_time_until_delayed_confirmation_is_callable(
                TimePeriod::with_days(42),
            )
            // Primary
            .add_factor_source_to_primary_threshold(
                FactorSourceID::sample_device(),
            )
            .add_factor_source_to_primary_override(
                FactorSourceID::sample_arculus(),
            )
            // Recovery
            .add_factor_source_to_recovery_override(
                FactorSourceID::sample_ledger(),
            )
            // Confirmation
            .add_factor_source_to_confirmation_override(
                FactorSourceID::sample_arculus_other(),
            );

        let original = shield.build().unwrap();

        // Reconstruct the builder from the *existing* security structure
        let reconstructed = SUT::with_security_structure_of_factor_source_ids(
            SecurityShieldBuilderMode::Strict,
            original.clone(),
        );

        // Mode is what we asked for when reconstructing
        assert_eq!(reconstructed.mode, SecurityShieldBuilderMode::Strict);

        // The getters reflect the same state carried over from `original`
        assert_eq!(reconstructed.get_name(), "Shield");
        assert_eq!(
            reconstructed.get_authentication_signing_factor(),
            Some(FactorSourceID::sample_device())
        );
        assert_eq!(
            reconstructed.get_time_until_timed_confirmation_is_callable(),
            TimePeriod::with_days(42)
        );
        assert_eq!(
            reconstructed.get_primary_threshold_factors(),
            vec![FactorSourceID::sample_device()]
        );
        assert_eq!(
            reconstructed.get_primary_override_factors(),
            vec![FactorSourceID::sample_arculus()]
        );
        assert_eq!(
            reconstructed.get_recovery_factors(),
            vec![FactorSourceID::sample_ledger()]
        );
        assert_eq!(
            reconstructed.get_confirmation_factors(),
            vec![FactorSourceID::sample_arculus_other()]
        );

        // Building again yields *the very same* SecurityStructureOfFactorSourceIds,
        let rebuilt = reconstructed.build().unwrap();
        assert_eq!(rebuilt, original);
    }

    #[test]
    fn with_security_structure_of_factor_sources_roundtrip() {
        let mut original = SecurityStructureOfFactorSources::sample_other();
        original.metadata.update_name(DisplayName::empty());
        pretty_assertions::assert_eq!(
            original.metadata.display_name,
            DisplayName::empty()
        );

        // Reconstruct the builder from the *existing* security structure
        let reconstructed = SUT::with_security_structure_of_factor_sources(
            SecurityShieldBuilderMode::Strict,
            original.clone(),
        );

        // Building again yields *the very same* SecurityStructureOfFactorSourceIds,
        let rebuilt = reconstructed.build().unwrap();
        pretty_assertions::assert_eq!(rebuilt, original.into());
    }
}
