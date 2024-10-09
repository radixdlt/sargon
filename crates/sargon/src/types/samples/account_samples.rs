use crate::prelude::*;

impl Account {
    pub(crate) fn sample_unsecurified_mainnet(
        name: impl AsRef<str>,
        genesis_factor_instance: HierarchicalDeterministicFactorInstance,
    ) -> Self {
        Self {
            network_id: NetworkID::Mainnet,
            address: AccountAddress::new(
                genesis_factor_instance.public_key.public_key,
                NetworkID::Mainnet,
            ),
            display_name: DisplayName::new(name).unwrap(),
            security_state:
                UnsecuredEntityControl::with_transaction_signing_only(
                    genesis_factor_instance,
                )
                .unwrap()
                .into(),
            appearance_id: Default::default(),
            flags: Default::default(),
            on_ledger_settings: Default::default(),
        }
    }

    pub(crate) fn sample_securified_mainnet(
        name: impl AsRef<str>,
        address: AccountAddress,
        make_role: impl Fn() -> GeneralRoleWithHierarchicalDeterministicFactorInstances,
    ) -> Self {
        let role = make_role();

        let matrix = MatrixOfFactorInstances::from(role);

        Self {
            network_id: NetworkID::Mainnet,
            address: address,
            display_name: DisplayName::new(name).unwrap(),
            security_state: SecuredEntityControl {
                access_controller_address:
                    AccessControllerAddress::sample_from_account_address(
                        address,
                    ),
                security_structure: SecurityStructureOfFactorInstances {
                    security_structure_id: SecurityStructureID::sample(),
                    matrix_of_factors: matrix,
                },
            }
            .into(),
            appearance_id: Default::default(),
            flags: Default::default(),
            on_ledger_settings: Default::default(),
        }
    }

    /// Alice | 0 | Unsecurified { Device }
    pub(crate) fn a0() -> Self {
        Self::sample_unsecurified_mainnet(
            "Alice",
            HierarchicalDeterministicFactorInstance::sample_fia0(),
        )
    }

    /// Bob | 1 | Unsecurified { Ledger }
    pub(crate) fn a1() -> Self {
        Self::sample_unsecurified_mainnet(
            "Bob",
            HierarchicalDeterministicFactorInstance::sample_fia1(),
        )
    }

    /// Carla | 2 | Securified { Single Threshold only }
    pub(crate) fn a2() -> Self {
        Self::sample_securified_mainnet(
            "Carla",
            AccountAddress::sample_at(2),
            || {
                let idx = HDPathComponent::from(2);
                GeneralRoleWithHierarchicalDeterministicFactorInstances::m2(
                HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                    CAP26EntityKind::Account,
                    idx,
                )
            )
            },
        )
    }

    /// David | 3 | Securified { Single Override only }
    pub(crate) fn a3() -> Self {
        Self::sample_securified_mainnet(
            "David",
            AccountAddress::sample_at(3),
            || {
                let idx = HDPathComponent::from(3);
                GeneralRoleWithHierarchicalDeterministicFactorInstances::m3(
                HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                    CAP26EntityKind::Account,
                    idx,
                )
            )
            },
        )
    }

    /// Emily | 4 | Securified { Threshold factors only #3 }
    pub(crate) fn a4() -> Self {
        Self::sample_securified_mainnet(
            "Emily",
            AccountAddress::sample_at(4),
            || {
                let idx = HDPathComponent::from(4);
                GeneralRoleWithHierarchicalDeterministicFactorInstances::m4(
                HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                    CAP26EntityKind::Account,
                    idx,
                )
            )
            },
        )
    }

    /// Frank | 5 | Securified { Override factors only #2 }
    pub(crate) fn a5() -> Self {
        Self::sample_securified_mainnet(
            "Frank",
            AccountAddress::sample_at(5),
            || {
                let idx = HDPathComponent::from(5);
                GeneralRoleWithHierarchicalDeterministicFactorInstances::m5(
                HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                    CAP26EntityKind::Account,
                    idx,
                )
            )
            },
        )
    }

    /// Grace | 6 | Securified { Threshold #3 and Override factors #2  }
    pub(crate) fn a6() -> Self {
        Self::sample_securified_mainnet(
            "Grace",
            AccountAddress::sample_at(6),
            || {
                let idx = HDPathComponent::from(6);
                GeneralRoleWithHierarchicalDeterministicFactorInstances::m6(
                HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                    CAP26EntityKind::Account,
                    idx,
                )
            )
            },
        )
    }

    /// Ida | 7 | Securified { Threshold only # 5/5 }
    pub(crate) fn a7() -> Self {
        Self::sample_securified_mainnet(
            "Ida",
            AccountAddress::sample_at(7),
            || {
                let idx = HDPathComponent::from(7);
                GeneralRoleWithHierarchicalDeterministicFactorInstances::m7(
                HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                    CAP26EntityKind::Account,
                    idx,
                )
            )
            },
        )
    }

    /// Jenny | 8 | Unsecurified { Device } (fs10)
    pub(crate) fn a8() -> Self {
        Self::sample_unsecurified_mainnet(
            "Jenny",
            HierarchicalDeterministicFactorInstance::sample_fia10(),
        )
    }

    /// Klara | 9 |  Securified { Threshold 1/1 and Override factors #1  }
    pub(crate) fn a9() -> Self {
        Self::sample_securified_mainnet(
            "Klara",
            AccountAddress::sample_at(9),
            || {
                let idx = HDPathComponent::from(9);
                GeneralRoleWithHierarchicalDeterministicFactorInstances::m8(
                HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                    CAP26EntityKind::Account,
                    idx,
                )
            )
            },
        )
    }
}
