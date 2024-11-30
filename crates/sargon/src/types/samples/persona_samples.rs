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
            HierarchicalDeterministicFactorInstance::sample_mainnet_identity_device_factor_fs_10_unsecurified_at_index(2),
            || {
                let idx =
                    Hardened::from_local_key_space(2u32, IsSecurified(true)).unwrap();
                GeneralRoleWithHierarchicalDeterministicFactorInstances::r2(HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                    CAP26EntityKind::Identity,
                    idx,
                ))
            },
        ),
        // Superman | 3 | Securified { Single Override only }
        Persona::sample_securified_mainnet(
            "Superman",
            HierarchicalDeterministicFactorInstance::sample_mainnet_identity_device_factor_fs_10_unsecurified_at_index(3),
            || {
                let idx =
                    Hardened::from_local_key_space(3u32, IsSecurified(true)).unwrap();
                GeneralRoleWithHierarchicalDeterministicFactorInstances::r3(HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                    CAP26EntityKind::Identity,
                    idx,
                ))
            },
        ),
        // Banksy | 4 | Securified { Threshold factors only #3 }
        Persona::sample_securified_mainnet(
            "Banksy",
            HierarchicalDeterministicFactorInstance::sample_mainnet_identity_device_factor_fs_10_unsecurified_at_index(4),
            || {
                let idx =
                    Hardened::from_local_key_space(4u32, IsSecurified(true)).unwrap();
                GeneralRoleWithHierarchicalDeterministicFactorInstances::r4(HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                    CAP26EntityKind::Identity,
                    idx,
                ))
            },
        ),
        // Voltaire | 5 | Securified { Override factors only #2 }
        Persona::sample_securified_mainnet(
            "Voltaire",
            HierarchicalDeterministicFactorInstance::sample_mainnet_identity_device_factor_fs_10_unsecurified_at_index(5),
            || {
                let idx =
                    Hardened::from_local_key_space(5u32, IsSecurified(true)).unwrap();
                GeneralRoleWithHierarchicalDeterministicFactorInstances::r5(HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                    CAP26EntityKind::Identity,
                    idx,
                ))
            },
        ),
        // Kasparov | 6 | Securified { Threshold #3 and Override factors #2  }
        Persona::sample_securified_mainnet(
            "Kasparov",
            HierarchicalDeterministicFactorInstance::sample_mainnet_identity_device_factor_fs_10_unsecurified_at_index(6),
            || {
                let idx =
                    Hardened::from_local_key_space(6u32, IsSecurified(true)).unwrap();
                GeneralRoleWithHierarchicalDeterministicFactorInstances::r6(HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                    CAP26EntityKind::Identity,
                    idx,
                ))
            },
        ),
        // Pelé | 7 | Securified { Threshold only # 5/5 }
        Persona::sample_securified_mainnet(
            "Pelé",
            HierarchicalDeterministicFactorInstance::sample_mainnet_identity_device_factor_fs_10_unsecurified_at_index(7),
            || {
                let idx =
                    Hardened::from_local_key_space(7u32, IsSecurified(true)).unwrap();
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
        veci: HierarchicalDeterministicFactorInstance,
        make_role: impl Fn() -> GeneralRoleWithHierarchicalDeterministicFactorInstances,
    ) -> Self {
        /*
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
        )
        .unwrap();
        let address =
            IdentityAddress::new(veci.public_key(), NetworkID::Mainnet);
        Self {
            network_id: NetworkID::Mainnet,
            address,
            display_name: DisplayName::new(name).unwrap(),
            security_state: SecuredEntityControl::new(
                veci.clone(),
                AccessControllerAddress::sample_from_identity_address(address),
                SecurityStructureOfFactorInstances {
                    security_structure_id: SecurityStructureID::sample(),
                    matrix_of_factors: matrix,
                },
            )
            .unwrap()
            .into(),
            flags: Default::default(),
            persona_data: Default::default(),
        }
        */
        unimplemented!("migrate MFA-Role Rules")
    }

    pub fn sample_at(index: usize) -> Self {
        ALL_PERSONA_SAMPLES[index].clone()
    }

    pub fn sample_all() -> Vec<Self> {
        ALL_PERSONA_SAMPLES.to_vec()
    }
}
