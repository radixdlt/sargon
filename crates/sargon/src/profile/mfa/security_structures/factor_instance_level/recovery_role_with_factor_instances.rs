use crate::prelude::*;

impl HasRoleKind for RecoveryRoleWithFactorInstances {
    fn role_kind() -> RoleKind {
        RoleKind::Recovery
    }
}

impl HasFactorInstances for RecoveryRoleWithFactorInstances {
    fn unique_factor_instances(&self) -> IndexSet<FactorInstance> {
        self.unique_factors()
    }
}

impl HasSampleValues for RecoveryRoleWithFactorInstances {
    fn sample() -> Self {
        Self::new([HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(54).into()], 1, [HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_securified_at_index(237).into()])
            .unwrap()
    }

    fn sample_other() -> Self {
        Self::new(
            [HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_securified_at_index(65).into(),
            HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_1_securified_at_index(25).into()],
        2,
            [HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_securified_at_index(31).into()],
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RecoveryRoleWithFactorInstances;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
