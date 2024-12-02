#![allow(non_camel_case_types)]

use crate::prelude::*;

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
pub struct AbstractMatrixBuilderOrBuilt<FACTOR, BUILT_MATRIX, BUILT_ROLE> {
    #[serde(skip)]
    #[doc(hidden)]
    pub(crate) built: PhantomData<BUILT_MATRIX>,

    pub(crate) primary_role:
        AbstractRoleBuilderOrBuilt<{ ROLE_PRIMARY }, FACTOR, BUILT_ROLE>,
    pub(crate) recovery_role:
        AbstractRoleBuilderOrBuilt<{ ROLE_RECOVERY }, FACTOR, BUILT_ROLE>,
    pub(crate) confirmation_role:
        AbstractRoleBuilderOrBuilt<{ ROLE_CONFIRMATION }, FACTOR, BUILT_ROLE>,

    pub number_of_days_until_auto_confirm: u16,
}

impl<FACTOR, BUILT_MATRIX, BUILT_ROLE>
    AbstractMatrixBuilderOrBuilt<FACTOR, BUILT_MATRIX, BUILT_ROLE>
{
    pub const DEFAULT_NUMBER_OF_DAYS_UNTIL_AUTO_CONFIRM: u16 = 14;

    /// # Safety
    /// Rust memory safe, but marked "unsafe" since it might allow for instantiation
    /// of unsafe - as in application **unsecure** - MatrixofFactors, which might
    /// lead to increase risk for end user to loose funds.
    pub unsafe fn unbuilt_with_roles_and_days(
        primary: AbstractRoleBuilderOrBuilt<
            { ROLE_PRIMARY },
            FACTOR,
            BUILT_ROLE,
        >,
        recovery: AbstractRoleBuilderOrBuilt<
            { ROLE_RECOVERY },
            FACTOR,
            BUILT_ROLE,
        >,
        confirmation: AbstractRoleBuilderOrBuilt<
            { ROLE_CONFIRMATION },
            FACTOR,
            BUILT_ROLE,
        >,
        number_of_days_until_auto_confirm: u16,
    ) -> Self {
        Self {
            built: PhantomData,
            primary_role: primary,
            recovery_role: recovery,
            confirmation_role: confirmation,
            number_of_days_until_auto_confirm,
        }
    }
}

pub type AbstractMatrixBuilt<FACTOR> =
    AbstractMatrixBuilderOrBuilt<FACTOR, (), ()>;

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
