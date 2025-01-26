#![cfg(test)]

use crate::prelude::*;

#[allow(clippy::upper_case_acronyms)]
type SUT = MatrixOfFactorInstances;

#[test]
fn wrong_entity_kind() {
    let invalid = unsafe {
        SUT::unbuilt_with_roles_and_days(
            PrimaryRoleWithFactorInstances::unbuilt_with_factors(Threshold::All, [
                HierarchicalDeterministicFactorInstance::sample_mainnet_entity_device_factor_fs_0_securified_at_index(
                CAP26EntityKind::Account,
                0,
            ).into(), HierarchicalDeterministicFactorInstance::sample_mainnet_entity_device_factor_fs_0_securified_at_index(
                CAP26EntityKind::Identity, // <--- Wrong entity kind
                1,
            ).into()], []),
            RecoveryRoleWithFactorInstances::unbuilt_with_factors(
                Threshold::zero(),
                [],
                [],
            ),
            ConfirmationRoleWithFactorInstances::unbuilt_with_factors(
                Threshold::zero(),
                [],
                [],
            ),
            TimePeriod::with_days(1),
        )
    };
    let res = invalid.index_agnostic_path_of_all_tx_signing_factor_instances();
    assert!(matches!(
        res,
        Err(CommonError::WrongEntityKindOfInFactorInstancesPath)
    ));
}

#[test]
fn wrong_key_kind() {
    let invalid = unsafe {
        SUT::unbuilt_with_roles_and_days(
            PrimaryRoleWithFactorInstances::unbuilt_with_factors(Threshold::All, [
                HierarchicalDeterministicFactorInstance::sample_mainnet_entity_device_factor_fs_0_securified_at_index(
                CAP26EntityKind::Account,
                0,
            ).into(),
            HierarchicalDeterministicFactorInstance::sample_with_key_kind_entity_kind_on_network_and_hardened_index(
                NetworkID::Mainnet,
                CAP26KeyKind::AuthenticationSigning, // <-- Wrong key kind
                CAP26EntityKind::Account,
                SecurifiedU30::ZERO
            ).into()], []),
            RecoveryRoleWithFactorInstances::unbuilt_with_factors(
                Threshold::zero(),
                [],
                [],
            ),
            ConfirmationRoleWithFactorInstances::unbuilt_with_factors(
                Threshold::zero(),
                [],
                [],
            ),
            TimePeriod::with_days(1),
        )
    };
    let res = invalid.index_agnostic_path_of_all_tx_signing_factor_instances();
    assert!(matches!(
        res,
        Err(CommonError::WrongKeyKindOfTransactionSigningFactorInstance)
    ));
}
