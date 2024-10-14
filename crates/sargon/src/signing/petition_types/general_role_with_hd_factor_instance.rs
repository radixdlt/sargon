use crate::prelude::*;

decl_role_with_factors!(
    /// A general depiction of each of the roles in a `MatrixOfFactorInstances`.
    /// `SignaturesCollector` can work on any `RoleKind` when dealing with a securified entity.
    General,
    HierarchicalDeterministicFactorInstance
);

impl TryFrom<(MatrixOfFactorInstances, RoleKind)>
    for GeneralRoleWithHierarchicalDeterministicFactorInstances
{
    type Error = CommonError;

    fn try_from((matrix, role): (MatrixOfFactorInstances, RoleKind)) -> Result<Self> {
        let (threshold_factors, threshold, override_factors) = match role {
            RoleKind::Primary => (
                matrix.primary_role.threshold_factors,
                matrix.primary_role.threshold,
                matrix.primary_role.override_factors,
            ),
            RoleKind::Recovery => (
                matrix.recovery_role.threshold_factors,
                matrix.recovery_role.threshold,
                matrix.recovery_role.override_factors,
            ),
            RoleKind::Confirmation => (
                matrix.confirmation_role.threshold_factors,
                matrix.confirmation_role.threshold,
                matrix.confirmation_role.override_factors,
            ),
        };

        let general_role = GeneralRoleWithHierarchicalDeterministicFactorInstances::new(
            threshold_factors
                .iter()
                .map(|f| HierarchicalDeterministicFactorInstance::try_from_factor_instance(f.clone()))
                .collect::<Result<Vec<HierarchicalDeterministicFactorInstance>>>()?,
            threshold,
            override_factors
                .iter()
                .map(|f| HierarchicalDeterministicFactorInstance::try_from_factor_instance(f.clone()))
                .collect::<Result<Vec<HierarchicalDeterministicFactorInstance>>>()?,
        );

        Ok(general_role)
    }
}

impl From<GeneralRoleWithHierarchicalDeterministicFactorInstances>
    for MatrixOfFactorInstances
{
    fn from(
        value: GeneralRoleWithHierarchicalDeterministicFactorInstances,
    ) -> Self {
        let threshold_factors = value
            .threshold_factors
            .iter()
            .map(|hd| hd.factor_instance())
            .collect::<Vec<FactorInstance>>();

        let override_factors = value
            .override_factors
            .iter()
            .map(|hd| hd.factor_instance())
            .collect::<Vec<FactorInstance>>();

        MatrixOfFactorInstances::new(
            PrimaryRoleWithFactorInstances::new(
                threshold_factors.clone(),
                value.threshold,
                override_factors.clone(),
            ),
            RecoveryRoleWithFactorInstances::new(
                threshold_factors.clone(),
                value.threshold,
                override_factors.clone(),
            ),
            ConfirmationRoleWithFactorInstances::new(
                threshold_factors.clone(),
                value.threshold,
                override_factors.clone(),
            ),
        )
    }
}

impl GeneralRoleWithHierarchicalDeterministicFactorInstances {
    pub fn override_only(
        factors: impl IntoIterator<Item = HierarchicalDeterministicFactorInstance>,
    ) -> Self {
        Self::new([], 0, factors)
    }

    pub fn single_override(
        factor: HierarchicalDeterministicFactorInstance,
    ) -> Self {
        Self::override_only([factor])
    }

    pub fn threshold_only(
        factors: impl IntoIterator<Item = HierarchicalDeterministicFactorInstance>,
        threshold: u8,
    ) -> Self {
        Self::new(factors, threshold, [])
    }

    pub fn single_threshold(
        factor: HierarchicalDeterministicFactorInstance,
    ) -> Self {
        Self::threshold_only([factor], 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = GeneralRoleWithHierarchicalDeterministicFactorInstances;

    #[test]
    fn test_from_primary_role() {
        assert_eq!(
            GeneralRoleWithHierarchicalDeterministicFactorInstances::try_from(
                (matrix(), RoleKind::Primary)
            ).unwrap(),
            SUT::new(
                [HierarchicalDeterministicFactorInstance::try_from_factor_instance(FactorInstance::sample()).unwrap()],
                1,
                []
            )
        )
    }

    #[test]
    fn test_from_recovery_role() {
        assert_eq!(
            GeneralRoleWithHierarchicalDeterministicFactorInstances::try_from(
                (matrix(), RoleKind::Recovery)
            ).unwrap(),
            SUT::new(
                [HierarchicalDeterministicFactorInstance::try_from_factor_instance(
                    FactorInstance::new(
                        FactorSourceIDFromHash::sample_ledger().into(),
                        FactorInstanceBadge::sample()
                    )
                ).unwrap()],
                1,
                []
            )
        )
    }

    #[test]
    fn test_from_confirmation_role() {
        assert_eq!(
            GeneralRoleWithHierarchicalDeterministicFactorInstances::try_from(
                (matrix(), RoleKind::Confirmation)
            ).unwrap(),
            SUT::new(
                [HierarchicalDeterministicFactorInstance::try_from_factor_instance(
                    FactorInstance::new(
                        FactorSourceIDFromHash::sample_passphrase().into(),
                        FactorInstanceBadge::sample()
                    )
                ).unwrap()],
                1,
                []
            )
        )
    }

    #[test]
    fn test_from_matrix_containing_physical_badge() {
        let matrix = MatrixOfFactorInstances::new(
            PrimaryRoleWithFactorInstances::new(
                [FactorInstance::sample_other()],
                1,
                [],
            ),
            recovery_role(),
            confirmation_role(),
        );

        assert_eq!(
            GeneralRoleWithHierarchicalDeterministicFactorInstances::try_from(
                (matrix, RoleKind::Primary)
            ),
            Err(CommonError::BadgeIsNotVirtualHierarchicalDeterministic)
        );
    }

    fn matrix() -> MatrixOfFactorInstances {
        MatrixOfFactorInstances::new(
            primary_role(),
            recovery_role(),
            confirmation_role(),
        )
    }

    fn primary_role() -> PrimaryRoleWithFactorInstances {
        PrimaryRoleWithFactorInstances::new([FactorInstance::sample()], 1, [])
    }

    fn recovery_role() -> RecoveryRoleWithFactorInstances {
        RecoveryRoleWithFactorInstances::new(
            [FactorInstance::new(
                FactorSourceIDFromHash::sample_ledger().into(),
                FactorInstanceBadge::sample(),
            )],
            1,
            [],
        )
    }

    fn confirmation_role() -> ConfirmationRoleWithFactorInstances {
        ConfirmationRoleWithFactorInstances::new(
            [FactorInstance::new(
                FactorSourceIDFromHash::sample_passphrase().into(),
                FactorInstanceBadge::sample(),
            )],
            1,
            [],
        )
    }
}
