use crate::prelude::*;

impl HierarchicalDeterministicFactorInstance {
    pub fn sample_id_to_instance(
        entity_kind: CAP26EntityKind,
        index: Hardened,
    ) -> impl Fn(FactorSourceIDFromHash) -> Self {
        move |id: FactorSourceIDFromHash| {
            Self::new_for_entity(id, entity_kind, index)
        }
    }

    pub fn sample_mainnet_tx_account(
        index: Hardened,
        factor_source_id: FactorSourceIDFromHash,
    ) -> Self {
        Self::new_for_entity(factor_source_id, CAP26EntityKind::Account, index)
    }

    pub fn sample_mainnet_tx_identity(
        index: Hardened,
        factor_source_id: FactorSourceIDFromHash,
    ) -> Self {
        Self::new_for_entity(factor_source_id, CAP26EntityKind::Identity, index)
    }

    /// 0 | unsecurified | device
    pub fn sample_fi0(entity_kind: CAP26EntityKind) -> Self {
        Self::new_for_entity(
            FactorSourceIDFromHash::sample_at(0),
            entity_kind,
            Hardened::from_local_key_space_unsecurified(0u32).unwrap(),
        )
    }

    /// Account: 0 | unsecurified | device
    pub fn sample_fia0() -> Self {
        Self::sample_fi0(CAP26EntityKind::Account)
    }
    /// Identity: 0 | unsecurified | device
    pub fn sample_fii0() -> Self {
        Self::sample_fi0(CAP26EntityKind::Identity)
    }

    /// 1 | unsecurified | ledger
    pub fn sample_fi1(entity_kind: CAP26EntityKind) -> Self {
        Self::new_for_entity(
            FactorSourceIDFromHash::sample_at(1),
            entity_kind,
            Hardened::from_local_key_space_unsecurified(1u32).unwrap(),
        )
    }

    /// Account: 1 | unsecurified | ledger
    pub fn sample_fia1() -> Self {
        Self::sample_fi1(CAP26EntityKind::Account)
    }
    /// Identity: 1 | unsecurified | ledger
    pub fn sample_fii1() -> Self {
        Self::sample_fi1(CAP26EntityKind::Identity)
    }

    /// 8 | Unsecurified { Device } (fs10)
    pub fn sample_fi10(entity_kind: CAP26EntityKind) -> Self {
        Self::new_for_entity(
            FactorSourceIDFromHash::sample_at(10),
            entity_kind,
            Hardened::from_local_key_space_unsecurified(8u32).unwrap(),
        )
    }

    /// Account: 8 | Unsecurified { Device } (fs10)
    pub fn sample_fia10() -> Self {
        Self::sample_fi10(CAP26EntityKind::Account)
    }

    /// 9 | Unsecurified { Device } (fs10)
    pub fn sample_fi11(entity_kind: CAP26EntityKind) -> Self {
        Self::new_for_entity(
            FactorSourceIDFromHash::sample_at(10),
            entity_kind,
            Hardened::from_local_key_space_unsecurified(9u32).unwrap(),
        )
    }

    /// Account: 9 | Unsecurified { Device } (fs10)
    pub fn sample_fia11() -> Self {
        Self::sample_fi11(CAP26EntityKind::Account)
    }

    /// Identity: 8 | Unsecurified { Device } (fs10)
    pub fn sample_fii10() -> Self {
        Self::sample_fi10(CAP26EntityKind::Identity)
    }

    pub fn sample_mainnet_entity_device_factor_fs_0_securified_at_index(
        entity_kind: CAP26EntityKind,
        index: u32,
    ) -> Self {
        Self::new_for_entity(
            FactorSourceIDFromHash::sample_at(0),
            entity_kind,
            Hardened::from_local_key_space(index, IsSecurified(true)).unwrap(),
        )
    }

    pub fn sample_mainnet_entity_device_factor_fs_10_securified_at_index(
        entity_kind: CAP26EntityKind,
        index: u32,
    ) -> Self {
        Self::new_for_entity(
            FactorSourceIDFromHash::sample_at(10),
            entity_kind,
            Hardened::from_local_key_space(index, IsSecurified(true)).unwrap(),
        )
    }

    pub fn sample_mainnet_entity_device_factor_fs_1_securified_at_index(
        entity_kind: CAP26EntityKind,
        index: u32,
    ) -> Self {
        Self::new_for_entity(
            FactorSourceIDFromHash::sample_at(1),
            entity_kind,
            Hardened::from_local_key_space(index, IsSecurified(true)).unwrap(),
        )
    }

    pub fn sample_mainnet_account_device_factor_fs_0_securified_at_index(
        index: u32,
    ) -> Self {
        Self::sample_mainnet_entity_device_factor_fs_0_securified_at_index(
            CAP26EntityKind::Account,
            index,
        )
    }

    pub fn sample_mainnet_account_device_factor_fs_10_securified_at_index(
        index: u32,
    ) -> Self {
        Self::sample_mainnet_entity_device_factor_fs_10_securified_at_index(
            CAP26EntityKind::Account,
            index,
        )
    }

    pub fn sample_mainnet_account_device_factor_fs_1_securified_at_index(
        index: u32,
    ) -> Self {
        Self::sample_mainnet_entity_device_factor_fs_1_securified_at_index(
            CAP26EntityKind::Account,
            index,
        )
    }

    pub fn sample_mainnet_entity_device_factor_fs_10_unsecurified_at_index(
        entity_kind: CAP26EntityKind,
        index: u32,
    ) -> Self {
        Self::new_for_entity(
            FactorSourceIDFromHash::sample_at(10),
            entity_kind,
            Hardened::from_local_key_space_unsecurified(index).unwrap(),
        )
    }

    pub fn sample_mainnet_account_device_factor_fs_10_unsecurified_at_index(
        index: u32,
    ) -> Self {
        Self::sample_mainnet_entity_device_factor_fs_10_unsecurified_at_index(
            CAP26EntityKind::Account,
            index,
        )
    }

    pub fn sample_mainnet_identity_device_factor_fs_10_unsecurified_at_index(
        index: u32,
    ) -> Self {
        Self::sample_mainnet_entity_device_factor_fs_10_unsecurified_at_index(
            CAP26EntityKind::Identity,
            index,
        )
    }
}
