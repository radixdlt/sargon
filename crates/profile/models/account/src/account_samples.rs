use crate::prelude::*;
use once_cell::sync::Lazy;

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
            2,
            HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_unsecurified_at_index(2),
            || {
                let idx =
                    Hardened::from_local_key_space(2u32, IsSecurified(true)).unwrap();
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
            3,
            HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_unsecurified_at_index(3),
            || {
                let idx =
                Hardened::from_local_key_space(3u32, IsSecurified(true)).unwrap();
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
            4,
            HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_unsecurified_at_index(4),
            || {
                let idx =
                Hardened::from_local_key_space(4u32, IsSecurified(true)).unwrap();
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
            5,
            HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_unsecurified_at_index(5),
            || {
                let idx =
                Hardened::from_local_key_space(5u32, IsSecurified(true)).unwrap();
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
            6,
            HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_unsecurified_at_index(6),
            || {
                let idx =
                Hardened::from_local_key_space(6u32, IsSecurified(true)).unwrap();
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
            7,
            HierarchicalDeterministicFactorInstance::sample_fia11(),
            || {
                let idx =
                    Hardened::from_local_key_space(7u32, IsSecurified(true)).unwrap();
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
            9,
            HierarchicalDeterministicFactorInstance::sample_fia12(),
            || {
                let idx =
                Hardened::from_local_key_space(9u32, IsSecurified(true)).unwrap();
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
        Self::with(
            NetworkID::Mainnet,
            AccountAddress::new_from_public_key(
                genesis_factor_instance.public_key.public_key,
                NetworkID::Mainnet,
            ),
            DisplayName::new(name).unwrap(),
            UnsecuredEntityControl::with_transaction_signing_only(
                genesis_factor_instance,
            )
            .unwrap(),
            EntityFlags::default(),
            AppearanceID::default(),
            OnLedgerSettings::default(),
        )
    }

    pub fn sample_securified_mainnet(
        name: impl AsRef<str>,
        rola_index: u32,
        veci: HierarchicalDeterministicFactorInstance,
        make_role: impl Fn() -> GeneralRoleWithHierarchicalDeterministicFactorInstances,
    ) -> Self {
        let role = make_role();
        assert_eq!(role.get_role_kind(), RoleKind::Primary, "If this tests fails you can update the code below to not be hardcoded to set the primary role...");
        let matrix = unsafe {
            // This is a completely super weak shield.
            let threshold_factors = role
                .get_threshold_factors()
                .into_iter()
                .map(FactorInstance::from)
                .collect_vec();
            let mut recovery_and_confirmation_override =
                threshold_factors.clone();

            let primary_override_factors = role
                .get_override_factors()
                .into_iter()
                .map(FactorInstance::from)
                .collect_vec();
            if recovery_and_confirmation_override.is_empty() {
                recovery_and_confirmation_override =
                    primary_override_factors.clone();
            }

            MatrixOfFactorInstances::unbuilt_with_roles_and_days(
                PrimaryRoleWithFactorInstances::with_factors(
                    role.get_threshold(),
                    threshold_factors,
                    primary_override_factors,
                ),
                RecoveryRoleWithFactorInstances::with_factors(
                    0,
                    [],
                    recovery_and_confirmation_override.clone(),
                ),
                ConfirmationRoleWithFactorInstances::with_factors(
                    0,
                    [],
                    recovery_and_confirmation_override.clone(),
                ),
                TimePeriod::with_days(237),
            )
        };

        let network_id = NetworkID::Mainnet;
        let address = AccountAddress::new_from_public_key(
            veci.public_key(),
            NetworkID::Mainnet,
        );

        let security_structure_of_factor_instances =
            SecurityStructureOfFactorInstances::new(
                SecurityStructureID::sample(),
                matrix,
                HierarchicalDeterministicFactorInstance::sample_with_key_kind_entity_kind_on_network_and_hardened_index(
                    NetworkID::Mainnet,
                    CAP26KeyKind::AuthenticationSigning,
                    CAP26EntityKind::Account,
                    SecurifiedU30::try_from(rola_index).unwrap(),
                ),
            )
            .unwrap();

        Self::with(
            network_id,
            address,
            DisplayName::new(name).unwrap(),
            SecuredEntityControl::new(
                Some(veci.clone()),
                AccessControllerAddress::sample_from_account_address(address),
                security_structure_of_factor_instances,
            )
            .unwrap(),
            EntityFlags::default(),
            AppearanceID::default(),
            OnLedgerSettings::default(),
        )
    }

    pub fn sample_at(index: usize) -> Self {
        ALL_ACCOUNT_SAMPLES[index].clone()
    }

    pub fn sample_all() -> Vec<Account> {
        ALL_ACCOUNT_SAMPLES.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_addresses() {
        let accounts_addresses = Account::sample_all()
            .into_iter()
            .map(|a| a.address)
            .collect::<Vec<_>>();
        assert_eq!(
            accounts_addresses.len(),
            HashSet::<AccountAddress>::from_iter(accounts_addresses.clone())
                .len()
        );
    }
}
