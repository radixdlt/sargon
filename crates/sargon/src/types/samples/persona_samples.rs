use crate::prelude::*;

impl Persona {
    pub(crate) fn sample_unsecurified_mainnet(
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

    pub(crate) fn sample_securified_mainnet(
        name: impl AsRef<str>,
        address: IdentityAddress,
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

    /// Satoshi | 0 | Unsecurified { Device }
    pub(crate) fn p0() -> Self {
        Self::sample_unsecurified_mainnet(
            "Satoshi",
            HierarchicalDeterministicFactorInstance::sample_fii0(),
        )
    }

    /// Batman | 1 | Unsecurified { Ledger }
    pub(crate) fn p1() -> Self {
        Self::sample_unsecurified_mainnet(
            "Batman",
            HierarchicalDeterministicFactorInstance::sample_fii1(),
        )
    }

    /// Ziggy | 2 | Securified { Single Threshold only }
    pub(crate) fn p2() -> Self {
        Self::sample_securified_mainnet(
            "Ziggy",
            IdentityAddress::sample_at(2),
            || {
                let idx = HDPathComponent::from(2);
                GeneralRoleWithHierarchicalDeterministicFactorInstances::m2(HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                CAP26EntityKind::Identity,
                idx,
            ))
            },
        )
    }

    /// Superman | 3 | Securified { Single Override only }
    pub(crate) fn p3() -> Self {
        Self::sample_securified_mainnet(
            "Superman",
            IdentityAddress::sample_at(3),
            || {
                let idx = HDPathComponent::from(3);
                GeneralRoleWithHierarchicalDeterministicFactorInstances::m3(HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                CAP26EntityKind::Identity,
                idx,
            ))
            },
        )
    }

    /// Banksy | 4 | Securified { Threshold factors only #3 }
    pub(crate) fn p4() -> Self {
        Self::sample_securified_mainnet(
            "Banksy",
            IdentityAddress::sample_at(4),
            || {
                let idx = HDPathComponent::from(4);
                GeneralRoleWithHierarchicalDeterministicFactorInstances::m4(HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                CAP26EntityKind::Identity,
                idx,
            ))
            },
        )
    }

    /// Voltaire | 5 | Securified { Override factors only #2 }
    pub(crate) fn p5() -> Self {
        Self::sample_securified_mainnet(
            "Voltaire",
            IdentityAddress::sample_at(5),
            || {
                let idx = HDPathComponent::from(5);
                GeneralRoleWithHierarchicalDeterministicFactorInstances::m5(HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                CAP26EntityKind::Identity,
                idx,
            ))
            },
        )
    }

    /// Kasparov | 6 | Securified { Threshold #3 and Override factors #2  }
    pub(crate) fn p6() -> Self {
        Self::sample_securified_mainnet(
            "Kasparov",
            IdentityAddress::sample_at(6),
            || {
                let idx = HDPathComponent::from(6);
                GeneralRoleWithHierarchicalDeterministicFactorInstances::m6(HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                CAP26EntityKind::Identity,
                idx,
            ))
            },
        )
    }

    /// Pelé | 7 | Securified { Threshold only # 5/5 }
    pub(crate) fn p7() -> Self {
        Self::sample_securified_mainnet(
            "Pelé",
            IdentityAddress::sample_at(7),
            || {
                let idx = HDPathComponent::from(7);
                GeneralRoleWithHierarchicalDeterministicFactorInstances::m7(HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                CAP26EntityKind::Identity,
                idx,
            ))
            },
        )
    }
}
