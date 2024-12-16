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

impl DerivationPath {
    /// # Safety
    /// Crashes for Bip44LikePath, this is only meant to be used for tests
    /// to map between IdentityPath -> IdentityPath
    unsafe fn as_persona(&self) -> Self {
        match self {
            Self::Account { value } => {
                IdentityPath::new(value.network_id, value.key_kind, value.index)
                    .into()
            }
            Self::Identity { value: _ } => self.clone(),
            Self::Bip44Like { value: _ } => panic!("unsupported"),
        }
    }
}

impl HierarchicalDeterministicFactorInstance {
    /// # Safety
    /// Completely unsafe, this is an invalid FactorInstance! It hardcodes
    /// the derivation path as a persona, resulting in an invalid (DerivationPath, PublicKey) pair.!
    unsafe fn invalid_hard_coding_derivation_path_as_persona(&self) -> Self {
        unsafe {
            Self::new(
                self.factor_source_id(),
                HierarchicalDeterministicPublicKey::new(
                    self.public_key(),
                    self.derivation_path().as_persona(),
                ),
            )
        }
    }
}

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
        assert_eq!(veci.get_entity_kind(), CAP26EntityKind::Identity);
        let role = make_role();
        assert_eq!(role.get_role_kind(), RoleKind::Primary, "If this tests fails you can update the code below to not be hardcoded to set the primary role...");
        let mut matrix = MatrixOfFactorInstances::sample();
        matrix.primary_role = PrimaryRoleWithFactorInstances::with_factors(
            role.get_threshold(),
            role.get_threshold_factors()
                .into_iter()
                .map(FactorInstance::from)
                .collect_vec(),
            role.get_override_factors()
                .into_iter()
                .map(FactorInstance::from)
                .collect_vec(),
        );
        unsafe {
            matrix.recovery_role =
                RecoveryRoleWithFactorInstances::with_factors(
                    0,
                    [],
                    matrix
                        .recovery()
                        .get_override_factors()
                        .iter()
                        .filter_map(|f| f.try_as_hd_factor_instances().ok())
                        .map(|f| {
                            f.invalid_hard_coding_derivation_path_as_persona()
                        })
                        .map(FactorInstance::from)
                        .collect_vec(),
                );
            matrix.confirmation_role =
                ConfirmationRoleWithFactorInstances::with_factors(
                    0,
                    [],
                    matrix
                        .confirmation()
                        .get_override_factors()
                        .iter()
                        .filter_map(|f| f.try_as_hd_factor_instances().ok())
                        .map(|f| {
                            f.invalid_hard_coding_derivation_path_as_persona()
                        })
                        .map(FactorInstance::from)
                        .collect_vec(),
                );
        }
        let address =
            IdentityAddress::new(veci.public_key(), NetworkID::Mainnet);

        let rola_index = u32::from(
            veci.derivation_entity_index().index_in_local_key_space(),
        );

        let security_structure_of_factor_instances =
            SecurityStructureOfFactorInstances::new(
                SecurityStructureID::sample(),
                matrix,
                HierarchicalDeterministicFactorInstance::sample_with_key_kind_entity_kind_on_network_and_hardened_index(
                    NetworkID::Mainnet,
                    CAP26KeyKind::AuthenticationSigning,
                    CAP26EntityKind::Identity,
                    SecurifiedU30::try_from(rola_index).unwrap(),
                ),
            )
            .unwrap();

        Self {
            network_id: NetworkID::Mainnet,
            address,
            display_name: DisplayName::new(name).unwrap(),
            security_state: SecuredEntityControl::new(
                veci.clone(),
                AccessControllerAddress::sample_from_identity_address(address),
                security_structure_of_factor_instances,
            )
            .unwrap()
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
