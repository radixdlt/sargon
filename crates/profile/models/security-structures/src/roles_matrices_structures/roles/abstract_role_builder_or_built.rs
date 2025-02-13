use serde::{Deserialize, Serialize};

use crate::prelude::*;

/// Either a role or a **builder of a role** with a threshold, threshold_factors and override_factors.
/// This type is shared by:
/// # Builder
/// * PrimaryRoleBuilder (FactorSourceID)
/// * RecoveryRoleBuilder (FactorSourceID)
/// * ConfirmationRoleBuilder (FactorSourceID)
///
/// # Built
///
/// ## FactorSourceID
/// * PrimaryRoleWithFactorSourceID
/// * RecoveryRoleWithFactorSourceID
/// * ConfirmationRoleWithFactorSourceID
///
/// ## FactorSource
/// * PrimaryRoleWithFactorSource
/// * RecoveryRoleWithFactorSource
/// * ConfirmationRoleWithFactorSource
///
/// ## FactorInstance
/// * PrimaryRoleWithFactorInstances
/// * RecoveryRoleWithFactorInstances
/// * ConfirmationRoleWithFactorInstances
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbstractRoleBuilderOrBuilt<const ROLE: u8, const MODE: u8, FACTOR> {
    /// How many threshold factors that must be used to perform some function with
    /// this role.
    threshold: Threshold,

    /// Factors which are used in combination with other factors, amounting to at
    /// least `threshold` many factors to perform some function with this role.
    threshold_factors: Vec<FACTOR>,

    /// Overriding / Super admin / "sudo" / God / factors, **ANY**
    /// single of these factor which can perform the function of this role,
    /// disregarding of `threshold`.
    override_factors: Vec<FACTOR>,
}

impl<FACTOR: IsMaybeKeySpaceAware>
    AbstractRoleBuilderOrBuilt<ROLE_RECOVERY, IS_BUILT_ROLE, FACTOR>
{
    /// WEAK SHIELD!
    ///
    /// # Safety
    /// Rust memory safe, but marked "unsafe" since it might allow for instantiation
    /// of unsafe - as in application **unsecure** - Role of Factors, which might
    /// lead to increase risk for end user to loose funds.
    pub unsafe fn empty() -> Self {
        Self::with_factors_and_threshold(Threshold::All, [], [])
    }
}

#[cfg(debug_assertions)]
impl<FACTOR: IsMaybeKeySpaceAware>
    AbstractRoleBuilderOrBuilt<ROLE_RECOVERY, IS_BUILT_ROLE, FACTOR>
{
    pub fn override_only(
        override_factors: impl IntoIterator<Item = FACTOR>,
    ) -> Self {
        Self::with_factors_and_threshold(Threshold::All, [], override_factors)
    }
}

impl<FACTOR: IsMaybeKeySpaceAware>
    AbstractRoleBuilderOrBuilt<ROLE_CONFIRMATION, IS_BUILT_ROLE, FACTOR>
{
    /// WEAK SHIELD!
    ///
    /// # Safety
    /// Rust memory safe, but marked "unsafe" since it might allow for instantiation
    /// of unsafe - as in application **unsecure** - Role of Factors, which might
    /// lead to increase risk for end user to loose funds.
    pub unsafe fn empty() -> Self {
        Self::with_factors_and_threshold(Threshold::All, [], [])
    }
}

#[cfg(debug_assertions)]
impl<FACTOR: IsMaybeKeySpaceAware>
    AbstractRoleBuilderOrBuilt<ROLE_CONFIRMATION, IS_BUILT_ROLE, FACTOR>
{
    pub fn override_only(
        override_factors: impl IntoIterator<Item = FACTOR>,
    ) -> Self {
        Self::with_factors_and_threshold(Threshold::All, [], override_factors)
    }
}

pub(crate) type AbstractBuiltRoleWithFactor<const ROLE: u8, FACTOR> =
    AbstractRoleBuilderOrBuilt<ROLE, IS_BUILT_ROLE, FACTOR>;

pub(crate) type RoleBuilder<const ROLE: u8> =
    AbstractRoleBuilderOrBuilt<ROLE, IS_ROLE_BUILDER, FactorSourceID>;

impl<const ROLE: u8, const MODE: u8, FACTOR: IsMaybeKeySpaceAware>
    AbstractRoleBuilderOrBuilt<ROLE, MODE, FACTOR>
{
    /// Removes all factors from this role and set threshold to `All`.
    pub fn reset(&mut self) {
        self.threshold_factors.clear();
        self.threshold = Threshold::All;
        self.override_factors.clear();
    }

    /// Removes all override factors from this role.
    pub fn remove_all_override_factors(&mut self) {
        self.override_factors.clear();
    }

    pub fn role(&self) -> RoleKind {
        RoleKind::from_u8(ROLE).expect("RoleKind should be valid")
    }

    /// # Safety
    /// Rust memory safe, but marked "unsafe" since it might allow for instantiation
    /// of unsafe - as in application **unsecure** - Role of Factors, which might
    /// lead to increase risk for end user to loose funds.
    pub unsafe fn unbuilt_with_factors(
        threshold: Threshold,
        threshold_factors: impl IntoIterator<Item = FACTOR>,
        override_factors: impl IntoIterator<Item = FACTOR>,
    ) -> Self {
        let assert_is_securified =
            |factors: &Vec<FACTOR>| -> Result<(), CommonError> {
                let trait_objects: Vec<&dyn IsMaybeKeySpaceAware> = factors
                    .iter()
                    .map(|x| x as &dyn IsMaybeKeySpaceAware)
                    .collect();
                if trait_objects
                    .iter()
                    .filter_map(|x| x.maybe_key_space())
                    .any(|x| x != KeySpace::Securified)
                {
                    return Err(
                        CommonError::IndexUnsecurifiedExpectedSecurified,
                    );
                }
                Ok(())
            };

        let threshold_factors = threshold_factors.into_iter().collect();
        let override_factors = override_factors.into_iter().collect();

        assert_is_securified(&threshold_factors)
            .expect("Should not have allowed building of invalid Role");
        assert_is_securified(&override_factors)
            .expect("Should not have allowed building of invalid Role");

        Self {
            threshold,
            threshold_factors,
            override_factors,
        }
    }

    pub fn with_factors(
        threshold: u8,
        threshold_factors: impl IntoIterator<Item = FACTOR>,
        override_factors: impl IntoIterator<Item = FACTOR>,
    ) -> Self {
        unsafe {
            Self::unbuilt_with_factors(
                Threshold::Specific(threshold),
                threshold_factors,
                override_factors,
            )
        }
    }

    pub fn with_factors_and_threshold(
        threshold: Threshold,
        threshold_factors: impl IntoIterator<Item = FACTOR>,
        override_factors: impl IntoIterator<Item = FACTOR>,
    ) -> Self {
        unsafe {
            Self::unbuilt_with_factors(
                threshold,
                threshold_factors,
                override_factors,
            )
        }
    }
}

impl<const ROLE: u8, const MODE: u8, FACTOR>
    AbstractRoleBuilderOrBuilt<ROLE, MODE, FACTOR>
{
    /// Threshold and Override factors mixed (threshold first).
    pub fn all_factors(&self) -> Vec<&FACTOR> {
        self.threshold_factors
            .iter()
            .chain(self.override_factors.iter())
            .collect()
    }

    /// Factors which are used in combination with other factors, amounting to at
    /// least `threshold` many factors to perform some function with this role.
    pub fn get_threshold_factors(&self) -> &Vec<FACTOR> {
        &self.threshold_factors
    }

    /// Overriding / Super admin / "sudo" / God / factors, **ANY**
    /// single of these factor which can perform the function of this role,
    /// disregarding of `threshold`.
    pub fn get_override_factors(&self) -> &Vec<FACTOR> {
        &self.override_factors
    }

    /// How many threshold factors that must be used to perform some function with
    /// this role.
    pub fn get_threshold_value(&self) -> u8 {
        self.threshold.value(self.threshold_factors.len())
    }

    /// The kind of threshold that must be used to perform some function with this role.
    pub fn get_threshold(&self) -> Threshold {
        self.threshold
    }
}
pub(crate) const ROLE_PRIMARY: u8 = 1;
pub(crate) const ROLE_RECOVERY: u8 = 2;
pub(crate) const ROLE_CONFIRMATION: u8 = 3;

pub(crate) trait RoleFromDiscriminator {
    fn from_u8(discriminator: u8) -> Option<Self>
    where
        Self: Sized;
}
impl RoleFromDiscriminator for RoleKind {
    fn from_u8(discriminator: u8) -> Option<Self> {
        match discriminator {
            ROLE_PRIMARY => Some(RoleKind::Primary),
            ROLE_RECOVERY => Some(RoleKind::Recovery),
            ROLE_CONFIRMATION => Some(RoleKind::Confirmation),
            _ => None,
        }
    }
}

impl<const ROLE: u8> RoleBuilder<ROLE> {
    pub(crate) fn new() -> Self {
        Self {
            threshold: Threshold::All,
            threshold_factors: Vec::new(),
            override_factors: Vec::new(),
        }
    }

    pub(crate) fn mut_threshold_factors(&mut self) -> &mut Vec<FactorSourceID> {
        &mut self.threshold_factors
    }

    pub(crate) fn mut_override_factors(&mut self) -> &mut Vec<FactorSourceID> {
        &mut self.override_factors
    }

    pub(crate) fn unchecked_add_factor_source_to_list(
        &mut self,
        factor_source_id: FactorSourceID,
        factor_list_kind: FactorListKind,
    ) {
        match factor_list_kind {
            FactorListKind::Threshold => {
                self.threshold_factors.push(factor_source_id)
            }
            FactorListKind::Override => {
                self.override_factors.push(factor_source_id)
            }
        }
    }

    pub(crate) fn unchecked_set_threshold(&mut self, threshold: Threshold) {
        self.threshold = threshold;
    }
}
