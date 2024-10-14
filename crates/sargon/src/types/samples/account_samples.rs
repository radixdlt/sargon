use crate::prelude::*;

static ALL_ACCOUNT_SAMPLES: Lazy<[Account; 10]> = Lazy::new(|| {
    [
        // Alice | 0 | Unsecurified { Device }
        Account::sample_unsecurified_mainnet(
            "Alice",
            HierarchicalDeterministicFactorInstance::sample_fia0(),
        ),
        // Bob | 1 | Unsecurified { Ledger }
        Account::sample_unsecurified_mainnet(
            "Bob",
            HierarchicalDeterministicFactorInstance::sample_fia1(),
        ),
        // Carla | 2 | Securified { Single Threshold only }
        Account::sample_securified_mainnet(
            "Carla",
            AccountAddress::random(NetworkID::Mainnet),
            || {
                let idx = HDPathComponent::from(2);
                GeneralRoleWithHierarchicalDeterministicFactorInstances::r2(
                    HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                        CAP26EntityKind::Account,
                        idx,
                    )
                )
            },
        ),
        // David | 3 | Securified { Single Override only }
        Account::sample_securified_mainnet(
            "David",
            AccountAddress::random(NetworkID::Mainnet),
            || {
                let idx = HDPathComponent::from(3);
                GeneralRoleWithHierarchicalDeterministicFactorInstances::r3(
                    HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                        CAP26EntityKind::Account,
                        idx,
                    )
                )
            },
        ),
        // Emily | 4 | Securified { Threshold factors only #3 }
        Account::sample_securified_mainnet(
            "Emily",
            AccountAddress::random(NetworkID::Mainnet),
            || {
                let idx = HDPathComponent::from(4);
                GeneralRoleWithHierarchicalDeterministicFactorInstances::r4(
                    HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                        CAP26EntityKind::Account,
                        idx,
                    )
                )
            },
        ),
        // Frank | 5 | Securified { Override factors only #2 }
        Account::sample_securified_mainnet(
            "Frank",
            AccountAddress::random(NetworkID::Mainnet),
            || {
                let idx = HDPathComponent::from(5);
                GeneralRoleWithHierarchicalDeterministicFactorInstances::r5(
                    HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                        CAP26EntityKind::Account,
                        idx,
                    )
                )
            },
        ),
        // Grace | 6 | Securified { Threshold #3 and Override factors #2  }
        Account::sample_securified_mainnet(
            "Grace",
            AccountAddress::random(NetworkID::Mainnet),
            || {
                let idx = HDPathComponent::from(6);
                GeneralRoleWithHierarchicalDeterministicFactorInstances::r6(
                    HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                        CAP26EntityKind::Account,
                        idx,
                    )
                )
            },
        ),
        // Ida | 7 | Securified { Threshold only # 5/5 }
        Account::sample_securified_mainnet(
            "Ida",
            AccountAddress::random(NetworkID::Mainnet),
            || {
                let idx = HDPathComponent::from(7);
                GeneralRoleWithHierarchicalDeterministicFactorInstances::r7(
                    HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                        CAP26EntityKind::Account,
                        idx,
                    )
                )
            },
        ),
        // Jenny | 8 | Unsecurified { Device } (fs10)
        Account::sample_unsecurified_mainnet(
            "Jenny",
            HierarchicalDeterministicFactorInstance::sample_fia10(),
        ),
        // Klara | 9 |  Securified { Threshold 1/1 and Override factors #1  }
        Account::sample_securified_mainnet(
            "Klara",
            AccountAddress::random(NetworkID::Mainnet),
            || {
                let idx = HDPathComponent::from(9);
                GeneralRoleWithHierarchicalDeterministicFactorInstances::r8(
                    HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                        CAP26EntityKind::Account,
                        idx,
                    )
                )
            },
        ),
    ]
});

impl Account {
    pub fn sample_unsecurified_mainnet(
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

    pub fn sample_securified_mainnet(
        name: impl AsRef<str>,
        address: AccountAddress,
        make_role: impl Fn() -> GeneralRoleWithHierarchicalDeterministicFactorInstances,
    ) -> Self {
        let role = make_role();

        let threshold_factors = role
            .threshold_factors
            .iter()
            .map(|hd| hd.factor_instance())
            .collect::<Vec<FactorInstance>>();

        let override_factors = role
            .override_factors
            .iter()
            .map(|hd| hd.factor_instance())
            .collect::<Vec<FactorInstance>>();

        let matrix = MatrixOfFactorInstances::new(
            PrimaryRoleWithFactorInstances::new(
                threshold_factors.clone(),
                role.threshold,
                override_factors.clone(),
            )
            .unwrap(),
            RecoveryRoleWithFactorInstances::new(
                threshold_factors.clone(),
                role.threshold,
                override_factors.clone(),
            )
            .unwrap(),
            ConfirmationRoleWithFactorInstances::new(
                threshold_factors.clone(),
                role.threshold,
                override_factors.clone(),
            )
            .unwrap(),
        );

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

    pub fn sample_at(index: usize) -> Self {
        ALL_ACCOUNT_SAMPLES[index].clone()
    }

    pub fn sample_all() -> Vec<Account> {
        ALL_ACCOUNT_SAMPLES.to_vec()
    }
}
