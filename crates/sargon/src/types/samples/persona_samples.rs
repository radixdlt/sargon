use crate::prelude::*;

static ALL_PERSONA_SAMPLES: Lazy<[Persona; 8]> = Lazy::new(|| {
    [
        // Satoshi | 0 | Unsecurified { Device }
        Persona::sample_unsecurified_mainnet(
            "Satoshi",
            HierarchicalDeterministicFactorInstance::sample_fii0(),
        ),
        // Batman | 1 | Unsecurified { Ledger }
        Persona::sample_unsecurified_mainnet(
            "Batman",
            HierarchicalDeterministicFactorInstance::sample_fii1(),
        ),
        // Ziggy | 2 | Securified { Single Threshold only }
        Persona::sample_securified_mainnet(
            "Ziggy",
            IdentityAddress::random(NetworkID::Mainnet),
            || {
                let idx = HDPathComponent::from(2);
                GeneralRoleWithHierarchicalDeterministicFactorInstances::r2(HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                    CAP26EntityKind::Identity,
                    idx,
                ))
            },
        ),
        // Superman | 3 | Securified { Single Override only }
        Persona::sample_securified_mainnet(
            "Superman",
            IdentityAddress::random(NetworkID::Mainnet),
            || {
                let idx = HDPathComponent::from(3);
                GeneralRoleWithHierarchicalDeterministicFactorInstances::r3(HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                    CAP26EntityKind::Identity,
                    idx,
                ))
            },
        ),
        // Banksy | 4 | Securified { Threshold factors only #3 }
        Persona::sample_securified_mainnet(
            "Banksy",
            IdentityAddress::random(NetworkID::Mainnet),
            || {
                let idx = HDPathComponent::from(4);
                GeneralRoleWithHierarchicalDeterministicFactorInstances::r4(HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                    CAP26EntityKind::Identity,
                    idx,
                ))
            },
        ),
        // Voltaire | 5 | Securified { Override factors only #2 }
        Persona::sample_securified_mainnet(
            "Voltaire",
            IdentityAddress::random(NetworkID::Mainnet),
            || {
                let idx = HDPathComponent::from(5);
                GeneralRoleWithHierarchicalDeterministicFactorInstances::r5(HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                    CAP26EntityKind::Identity,
                    idx,
                ))
            },
        ),
        // Kasparov | 6 | Securified { Threshold #3 and Override factors #2  }
        Persona::sample_securified_mainnet(
            "Kasparov",
            IdentityAddress::random(NetworkID::Mainnet),
            || {
                let idx = HDPathComponent::from(6);
                GeneralRoleWithHierarchicalDeterministicFactorInstances::r6(HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                    CAP26EntityKind::Identity,
                    idx,
                ))
            },
        ),
        // Pelé | 7 | Securified { Threshold only # 5/5 }
        Persona::sample_securified_mainnet(
            "Pelé",
            IdentityAddress::random(NetworkID::Mainnet),
            || {
                let idx = HDPathComponent::from(7);
                GeneralRoleWithHierarchicalDeterministicFactorInstances::r7(HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                    CAP26EntityKind::Identity,
                    idx,
                ))
            },
        ),
    ]
});

impl Persona {
    pub fn sample_unsecurified_mainnet(
        name: impl AsRef<str>,
        genesis_factor_instance: HierarchicalDeterministicFactorInstance,
    ) -> Self {
        Self {
            network_id: NetworkID::Mainnet,
            address: IdentityAddress::new(
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
            flags: Default::default(),
            persona_data: Default::default(),
        }
    }

    pub fn sample_securified_mainnet(
        name: impl AsRef<str>,
        address: IdentityAddress,
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
            ),
            RecoveryRoleWithFactorInstances::new(
                threshold_factors.clone(),
                role.threshold,
                override_factors.clone(),
            ),
            ConfirmationRoleWithFactorInstances::new(
                threshold_factors.clone(),
                role.threshold,
                override_factors.clone(),
            ),
        );

        Self {
            network_id: NetworkID::Mainnet,
            address: address,
            display_name: DisplayName::new(name).unwrap(),
            security_state: SecuredEntityControl {
                access_controller_address:
                    AccessControllerAddress::sample_from_identity_address(
                        address,
                    ),
                security_structure: SecurityStructureOfFactorInstances {
                    security_structure_id: SecurityStructureID::sample(),
                    matrix_of_factors: matrix,
                },
            }
            .into(),
            flags: Default::default(),
            persona_data: Default::default(),
        }
    }

    pub fn sample_at(index: usize) -> Self {
        ALL_PERSONA_SAMPLES[index].clone()
    }

    pub fn sample_all() -> Vec<Self> {
        ALL_PERSONA_SAMPLES.to_vec()
    }
}
