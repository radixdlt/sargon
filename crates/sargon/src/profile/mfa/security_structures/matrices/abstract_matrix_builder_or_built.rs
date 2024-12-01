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
pub struct AbstractMatrixBuilderOrBuilt<F, T, U> {
    #[serde(skip)]
    #[doc(hidden)]
    pub(crate) built: PhantomData<T>,

    pub(crate) primary_role: AbstractRoleBuilderOrBuilt<{ ROLE_PRIMARY }, F, U>,
    pub(crate) recovery_role:
        AbstractRoleBuilderOrBuilt<{ ROLE_RECOVERY }, F, U>,
    pub(crate) confirmation_role:
        AbstractRoleBuilderOrBuilt<{ ROLE_CONFIRMATION }, F, U>,

    pub number_of_days_until_auto_confirm: u16,
}

impl<F, T, U> AbstractMatrixBuilderOrBuilt<F, T, U> {
    pub const DEFAULT_NUMBER_OF_DAYS_UNTIL_AUTO_CONFIRM: u16 = 14;
}

pub type AbstractMatrixBuilt<F> = AbstractMatrixBuilderOrBuilt<F, (), ()>;

impl<F> AbstractMatrixBuilt<F> {
    pub fn primary(&self) -> &AbstractBuiltRoleWithFactor<{ ROLE_PRIMARY }, F> {
        &self.primary_role
    }

    pub fn recovery(
        &self,
    ) -> &AbstractBuiltRoleWithFactor<{ ROLE_RECOVERY }, F> {
        &self.recovery_role
    }

    pub fn confirmation(
        &self,
    ) -> &AbstractBuiltRoleWithFactor<{ ROLE_CONFIRMATION }, F> {
        &self.confirmation_role
    }
}

impl<F: std::cmp::Eq + std::hash::Hash> AbstractMatrixBuilt<F> {
    pub fn all_factors(&self) -> HashSet<&F> {
        let mut factors = HashSet::new();
        factors.extend(self.primary_role.all_factors());
        factors.extend(self.recovery_role.all_factors());
        factors.extend(self.confirmation_role.all_factors());
        factors
    }
}
