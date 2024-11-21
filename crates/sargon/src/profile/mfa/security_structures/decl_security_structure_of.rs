use crate::prelude::*;

macro_rules! decl_role_with_factors_additional_impl {
    (
        $role: ident,
        FactorInstance
    ) => {
        paste! {
            impl From<[< $role RoleWithFactorInstance s >]> for ScryptoAccessRule {
                fn from(value: [< $role RoleWithFactorInstance s >]) -> Self {
                    ScryptoAccessRule::Protected(ScryptoCompositeRequirement::AnyOf(vec![
                        ScryptoCompositeRequirement::BasicRequirement(ScryptoBasicRequirement::CountOf(
                            value.threshold,
                            value
                                .threshold_factors
                                .into_iter()
                                .map(|instance| instance.badge)
                                .map(ScryptoResourceOrNonFungible::from)
                                .collect(),
                        )),
                        ScryptoCompositeRequirement::BasicRequirement(ScryptoBasicRequirement::AnyOf(
                            value
                                .override_factors
                                .into_iter()
                                .map(|instance| instance.badge)
                                .map(ScryptoResourceOrNonFungible::from)
                                .collect(),
                        )),
                    ]))
                }
            }
        }
    };
    (
        $role: ident,
        $factor: ident
    ) => {}
}
pub(crate) use decl_role_with_factors_additional_impl;

macro_rules! decl_role_with_factors_with_role_kind_attrs {
    (
        $(
            #[doc = $expr: expr]
        )*
        $role: ident,
        $factor: ident,
        $($extra_field_name:ident: $extra_field_type:ty,)*
    ) => {
        paste! {
            $(
                #[doc = $expr]
            )*
            #[derive(
                Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash,
            )]
            #[serde(rename_all = "camelCase")]
            pub struct [< $role RoleWith $factor s >] {

                /// Factors which are used in combination with other instances, amounting to at
                /// least `threshold` many instances to perform some function with this role.
                ///
                /// # Implementation
                /// Must allow duplicates, thus using `Vec` since at FactorSourceKind level
                /// we might wanna use duplicates, allowing us to build a "template"
                /// structure where a role might contain two `FactorSourceKind::TrustedContact`,
                /// meaning an instance of this template at FactorSource level
                /// (`SecurityStructureOfFactorSources`) will contain two different
                /// `TrustedContactFactorSource`s.
                pub threshold_factors: Vec<$factor>,

                /// How many threshold factors that must be used to perform some function with this role.
                pub threshold: u8,

                /// Overriding / Super admin / "sudo" / God / factors, **ANY**
                /// single of these factor which can perform the function of this role,
                /// disregarding of `threshold`.
                pub override_factors: Vec<$factor>,

                $(pub $extra_field_name: $extra_field_type,)*
            }


            impl RoleWithFactors<$factor> for [< $role RoleWith $factor s >] {
                fn get_threshold_factors(&self) -> &Vec<$factor> {
                    &self.threshold_factors
                }

                fn get_threshold(&self) -> u8 {
                    self.threshold
                }

                fn get_override_factors(&self) -> &Vec<$factor> {
                    &self.override_factors
                }
            }


            impl [< $role RoleWith $factor s >] {
                fn with_factor_list_of_kind<T>(&self, list_kind: impl Into<Option<FactorListKind>>, access: impl Fn(Vec<&$factor>) -> T) -> T {
                    match list_kind.into() {
                        None => access(self.all_factors()),
                        Some(FactorListKind::Threshold) => access(self.threshold_factors.iter().collect()),
                        Some(FactorListKind::Override) => access(self.override_factors.iter().collect()),
                    }
                }

                fn all_factors_of_kind_in_list_of_kind(&self, factor_source_kind: FactorSourceKind, list_kind: impl Into<Option<FactorListKind>>) -> Vec<$factor> {
                    self.with_factor_list_of_kind(list_kind, |factors| factors.into_iter().filter(|f| f.get_factor_source_kind() == factor_source_kind).map(|x| x.clone()).collect())
                }

                fn number_of_factors_of_kind_in_list_of_kind(&self, factor_source_kind: FactorSourceKind, list_kind: impl Into<Option<FactorListKind>>) -> usize {
                    self.all_factors_of_kind_in_list_of_kind(factor_source_kind, list_kind).len()
                }

                fn number_of_factors_of_kind_in_any_list(&self, factor_source_kind: FactorSourceKind) -> usize {
                    self.number_of_factors_of_kind_in_list_of_kind(factor_source_kind, None)
                }

                fn contains_factor_of_kind_in_list_of_kind(&self, factor_source_kind: FactorSourceKind, list_kind: impl Into<Option<FactorListKind>>) -> bool {
                    self.number_of_factors_of_kind_in_list_of_kind(factor_source_kind, list_kind) > 0
                }

                fn contains_factor_of_kind_in_any_list(&self, factor_source_kind: FactorSourceKind) -> bool {
                    self.contains_factor_of_kind_in_list_of_kind(factor_source_kind, None)
                }

                fn factor_list_kind_of_factor_of_kind(&self, kind: FactorSourceKind) -> Option<FactorListKind> {
                    let found_in_threshold = self.contains_factor_of_kind_in_list_of_kind(kind, FactorListKind::Threshold);
                    let found_in_override = self.contains_factor_of_kind_in_list_of_kind(kind, FactorListKind::Threshold);
                    assert!(!(found_in_threshold && found_in_override), "Invalid, factor cannot be in both lists!");
                    if found_in_threshold {
                        Some(FactorListKind::Threshold)
                    } else if found_in_override {
                        Some(FactorListKind::Override)
                    } else {
                        None
                    }
                }

                /// Validates the **Primary** role for `Device` FactorSourceKind.
                fn validate_device_primary(&self) -> PrimaryRoleInIsolationValidation {
                    if self.number_of_factors_of_kind_in_any_list(FactorSourceKind::Device) > 1 {
                        return PrimaryRoleInIsolationValidation::Err(FactorsInvalidReason::ForeverInvalid {
                            violation: FactorRulesViolationPrimaryRoleInIsolation::MultipleDeviceFactors
                        })
                    }
                    Ok(())
                }

                /// Validates the **Recovery** role for `Device` FactorSourceKind.
                fn validate_device_recovery(&self) -> RecoveryRoleInIsolationValidation {
                    self.validate_recovery_contains_no_threshold_factors()
                    // No further validation needed
                }

                /// Validates the **Confirmation** role for `Device` FactorSourceKind.
                fn validate_device_confirmation(&self) -> ConfirmationRoleInIsolationValidation {
                    self.validate_confirmation_contains_no_threshold_factors()
                    // No further validation needed
                }

                /// Validates the **Primary** role for `Ledger` FactorSourceKind.
                ///
                /// It is OK to use Ledger in Primary role (any list, without further restrictions).
                fn validate_ledger_primary(&self) -> PrimaryRoleInIsolationValidation {
                    Ok(())
                    // No further validation needed
                }

                /// Validates the **Recovery** role for `Ledger` FactorSourceKind.
                ///
                /// It is OK to use Ledger in Recovery role (in override list).
                fn validate_ledger_recovery(&self) -> RecoveryRoleInIsolationValidation {
                    self.validate_recovery_contains_no_threshold_factors()
                    // No further validation needed
                }

                /// Validates the **Confirmation** role for `Ledger` FactorSourceKind.
                ///
                /// It is OK to use Ledger in Confirmation role (in override list).
                fn validate_ledger_confirmation(&self) -> ConfirmationRoleInIsolationValidation {
                    self.validate_confirmation_contains_no_threshold_factors()
                    // No further validation needed
                }

                /// Validates the **Primary** role for `Arculus` FactorSourceKind.
                ///
                /// It is OK to use Arculus in Primary role (any list, without further restrictions).
                fn validate_arculus_primary(&self) -> PrimaryRoleInIsolationValidation {
                    Ok(())
                    // No further validation needed
                }

                /// Validates the **Recovery** role for `Arculus` FactorSourceKind.
                ///
                /// It is OK to use Arculus in Recovery role (in override list).
                fn validate_arculus_recovery(&self) -> RecoveryRoleInIsolationValidation {
                    self.validate_recovery_contains_no_threshold_factors()
                    // No further validation needed
                }

                /// Validates the **Confirmation** role for `Arculus` FactorSourceKind.
                ///
                /// It is OK to use Arculus in Confirmation role (in override list).
                fn validate_arculus_confirmation(&self) -> ConfirmationRoleInIsolationValidation {
                    self.validate_confirmation_contains_no_threshold_factors()
                    // No further validation needed
                }

                /// Validates the **Primary** role for `SecurityQuestions` FactorSourceKind.
                ///
                /// It is NOT OK to use SecurityQuestions in Primary role (any list).
                fn validate_security_questions_primary(&self) -> PrimaryRoleInIsolationValidation {
                    if self.contains_factor_of_kind_in_any_list(FactorSourceKind::SecurityQuestions) {
                        return PrimaryRoleInIsolationValidation::Err(FactorsInvalidReason::ForeverInvalid {
                            violation: FactorRulesViolationPrimaryRoleInIsolation::SecurityQuestionsCannotBeUsed
                        })
                    }
                    Ok(())
                }

                /// Validates the **Recovery** role for `SecurityQuestions` FactorSourceKind.
                ///
                /// It is NOT OK to use SecurityQuestions in Recovery role (any list).
                fn validate_security_questions_recovery(&self) -> RecoveryRoleInIsolationValidation {
                    self.validate_recovery_contains_no_threshold_factors()?;
                    if self.contains_factor_of_kind_in_any_list(FactorSourceKind::SecurityQuestions) {
                        return RecoveryRoleInIsolationValidation::Err(FactorsInvalidReason::ForeverInvalid {
                            violation: FactorRulesViolationRecoveryRoleInIsolation::SecurityQuestionsCannotBeUsed
                        })
                    }
                    Ok(())

                }

                /// Validates the **Confirmation** role for `SecurityQuestions` FactorSourceKind.
                ///
                /// It is OK to use SecurityQuestions in Confirmation role (in override list).
                fn validate_security_questions_confirmation(&self) -> ConfirmationRoleInIsolationValidation {
                    self.validate_confirmation_contains_no_threshold_factors()
                    // No further validation needed
                }

                /// Validates the **Primary** role for `Passphrase` FactorSourceKind.
                ///
                /// It is conditionally OK to use Passphrase in Primary if and only if:
                /// * It is used in Threshold list
                /// * The Threshold list contains at least one other kind than Passphrase
                /// * The Threshold is at least 2
                ///
                /// The validation error returned is of type `NotYetValid`, meaning that
                /// the state can become valid if another FactorSource is added to the Threshold list.
                fn validate_passphrase_primary(&self) -> PrimaryRoleInIsolationValidation {
                    if let Some(list_kind) = self.factor_list_kind_of_factor_of_kind(FactorSourceKind::Passphrase) {
                        match list_kind {
                            FactorListKind::Threshold => {
                                let threshold_factors_contains_other_kinds_than_passphrase = self.with_factor_list_of_kind(list_kind, |fs| fs.iter().filter(|f| f.get_factor_source_kind() != FactorSourceKind::Passphrase).count() >= 1);
                                if !threshold_factors_contains_other_kinds_than_passphrase {
                                    return PrimaryRoleInIsolationValidation::Err(FactorsInvalidReason::NotYetValid {
                                        violation: FactorRulesViolationPrimaryRoleInIsolation::ThresholdFactorsMustContainAtLeastOneOtherKindThanPassphrase
                                    })
                                }
                                if self.threshold < 2 {
                                    return PrimaryRoleInIsolationValidation::Err(FactorsInvalidReason::NotYetValid {
                                        violation: FactorRulesViolationPrimaryRoleInIsolation::ThresholdMustBeAtLeastTwoWhenPassphraseIsUsed
                                    })
                                }

                            }
                            FactorListKind::Override => {
                                return PrimaryRoleInIsolationValidation::Err(FactorsInvalidReason::ForeverInvalid {
                                    violation: FactorRulesViolationPrimaryRoleInIsolation::PassphraseCannotBeInOverride
                                })
                            }
                        }
                    }
                    Ok(())
                }

                /// Validates the **Recovery** role for `Passphrase` FactorSourceKind.
                ///
                /// It is NOT OK to use Passphrase in Recovery role (any list).
                fn validate_passphrase_recovery(&self) -> RecoveryRoleInIsolationValidation {
                    self.validate_recovery_contains_no_threshold_factors()?;
                    if self.contains_factor_of_kind_in_any_list(FactorSourceKind::Passphrase) {
                        return RecoveryRoleInIsolationValidation::Err(FactorsInvalidReason::ForeverInvalid {
                            violation: FactorRulesViolationRecoveryRoleInIsolation::PassphraseCannotBeUsed
                        })
                    }
                    Ok(())
                }

                /// Validates the **Confirmation** role for `Passphrase` FactorSourceKind.
                ///
                /// It is OK to use Passphrase in Confirmation role (in override list).
                fn validate_passphrase_confirmation(&self) -> ConfirmationRoleInIsolationValidation {
                    self.validate_confirmation_contains_no_threshold_factors()
                    // No further validation needed
                }

                /// Validates the **Primary** role for `OffDeviceMnemonic` FactorSourceKind.
                ///
                /// It is OK to use OffDeviceMnemonic in Primary role (any list, without further restrictions).
                fn validate_off_device_mnemonic_primary(&self) -> PrimaryRoleInIsolationValidation {
                    Ok(())
                 }

                /// Validates the **Recovery** role for `OffDeviceMnemonic` FactorSourceKind.
                ///
                /// It is OK to use OffDeviceMnemonic in Recovery role (in override list).
                fn validate_off_device_mnemonic_recovery(&self) -> RecoveryRoleInIsolationValidation {
                    self.validate_recovery_contains_no_threshold_factors()
                    // No further validation needed
                 }

                /// Validates the **Confirmation** role for `OffDeviceMnemonic` FactorSourceKind.
                ///
                /// It is OK to use OffDeviceMnemonic in Confirmation role (in override list).
                fn validate_off_device_mnemonic_confirmation(&self) -> ConfirmationRoleInIsolationValidation {
                    self.validate_confirmation_contains_no_threshold_factors()
                    // No further validation needed
                 }

                fn validate_primary(&self) -> PrimaryRoleInIsolationValidation {
                    assert!(self.get_mfa_role() == RoleKind::Recovery, "Wrong role kind, called validation for Primary but role is {:?}", self.get_mfa_role());

                    self.validate_device_primary()?;
                    self.validate_ledger_primary()?;
                    self.validate_arculus_primary()?;
                    self.validate_security_questions_primary()?;
                    self.validate_passphrase_primary()?;
                    self.validate_off_device_mnemonic_primary()?;

                    Ok(())
                }

                fn validate_recovery_contains_no_threshold_factors(&self) -> RecoveryRoleInIsolationValidation {
                    if !self.threshold_factors.is_empty() {
                        return RecoveryRoleInIsolationValidation::Err(FactorsInvalidReason::ForeverInvalid {
                            violation: FactorRulesViolationRecoveryRoleInIsolation::RoleContainsThresholdFactors
                        })
                    }
                    Ok(())
                }

                fn validate_recovery(&self) -> RecoveryRoleInIsolationValidation {
                    assert!(self.get_mfa_role() == RoleKind::Recovery, "Wrong role kind, called validation for Recovery but role is {:?}", self.get_mfa_role());

                    self.validate_recovery_contains_no_threshold_factors()?;

                    self.validate_device_recovery()?;
                    self.validate_ledger_recovery()?;
                    self.validate_arculus_recovery()?;
                    self.validate_security_questions_recovery()?;
                    self.validate_passphrase_recovery()?;
                    self.validate_off_device_mnemonic_recovery()?;

                    Ok(())
                }

                fn validate_confirmation_contains_no_threshold_factors(&self) -> ConfirmationRoleInIsolationValidation {
                    if !self.threshold_factors.is_empty() {
                        return ConfirmationRoleInIsolationValidation::Err(FactorsInvalidReason::ForeverInvalid {
                            violation: FactorRulesViolationConfirmationRoleInIsolation::RoleContainsThresholdFactors
                        })
                    }
                    Ok(())
                }

                fn validate_confirmation(&self) -> ConfirmationRoleInIsolationValidation {
                    assert!(self.get_mfa_role() == RoleKind::Confirmation, "Wrong role kind, called validation for Confirmation but role is {:?}", self.get_mfa_role());

                    self.validate_confirmation_contains_no_threshold_factors()?;

                    self.validate_device_confirmation()?;
                    self.validate_ledger_confirmation()?;
                    self.validate_arculus_confirmation()?;
                    self.validate_security_questions_confirmation()?;
                    self.validate_passphrase_confirmation()?;
                    self.validate_off_device_mnemonic_confirmation()?;
                    Ok(())
                }

                fn validate_strict(&self) -> RolesInIsolationValidation {
                    let role = self.get_mfa_role();
                    match role {
                        RoleKind::Primary => self.validate_primary().into_roles()?,
                        RoleKind::Recovery => self.validate_recovery().into_roles()?,
                        RoleKind::Confirmation => self.validate_confirmation().into_roles()?,
                    }
                    Ok(())
                }

                fn validate(&self, allow_not_yet_valid: bool) -> RolesInIsolationValidation {
                    let strict = self.validate_strict();
                    if allow_not_yet_valid {
                        strict.allow_not_yet_valid()
                    } else {
                        strict
                    }
                }

                fn add_factor_to_list_without_validation(&mut self, factor: &$factor, list_kind: FactorListKind) {
                    match list_kind {
                        FactorListKind::Threshold => {
                            self.threshold_factors.push(factor.clone())
                        }
                        FactorListKind::Override => {
                            self.override_factors.push(factor.clone())
                        }
                    }
                }

                pub fn validation_state_if_add_factor_to_list(&self, factor: &$factor, list_kind: FactorListKind, allow_not_yet_valid: bool) -> RolesInIsolationValidation {
                    let mut copy = self.clone();
                    copy.add_factor_to_list_without_validation(factor, list_kind);
                    copy.validate(allow_not_yet_valid)
                }

                /// Only mutates `self` if validation passes, and validation is conditional on `allow_not_yet_valid`
                pub fn add_factor_to_list(&mut self, factor: &$factor, list_kind: FactorListKind, allow_not_yet_valid: bool) -> RolesInIsolationValidation {
                    self.validation_state_if_add_factor_to_list(factor, list_kind, allow_not_yet_valid)?;
                    self.add_factor_to_list_without_validation(factor, list_kind);
                    Ok(())
                }

                pub fn unique_factors(&self) -> IndexSet<$factor> {
                    self.all_factors().into_iter().map(|x| x.clone()).collect()
                }


                /// If `validation` is not `Skip`, we require the structure of factors to be valid.
                ///
                /// # Panics
                /// Panics if `threshold > threshold_factor.len()`
                ///
                /// Panics if the same factor is present in both lists
                ///
                /// Panics if Factor elements are FactorInstances and the derivation
                /// path contains a non-securified last path component.
                pub fn with_factors_and_role(
                    $($extra_field_name: $extra_field_type,)*
                    threshold_factors: impl IntoIterator<Item = $factor>,
                    threshold: u8,
                    override_factors: impl IntoIterator<Item = $factor>,
                    validation: FactorRolesValidation,
                ) -> Result<Self> {

                    let assert_is_securified = |factors: &Vec::<$factor>| -> Result<()> {
                        let trait_objects: Vec<&dyn IsMaybeKeySpaceAware> = factors.iter().map(|x| x as &dyn IsMaybeKeySpaceAware).collect();
                        if trait_objects.iter()
                        .filter_map(|x| x.maybe_key_space())
                        .any(|x| x != KeySpace::Securified) {
                            return Err(crate::CommonError::IndexUnsecurifiedExpectedSecurified)
                        }
                        Ok(())
                    };


                    let threshold_factors = threshold_factors.into_iter().collect_vec();

                    if threshold_factors.len() < threshold as usize {
                        return Err(CommonError::InvalidSecurityStructureThresholdExceedsFactors {
                            threshold,
                            factors: threshold_factors.len() as u8
                        })
                    }

                    let override_factors = override_factors.into_iter().collect_vec();

                    assert_is_securified(&threshold_factors)?;
                    assert_is_securified(&override_factors)?;

                    if !HashSet::<$factor>::from_iter(threshold_factors.clone())
                            .intersection(&HashSet::<$factor>::from_iter(override_factors.clone()))
                            .collect_vec()
                            .is_empty() {
                        return Err(CommonError::InvalidSecurityStructureFactorInBothThresholdAndOverride)
                    }

                    let unvalidated = Self {
                        threshold_factors,
                        threshold,
                        override_factors,
                        $($extra_field_name,)*
                    };


                    match validation {
                        FactorRolesValidation::Skip => Ok(unvalidated),
                        FactorRolesValidation::Validate { allow_not_yet_valid } => {
                            unvalidated.validate(allow_not_yet_valid).map_err(|e| CommonError::from(e))?;
                            let validated = unvalidated;
                            Ok(validated)
                        }
                    }
                }
            }
        }
    };
}

pub(crate) use decl_role_with_factors_with_role_kind_attrs;

macro_rules! decl_role_with_factors {
    (
        $(
            #[doc = $expr: expr]
        )*
        $role: ident,
        $factor: ident
    ) => {

        decl_role_with_factors_with_role_kind_attrs!(
            $(
                #[doc = $expr]
            )*
            $role,
            $factor,
        );


        paste! {


            impl HasMfaRole for [< $role RoleWith $factor s >] {
                fn mfa_role() -> RoleKind {
                    RoleKind::$role
                }
            }

           impl [< $role RoleWith $factor s >] {

                pub fn new(
                    threshold_factors: impl IntoIterator<Item = $factor>,
                    threshold: u8,
                    override_factors: impl IntoIterator<Item = $factor>
                ) -> Result<Self> {
                    Self::with_factors_and_role(threshold_factors, threshold, override_factors, FactorRolesValidation::Skip /* TODO: MFA-Rules: change to `Validate` */)
                }


                /// # Panics
                /// Panics if `threshold > factors.len()`
                ///
                /// Panics if Factor elements are FactorInstances and the derivation
                /// path contains a non-securified last path component.
                pub fn threshold_factors_only(
                    factors: impl IntoIterator<Item = $factor>,
                    threshold: u8,
                ) -> Result<Self> {
                    Self::new(factors, threshold, [])
                }

                /// # Panics
                /// Panics if Factor elements are FactorInstances and the derivation
                /// path contains a non-securified last path component.
                pub fn override_only(
                    factors: impl IntoIterator<Item = $factor>,
                ) -> Result<Self> {
                    Self::new([], 0, factors)
                }
            }
        }

        decl_role_with_factors_additional_impl!($role, $factor);
    };
}

pub(crate) use decl_role_with_factors;

macro_rules! decl_role_runtime_kind_with_factors {
    (
        $(
            #[doc = $expr: expr]
        )*
        $role: ident,
        $factor: ident
    ) => {
        decl_role_with_factors_with_role_kind_attrs!(
            $(
                #[doc = $expr]
            )*
            $role,
            $factor,
            role: RoleKind,
        );

        paste! {
            impl HasMfaRoleObjectSafe for [< $role RoleWith $factor s >] {
                fn get_mfa_role(&self) -> RoleKind {
                    self.role
                }
            }
        }

    };
}

pub(crate) use decl_role_runtime_kind_with_factors;

macro_rules! decl_matrix_of_factors {
    (
        $(
            #[doc = $expr: expr]
        )*
        $factor: ident
    ) => {
        paste! {

            decl_role_with_factors!(
                /// PrimaryRole is used for Signing Transactions.
                Primary,
                $factor
            );

            decl_role_with_factors!(
                /// RecoveryRole is used to recover lost access to an entity.
                Recovery,
                $factor
            );

            decl_role_with_factors!(
                /// ConfirmationRole is used to confirm recovery.
                Confirmation,
                $factor
            );

            $(
                #[doc = $expr]
            )*
            #[derive(
                Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash,
            )]
            #[serde(rename_all = "camelCase")]
            pub struct [< MatrixOf $factor s >] {

                /// Used for Signing transactions
                pub primary_role: [< PrimaryRoleWith $factor s >],

                /// Used to initiate recovery - resetting the used Security Shield
                /// of an entity.
                pub recovery_role: [< RecoveryRoleWith $factor s >],

                /// To confirm recovery.
                pub confirmation_role: [< ConfirmationRoleWith $factor s >],
            }

            impl [< MatrixOf $factor s >] {

                fn validate_roles_in_isolation(&self) -> RolesInIsolationValidation {
                    Ok(())
                }

                fn validate_roles_combined(&self) -> RolesCombinedValidation {
                    Ok(())
                }

                fn validate_strict(&self) -> FactorRulesValidation {
                    self.validate_roles_in_isolation().into_structure()?;
                    self.validate_roles_combined().into_structure()?;
                    Ok(())
                }

                fn validate(&self, allow_not_yet_valid: bool) -> FactorRulesValidation {
                    let strict = self.validate_strict();
                    if allow_not_yet_valid {
                        strict.allow_not_yet_valid()
                    } else {
                        strict
                    }
                }

                pub fn new(
                    primary_role: [< PrimaryRoleWith $factor s >],
                    recovery_role: [< RecoveryRoleWith $factor s >],
                    confirmation_role: [< ConfirmationRoleWith $factor s >],
                    validation: FactorRolesValidation,
                ) -> Result<Self> {
                    let unvalidated = Self {
                        primary_role,
                        recovery_role,
                        confirmation_role,
                    };
                    match validation {
                        FactorRolesValidation::Skip => return Ok(unvalidated),
                        FactorRolesValidation::Validate { allow_not_yet_valid } => {
                            unvalidated.validate(allow_not_yet_valid).map_err(|e| CommonError::from(e))?;
                            let validated = unvalidated;
                            Ok(validated)
                        }
                    }

                }

                pub fn all_factors(&self) -> Vec<&$factor> {
                    let mut factors = Vec::new();
                    factors.extend(self.primary_role.all_factors());
                    factors.extend(self.recovery_role.all_factors());
                    factors.extend(self.confirmation_role.all_factors());
                    factors
                }

                pub fn get_role_of_kind(&self, role_kind: RoleKind) -> &dyn RoleWithFactors<$factor> {
                    match role_kind {
                        RoleKind::Confirmation => &self.confirmation_role,
                        RoleKind::Primary => &self.primary_role,
                        RoleKind::Recovery => &self.recovery_role,
                    }
                }
            }
        }
    };
}

pub(crate) use decl_matrix_of_factors;

macro_rules! decl_security_structure_of {
    (
        $(
            #[doc = $expr: expr]
        )*
        $factor: ident,
    ) => {

        decl_matrix_of_factors!($factor);

        paste! {

            $(
                #[doc = $expr]
            )*
            #[derive(
                Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash,
            )]
            #[serde(rename_all = "camelCase")]
            pub struct [< SecurityStructureOf $factor s >] {
                /// Metadata of this Security Structure, such as globally unique and
                /// stable identifier, creation date and user chosen label (name).
                pub metadata: SecurityStructureMetadata,

                /// The amount of time until Confirmation Role is automatically
                /// exercised, inputted by user in Days in UI, but translate it into
                /// epochs ("block time").
                pub number_of_epochs_until_auto_confirmation: u64,

                /// The structure of factors to use for certain roles, Primary, Recovery
                /// and Confirmation role.
                pub matrix_of_factors: [< MatrixOf $factor s >],
            }

            impl [< SecurityStructureOf $factor s >] {
                pub fn new(metadata: SecurityStructureMetadata, number_of_epochs_until_auto_confirmation: u64, matrix_of_factors: [< MatrixOf $factor s >]) -> Self {
                    Self {
                        metadata,
                        number_of_epochs_until_auto_confirmation,
                        matrix_of_factors
                    }
                }

                pub fn all_factors(&self) -> Vec<&$factor> {
                    self.matrix_of_factors.all_factors()
                }
            }
        }
    };
}

pub(crate) use decl_security_structure_of;
