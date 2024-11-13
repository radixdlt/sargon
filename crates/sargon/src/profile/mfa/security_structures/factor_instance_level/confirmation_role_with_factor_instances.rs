use crate::prelude::*;

impl HasRoleKind for ConfirmationRoleWithFactorInstances {
    fn role_kind() -> RoleKind {
        RoleKind::Confirmation
    }
}

impl HasSampleValues for ConfirmationRoleWithFactorInstances {
    fn sample() -> Self {
        Self::new([HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(27).into()], 1, [HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_securified_at_index(13).into()])
            .unwrap()
    }

    fn sample_other() -> Self {
        Self::new(
            [HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(6).into(), HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_securified_at_index(42).into()],
            2,
            [HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_securified_at_index(19).into()],
        )
        .unwrap()
    }
}
