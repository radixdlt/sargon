#![allow(non_camel_case_types)]

use crate::prelude::*;

/// One of two possible `MODE_OF_MATRIX` values, used for the **builder of a matrix**.
pub const IS_MATRIX_BUILDER: u8 = 0;
/// One of two possible `MODE_OF_MATRIX` values, used for the **built matrix**.
pub const IS_BUILT_MATRIX: u8 = 1;

/// One of two possible `MODE_OF_ROLE` values, used for the **builder of roles**.
pub const IS_ROLE_BUILDER: u8 = 0;

/// One of two possible `MODE_OF_ROLE` values, used for the **built roles**.
pub const IS_BUILT_ROLE: u8 = 0;

/// Either a matrix or a **builder of a matrix** with a Primary, Recovery and Confirmation
/// role or **builder of roles**.
/// This type is shared by:
/// * MatrixBuilder (FactorSourceID)
///
/// # Built
/// * MatrixOfFactorSources
/// * MatrixOfFactorSourceIds
/// * MatrixOfFactorInstances
///
/// For "built types" the `built` field is `PhantomData<()>`, for the `MatrixBuilder`
/// it is `PhantomData<FactorSourceID>`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbstractMatrixBuilderOrBuilt<
    const MODE_OF_MATRIX: u8,
    const MODE_OF_ROLE: u8,
    FACTOR,
> {
    pub(crate) primary_role:
        AbstractRoleBuilderOrBuilt<{ ROLE_PRIMARY }, MODE_OF_ROLE, FACTOR>,
    pub(crate) recovery_role:
        AbstractRoleBuilderOrBuilt<{ ROLE_RECOVERY }, MODE_OF_ROLE, FACTOR>,
    pub(crate) confirmation_role:
        AbstractRoleBuilderOrBuilt<{ ROLE_CONFIRMATION }, MODE_OF_ROLE, FACTOR>,

    pub time_until_delayed_confirmation_is_callable: TimePeriod,
}

impl<const MODE_OF_MATRIX: u8, const MODE_OF_ROLE: u8, FACTOR>
    AbstractMatrixBuilderOrBuilt<MODE_OF_MATRIX, MODE_OF_ROLE, FACTOR>
{
    pub const DEFAULT_TIME_UNTIL_DELAYED_CONFIRMATION_IS_CALLABLE: TimePeriod =
        TimePeriod::with_days(14);

    /// # Safety
    /// Rust memory safe, but marked "unsafe" since it might allow for unsafe
    /// SecurityShields.
    pub unsafe fn set_primary_role(
        &mut self,
        primary_role: AbstractRoleBuilderOrBuilt<
            { ROLE_PRIMARY },
            MODE_OF_ROLE,
            FACTOR,
        >,
    ) {
        self.primary_role = primary_role
    }

    /// # Safety
    /// Rust memory safe, but marked "unsafe" since it might allow for unsafe
    /// SecurityShields.
    pub unsafe fn set_recovery_role(
        &mut self,
        recovery_role: AbstractRoleBuilderOrBuilt<
            { ROLE_RECOVERY },
            MODE_OF_ROLE,
            FACTOR,
        >,
    ) {
        self.recovery_role = recovery_role
    }

    /// # Safety
    /// Rust memory safe, but marked "unsafe" since it might allow for unsafe
    /// SecurityShields.
    pub unsafe fn set_confirmation_role(
        &mut self,
        confirmation_role: AbstractRoleBuilderOrBuilt<
            { ROLE_CONFIRMATION },
            MODE_OF_ROLE,
            FACTOR,
        >,
    ) {
        self.confirmation_role = confirmation_role
    }

    /// # Safety
    /// Rust memory safe, but marked "unsafe" since it might allow for instantiation
    /// of unsafe - as in application **unsecure** - MatrixOfFactors, which might
    /// lead to increase risk for end user to loose funds.
    pub unsafe fn unbuilt_with_roles_and_days(
        primary_role: AbstractRoleBuilderOrBuilt<
            { ROLE_PRIMARY },
            MODE_OF_ROLE,
            FACTOR,
        >,
        recovery_role: AbstractRoleBuilderOrBuilt<
            { ROLE_RECOVERY },
            MODE_OF_ROLE,
            FACTOR,
        >,
        confirmation_role: AbstractRoleBuilderOrBuilt<
            { ROLE_CONFIRMATION },
            MODE_OF_ROLE,
            FACTOR,
        >,
        time_until_delayed_confirmation_is_callable: TimePeriod,
    ) -> Self {
        Self {
            primary_role,
            recovery_role,
            confirmation_role,
            time_until_delayed_confirmation_is_callable,
        }
    }
}

pub type AbstractMatrixBuilt<FACTOR> =
    AbstractMatrixBuilderOrBuilt<IS_BUILT_MATRIX, IS_BUILT_ROLE, FACTOR>;

impl<FACTOR> AbstractMatrixBuilt<FACTOR> {
    pub fn primary(
        &self,
    ) -> &AbstractBuiltRoleWithFactor<{ ROLE_PRIMARY }, FACTOR> {
        &self.primary_role
    }

    pub fn recovery(
        &self,
    ) -> &AbstractBuiltRoleWithFactor<{ ROLE_RECOVERY }, FACTOR> {
        &self.recovery_role
    }

    pub fn confirmation(
        &self,
    ) -> &AbstractBuiltRoleWithFactor<{ ROLE_CONFIRMATION }, FACTOR> {
        &self.confirmation_role
    }
}

impl<FACTOR: std::cmp::Eq + std::hash::Hash> AbstractMatrixBuilt<FACTOR> {
    pub fn all_factors(&self) -> HashSet<&FACTOR> {
        let mut factors = HashSet::new();
        factors.extend(self.primary_role.all_factors());
        factors.extend(self.recovery_role.all_factors());
        factors.extend(self.confirmation_role.all_factors());
        factors
    }
}
