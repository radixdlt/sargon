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
