use crate::prelude::*;

/// A general depiction of each of the roles in a `MatrixOfFactorInstances`.
/// `SignaturesCollector` can work on any `RoleKind` when dealing with a securified entity.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct GeneralRoleWithHierarchicalDeterministicFactorInstances {
    role: RoleKind,
    threshold: u8,
    threshold_factors: Vec<HierarchicalDeterministicFactorInstance>,
    override_factors: Vec<HierarchicalDeterministicFactorInstance>,
}

impl GeneralRoleWithHierarchicalDeterministicFactorInstances {
    pub fn get_threshold(&self) -> u8 {
        self.threshold
    }

    pub fn get_threshold_factors(
        &self,
    ) -> Vec<HierarchicalDeterministicFactorInstance> {
        self.threshold_factors.clone()
    }

    pub fn get_override_factors(
        &self,
    ) -> Vec<HierarchicalDeterministicFactorInstance> {
        self.override_factors.clone()
    }

    pub fn with_factors_and_role(
        role: RoleKind,
        threshold_factors: impl IntoIterator<
            Item = HierarchicalDeterministicFactorInstance,
        >,
        threshold: u8,
        override_factors: impl IntoIterator<
            Item = HierarchicalDeterministicFactorInstance,
        >,
    ) -> Result<Self, CommonError> {
        let threshold_factors = threshold_factors.into_iter().collect_vec();
        let override_factors = override_factors.into_iter().collect_vec();

        // validate
        let _ = PrimaryRoleWithFactorInstances::with_factors(
            threshold,
            threshold_factors
                .clone()
                .into_iter()
                .map(FactorInstance::from)
                .collect_vec(),
            override_factors
                .clone()
                .into_iter()
                .map(FactorInstance::from)
                .collect_vec(),
        );

        Ok(Self {
            role,
            threshold,
            threshold_factors,
            override_factors,
        })
    }
}

impl HasRoleKindObjectSafe
    for GeneralRoleWithHierarchicalDeterministicFactorInstances
{
    fn get_role_kind(&self) -> RoleKind {
        self.role
    }
}

impl TryFrom<(MatrixOfFactorInstances, RoleKind)>
    for GeneralRoleWithHierarchicalDeterministicFactorInstances
{
    type Error = CommonError;

    fn try_from(
        (matrix, role_kind): (MatrixOfFactorInstances, RoleKind),
    ) -> Result<Self, CommonError> {
        let threshold_factors: Vec<FactorInstance>;
        let override_factors: Vec<FactorInstance>;
        let threshold: u8;

        match role_kind {
            RoleKind::Primary => {
                let role = matrix.primary();
                threshold = role.get_threshold();
                threshold_factors = role.get_threshold_factors().clone();
                override_factors = role.get_override_factors().clone();
            }
            RoleKind::Recovery => {
                let role = matrix.recovery();
                threshold = role.get_threshold();
                threshold_factors = role.get_threshold_factors().clone();
                override_factors = role.get_override_factors().clone();
            }
            RoleKind::Confirmation => {
                let role = matrix.confirmation();
                threshold = role.get_threshold();
                threshold_factors = role.get_threshold_factors().clone();
                override_factors = role.get_override_factors().clone();
            }
        }

        Self::with_factors_and_role(
            role_kind,
            threshold_factors
                .iter()
                .map(|f| {
                    HierarchicalDeterministicFactorInstance::try_from_factor_instance(f.clone())
                })
                .collect::<Result<Vec<HierarchicalDeterministicFactorInstance>, CommonError>>()?,
            threshold,
            override_factors
                .iter()
                .map(|f| {
                    HierarchicalDeterministicFactorInstance::try_from_factor_instance(f.clone())
                })
                .collect::<Result<Vec<HierarchicalDeterministicFactorInstance>, CommonError>>()?,
        )
    }
}

impl GeneralRoleWithHierarchicalDeterministicFactorInstances {
    pub fn single_override(
        role: RoleKind,
        factor: HierarchicalDeterministicFactorInstance,
    ) -> Self {
        assert!(factor.is_securified(), "non securified factor");
        Self::with_factors_and_role(role, [], 0, [factor])
            .expect("Zero threshold with zero threshold factors and one override should not fail.")
    }

    pub fn single_threshold(
        role: RoleKind,
        factor: HierarchicalDeterministicFactorInstance,
    ) -> Self {
        assert!(factor.is_securified(), "non securified factor");
        Self::with_factors_and_role(role, [factor], 1, []).expect(
            "Single threshold with one threshold factor should not fail.",
        )
    }
}

impl HasSampleValues
    for GeneralRoleWithHierarchicalDeterministicFactorInstances
{
    fn sample() -> Self {
        Self::try_from((MatrixOfFactorInstances::sample(), RoleKind::Primary))
            .expect("Sample should not fail")
    }

    fn sample_other() -> Self {
        Self::try_from((
            MatrixOfFactorInstances::sample_other(),
            RoleKind::Recovery,
        ))
        .expect("Sample should not fail")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = GeneralRoleWithHierarchicalDeterministicFactorInstances;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    fn matrix() -> MatrixOfFactorInstances {
        MatrixOfFactorInstances::sample()
    }

    #[test]
    fn test_from_primary_role() {
        pretty_assertions::assert_eq!(
            SUT::try_from(
                (matrix(), RoleKind::Primary)
            ).unwrap(),
            SUT::with_factors_and_role(
                RoleKind::Primary,
                [
                    HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(0),
                    HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_1_securified_at_index(0)
                    ],
                2,
                []
            ).unwrap()
        )
    }

    #[test]
    fn test_single_threshold() {
        pretty_assertions::assert_eq!(
            SUT::single_threshold(RoleKind::Primary,  HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_1_securified_at_index(0)),
            SUT::with_factors_and_role(
                RoleKind::Primary,
                [
                    HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_1_securified_at_index(0)
                    ],
                1,
                []
            ).unwrap()
        )
    }

    #[test]
    fn test_get_role() {
        let test = |role: RoleKind| {
            let sut = SUT::single_override(
                role,
                HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(0)
            );
            assert_eq!(sut.get_role_kind(), role);
        };
        test(RoleKind::Primary);
        test(RoleKind::Confirmation);
        test(RoleKind::Recovery);
    }

    #[test]
    fn test_from_recovery_role() {
        let m = matrix();
        let r = m.recovery();
        assert_eq!(
            SUT::try_from((matrix(), RoleKind::Recovery)).unwrap(),
            SUT::with_factors_and_role(
                RoleKind::Recovery,
                [],
                0,
                r.get_override_factors()
                    .clone()
                    .into_iter()
                    .map(|f: FactorInstance| {
                        HierarchicalDeterministicFactorInstance::try_from_factor_instance(f)
                            .unwrap()
                    })
                    .collect_vec(),
            )
            .unwrap()
        )
    }

    #[test]
    fn test_from_confirmation_role() {
        let m = matrix();
        let r = m.confirmation();
        assert_eq!(
            SUT::try_from((matrix(), RoleKind::Confirmation)).unwrap(),
            SUT::with_factors_and_role(
                RoleKind::Confirmation,
                [],
                0,
                r.get_override_factors()
                    .clone()
                    .into_iter()
                    .map(|f: FactorInstance| {
                        HierarchicalDeterministicFactorInstance::try_from_factor_instance(f)
                            .unwrap()
                    })
                    .collect_vec(),
            )
            .unwrap()
        )
    }

    #[test]
    fn test_from_matrix_containing_physical_badge() {
        let mut matrix = MatrixOfFactorInstances::sample();
        matrix.primary_role = PrimaryRoleWithFactorInstances::with_factors(
            0,
            [],
            [FactorInstance::sample_other()],
        );

        assert_eq!(
            SUT::try_from((matrix, RoleKind::Primary)),
            Err(CommonError::BadgeIsNotVirtualHierarchicalDeterministic)
        );
    }
}
