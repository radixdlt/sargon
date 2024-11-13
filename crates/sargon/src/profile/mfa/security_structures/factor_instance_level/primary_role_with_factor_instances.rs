use crate::prelude::*;

impl HasRoleKind for PrimaryRoleWithFactorInstances {
    fn role_kind() -> RoleKind {
        RoleKind::Primary
    }
}

impl HasSampleValues for PrimaryRoleWithFactorInstances {
    fn sample() -> Self {
        Self::new([HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(0).into()], 1, [HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_securified_at_index(0).into()])
            .unwrap()
    }

    fn sample_other() -> Self {
        Self::new(
            [HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(1).into(),
            HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_securified_at_index(11).into(),
            HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_securified_at_index(12).into()],
            2,
            [HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_securified_at_index(6).into()],
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PrimaryRoleWithFactorInstances;

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
    fn primary_role_non_securified_threshold_instances_is_err() {
        assert!(matches!(
            SUT::threshold_factors_only(
                [
                    HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_unsecurified_at_index(0).into()
                ],
                1,
            ),
            Err(CommonError::IndexUnsecurifiedExpectedSecurified)
        ));
    }
}
