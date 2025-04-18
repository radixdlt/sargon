use crate::prelude::*;

/// An analyzer of a `Profile` for some `network_id` (i.e. analyzer of `ProfileNetwork`),
/// reading out the max derivation entity index for Unsecurified/Securified Accounts/Personas
/// for some factor source id.
#[derive(derive_more::Deref)]
pub struct NextDerivationEntityIndexProfileAnalyzingAssigner {
    entities_on_network: EntitiesOnNetwork,
}

impl NextDerivationEntityIndexProfileAnalyzingAssigner {
    /// `Profile` is optional so that one can use the same initializer from `FactorInstancesProvider`,
    /// which accepts an optional Profile. Will just default to empty lists if `None` is passed,
    /// effectively making this whole assigner NOOP.
    pub fn new(
        network_id: NetworkID,
        profile: impl Into<Option<Arc<Profile>>>,
    ) -> Self {
        let profile = profile.into();
        let unsecurified_accounts_on_network = profile
            .as_ref()
            .map(|p| p.unsecurified_accounts_on_network(network_id))
            .unwrap_or_default();

        let securified_accounts_on_network = profile
            .as_ref()
            .map(|p| p.securified_accounts_on_network(network_id))
            .unwrap_or_default();

        let unsecurified_personas_on_network = profile
            .as_ref()
            .map(|p| p.unsecurified_personas_on_network(network_id))
            .unwrap_or_default();

        let securified_personas_on_network = profile
            .as_ref()
            .map(|p| p.securified_personas_on_network(network_id))
            .unwrap_or_default();

        let entities_on_network = EntitiesOnNetwork::with_split(
            network_id,
            unsecurified_accounts_on_network,
            securified_accounts_on_network,
            unsecurified_personas_on_network,
            securified_personas_on_network,
        )
        .expect("Should have only queried entities on the correct network");

        Self {
            entities_on_network,
        }
    }

    fn max_entity_veci(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        entities: impl IntoIterator<Item = AnyUnsecurifiedEntity>,
        securified_entities: impl IntoIterator<Item = AnySecurifiedEntity>,
        entity_kind: CAP26EntityKind,
        key_space: KeySpace,
    ) -> Option<HDPathComponent> {
        let max_veci = |vecis: IndexSet<VirtualEntityCreatingInstance>| -> Option<HDPathComponent> {
            vecis
                .into_iter()
                .map(|x| x.factor_instance())
                .filter(|f| f.factor_source_id == factor_source_id)
                .map(|f| f.derivation_path())
                .map(|p| {
                    AssertMatches {
                        network_id: self.network_id,
                        key_kind: CAP26KeyKind::TransactionSigning,
                        entity_kind,
                        key_space,
                    }
                    .matches(&p)
                })
                .map(|p| p.index())
                .max()
        };

        let of_unsecurified =
            max_veci(entities.into_iter().map(|x| x.veci()).collect());

        print!("{:?}", of_unsecurified);
        // The securified entities might have been originally created - having a veci -
        // with the same factor source id.
        let of_securified = max_veci(
            securified_entities
                .into_iter()
                .filter_map(|x| x.veci())
                .collect::<IndexSet<VirtualEntityCreatingInstance>>(),
        );

        std::cmp::max(of_unsecurified, of_securified)
    }

    /// Returns the Max Derivation Entity Index of Unsecurified Accounts controlled
    /// by `factor_source_id`, or `None` if no unsecurified account controlled by that
    /// factor source id found.
    fn max_account_veci(
        &self,
        factor_source_id: FactorSourceIDFromHash,
    ) -> Option<HDPathComponent> {
        self.max_entity_veci(
            factor_source_id,
            self.unsecurified_accounts_on_network
                .clone()
                .iter()
                .map(Into::<AnyUnsecurifiedEntity>::into)
                .collect::<IdentifiedVecOf<_>>(),
            self.securified_accounts_on_network
                .clone()
                .into_iter()
                .map(|x| x.erase_to_any()),
            CAP26EntityKind::Account,
            KeySpace::Unsecurified { is_hardened: true },
        )
    }

    /// Returns the Max Derivation Entity Index of Unsecurified Personas controlled
    /// by `factor_source_id`, or `None` if no unsecurified persona controlled by that
    /// factor source id found.
    fn max_identity_veci(
        &self,
        factor_source_id: FactorSourceIDFromHash,
    ) -> Option<HDPathComponent> {
        self.max_entity_veci(
            factor_source_id,
            self.unsecurified_personas_on_network
                .clone()
                .iter()
                .map(Into::<AnyUnsecurifiedEntity>::into)
                .collect::<IdentifiedVecOf<_>>(),
            self.securified_personas_on_network
                .clone()
                .into_iter()
                .map(|x| x.erase_to_any()),
            CAP26EntityKind::Identity,
            KeySpace::Unsecurified { is_hardened: true },
        )
    }

    /// Returns the Max Derivation Entity Index of Securified Accounts controlled
    /// by `factor_source_id`, or `None` if no securified account controlled by that
    /// factor source id found.
    /// By "controlled by" we mean having a MatrixOfFactorInstances which has that
    /// factor in **any role** in its MatrixOfFactorInstances.
    fn max_entity_mfa<
        E: IsBaseEntity
            + std::hash::Hash
            + Eq
            + Clone
            + std::fmt::Debug
            + Identifiable,
    >(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        unsecurified_entities: &IdentifiedVecOf<AbstractUnsecurifiedEntity<E>>,
        securified_entities: &IdentifiedVecOf<AbstractSecurifiedEntity<E>>,
        entity_kind: CAP26EntityKind,
    ) -> Option<HDPathComponent> {
        let predicate = AssertMatches {
            network_id: self.network_id,
            key_kind: CAP26KeyKind::TransactionSigning,
            entity_kind,
            key_space: KeySpace::Securified,
        };
        let max_securified = securified_entities
            .iter()
            .flat_map(|e| {
                e.highest_derivation_path_index(factor_source_id, predicate)
            })
            .max();

        let max_provisional_unsecurified = unsecurified_entities
            .iter()
            .filter_map(|e| e.provisional_securified_config.clone())
            .flat_map(|x| {
                x.highest_derivation_path_index(factor_source_id, predicate)
            })
            .max();

        max_securified.max(max_provisional_unsecurified)
    }

    /// Returns the Max Derivation Entity Index of Securified Accounts controlled
    /// by `factor_source_id`, or `None` if no securified account controlled by that
    /// factor source id found.
    /// By "controlled by" we mean having a MatrixOfFactorInstances which has that
    /// factor in **any role** in its MatrixOfFactorInstances.
    fn max_account_mfa(
        &self,
        factor_source_id: FactorSourceIDFromHash,
    ) -> Option<HDPathComponent> {
        self.max_entity_mfa(
            factor_source_id,
            &self.unsecurified_accounts_on_network,
            &self.securified_accounts_on_network,
            CAP26EntityKind::Account,
        )
    }

    /// Returns the Max Derivation Entity Index of Securified Persona controlled
    /// by `factor_source_id`, or `None` if no securified persona controlled by that
    /// factor source id found.
    /// By "controlled by" we mean having a MatrixOfFactorInstances which has that
    /// factor in **any role** in its MatrixOfFactorInstances.
    fn max_identity_mfa(
        &self,
        factor_source_id: FactorSourceIDFromHash,
    ) -> Option<HDPathComponent> {
        self.max_entity_mfa(
            factor_source_id,
            &self.unsecurified_personas_on_network,
            &self.securified_personas_on_network,
            CAP26EntityKind::Identity,
        )
    }

    fn max_entity_rola<
        E: IsBaseEntity
            + std::hash::Hash
            + Eq
            + Clone
            + std::fmt::Debug
            + Identifiable,
    >(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        unsecurified_entities: &IdentifiedVecOf<AbstractUnsecurifiedEntity<E>>,
        securified_entities: &IdentifiedVecOf<AbstractSecurifiedEntity<E>>,
        entity_kind: CAP26EntityKind,
    ) -> Option<HDPathComponent> {
        let predicate = AssertMatches {
            network_id: self.network_id,
            key_kind: CAP26KeyKind::AuthenticationSigning,
            entity_kind,
            key_space: KeySpace::Securified,
        };

        let max_unsecurified = unsecurified_entities
        .iter()
        .filter(|e| e.network_id() == self.network_id)
        .flat_map(|e| {
            // Check if this entity's security state has a provisional securified config and if
            // the factor source id matches the specified one.
            e
            .provisional_securified_config
            .as_ref()
            .and_then(|x|
                match x {
                    ProvisionalSecurifiedConfig::FactorInstancesDerived { value } => if value.authentication_signing_factor_instance.factor_source_id == factor_source_id {
                    // Matches
                    Some(value.authentication_signing_factor_instance.derivation_entity_index())
                } else { None }})
        })
        .max();

        let max_securified = securified_entities
            .iter()
            .flat_map(|e| {

                // Check if the non-provisional's FactorSourceID matches the specified one.
                let non_provisional_matching = {
                    let maybe_wrong_factor = e.securified_entity_control.authentication_signing_factor_instance();

                    if maybe_wrong_factor.factor_source_id == factor_source_id {
                        Some(maybe_wrong_factor.clone())
                    } else {
                        None
                    }
                };

                // Check if this entity's security state has a provisional securified config and if
                // the factor source id matches the specified one.
                let provisional = e.securified_entity_control.provisional_securified_config.as_ref().and_then(|x|
                    match x {
                        ProvisionalSecurifiedConfig::FactorInstancesDerived { value } => if value.authentication_signing_factor_instance.factor_source_id == factor_source_id {
                        // Matches
                        Some(value.authentication_signing_factor_instance.clone())
                    } else { None }});

                highest_derivation_index_of_hd_factors(
                    [provisional, non_provisional_matching],
                    factor_source_id,
                    predicate,
                )
            })
            .max();

        max_unsecurified.max(max_securified)
    }

    /// Returns the max index of true ROLA keys of securified accounts with
    /// factor source matching factor_source_id - if any.
    fn max_account_rola(
        &self,
        factor_source_id: FactorSourceIDFromHash,
    ) -> Option<HDPathComponent> {
        self.max_entity_rola(
            factor_source_id,
            &self.unsecurified_accounts_on_network,
            &self.securified_accounts_on_network,
            CAP26EntityKind::Account,
        )
    }

    /// Returns the max index of true ROLA keys of securified personas with
    /// factor source matching factor_source_id - if any.
    fn max_identity_rola(
        &self,
        factor_source_id: FactorSourceIDFromHash,
    ) -> Option<HDPathComponent> {
        self.max_entity_rola(
            factor_source_id,
            &self.unsecurified_personas_on_network,
            &self.securified_personas_on_network,
            CAP26EntityKind::Identity,
        )
    }

    /// Finds the "next" derivation entity index `HDPathComponent`, for
    /// the given `IndexAgnosticPath` adnd `factor_source_id`, which is `Max + 1`.
    /// Returns `None` if `Max` is `None` (see `max_account_veci`, `max_identity_mfa` for more details).
    ///
    /// Returns `Err` if the addition of one would overflow.
    pub fn next(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        agnostic_path: IndexAgnosticPath,
    ) -> Result<Option<HDPathComponent>> {
        if agnostic_path.network_id != self.network_id {
            return Err(CommonError::NetworkDiscrepancy {
                expected: self.network_id.to_string(),
                actual: agnostic_path.network_id.to_string(),
            });
        }
        let derivation_preset = DerivationPreset::try_from(agnostic_path)?;

        let max = match derivation_preset {
            DerivationPreset::AccountVeci => {
                self.max_account_veci(factor_source_id)
            }
            DerivationPreset::AccountMfa => {
                self.max_account_mfa(factor_source_id)
            }
            DerivationPreset::IdentityVeci => {
                self.max_identity_veci(factor_source_id)
            }
            DerivationPreset::IdentityMfa => {
                self.max_identity_mfa(factor_source_id)
            }
            DerivationPreset::AccountRola => {
                self.max_account_rola(factor_source_id)
            }
            DerivationPreset::IdentityRola => {
                self.max_identity_rola(factor_source_id)
            }
        };

        let Some(max) = max else { return Ok(None) };
        max.checked_add_one_to_global().map(Some)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NextDerivationEntityIndexProfileAnalyzingAssigner;

    #[test]
    fn test_network_discrepancy() {
        let sut = SUT::new(NetworkID::Mainnet, None);
        assert_eq!(
            sut.next(
                FactorSourceIDFromHash::sample_at(0),
                DerivationPreset::AccountVeci
                    .index_agnostic_path_on_network(NetworkID::Stokenet),
            ),
            Err(CommonError::NetworkDiscrepancy {
                expected: NetworkID::Mainnet.to_string(),
                actual: NetworkID::Stokenet.to_string()
            })
        );
    }

    #[test]
    fn test_next_account_veci_with_single_at_0_is_1() {
        let preset = DerivationPreset::AccountVeci;
        let network_id = NetworkID::Mainnet;
        let sut = SUT::new(
            network_id,
            Arc::new(Profile::sample_from(
                FactorSource::sample_all(),
                [&Account::sample_at(0)],
                [],
            )),
        );
        let next = sut
            .next(
                FactorSourceIDFromHash::sample_at(0),
                preset.index_agnostic_path_on_network(network_id),
            )
            .unwrap();

        assert_eq!(
            next,
            HDPathComponent::from_local_key_space(
                1,
                KeySpace::Unsecurified { is_hardened: true }
            )
            .ok()
        )
    }

    #[test]
    fn test_next_account_veci_with_unused_factor_is_none() {
        let preset = DerivationPreset::AccountVeci;
        let network_id = NetworkID::Mainnet;
        let sut = SUT::new(
            network_id,
            Arc::new(Profile::sample_from(
                FactorSource::sample_all(),
                [&Account::sample_at(0)],
                [],
            )),
        );
        let next = sut
            .next(
                FactorSourceIDFromHash::sample_at(1), // <-- UNUSED
                preset.index_agnostic_path_on_network(network_id),
            )
            .unwrap();

        assert_eq!(next, None)
    }

    #[test]
    fn test_next_account_mfa_with_single_unsecurified_is_none() {
        let preset = DerivationPreset::AccountMfa;
        let network_id = NetworkID::Mainnet;
        let sut = SUT::new(
            network_id,
            Arc::new(Profile::sample_from(
                FactorSource::sample_all(),
                [&Account::sample_at(0)],
                [],
            )),
        );
        let next = sut
            .next(
                FactorSourceIDFromHash::sample_at(0),
                preset.index_agnostic_path_on_network(network_id),
            )
            .unwrap();

        assert_eq!(next, None)
    }

    #[test]
    fn test_next_account_mfa_with_single_unsecurified_with_provisional_0_is_1()
    {
        let preset = DerivationPreset::AccountMfa;
        let network_id = NetworkID::Mainnet;

        let mut account = Account::sample_at(0);

        let fi = HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(0);
        let fsid = fi.factor_source_id;
        let fi = FactorInstance::from(fi);

        let matrix_of_factors = unsafe {
            // An invalid matrix! Ok for this test...
            MatrixOfFactorInstances::unbuilt_with_roles_and_days(
                PrimaryRoleWithFactorInstances::unbuilt_with_factors(
                    Threshold::All,
                    [fi.clone()],
                    [],
                ),
                RecoveryRoleWithFactorInstances::override_only([fi.clone()]),
                ConfirmationRoleWithFactorInstances::override_only(
                    [fi.clone()],
                ),
                TimePeriod::with_days(10),
            )
        };

        let ssofi = SecurityStructureOfFactorInstances::new(SecurityStructureID::sample(), matrix_of_factors, HierarchicalDeterministicFactorInstance::sample_auth_signing_account_securified()).unwrap();

        account.set_provisional(
            ProvisionalSecurifiedConfig::FactorInstancesDerived {
                value: ssofi,
            },
        );
        let sut = SUT::new(
            network_id,
            Arc::new(Profile::sample_from(
                FactorSource::sample_all(),
                [&account],
                [],
            )),
        );
        let next = sut
            .next(fsid, preset.index_agnostic_path_on_network(network_id))
            .unwrap();

        assert_eq!(
            next,
            HDPathComponent::from_local_key_space(1, KeySpace::Securified).ok()
        )
    }

    #[test]
    fn test_next_persona_mfa_with_single_unsecurified_with_provisional_0_is_1()
    {
        let preset = DerivationPreset::IdentityMfa;
        let network_id = NetworkID::Mainnet;

        let mut persona = Persona::sample_at(0);

        let fi = HierarchicalDeterministicFactorInstance::sample_mainnet_entity_device_factor_fs_0_securified_at_index(
            CAP26EntityKind::Identity,
            0,
        );
        let fsid = fi.factor_source_id;
        let fi = FactorInstance::from(fi);

        let matrix_of_factors = unsafe {
            // An invalid matrix! Ok for this test...
            MatrixOfFactorInstances::unbuilt_with_roles_and_days(
                PrimaryRoleWithFactorInstances::unbuilt_with_factors(
                    Threshold::All,
                    [fi.clone()],
                    [],
                ),
                RecoveryRoleWithFactorInstances::override_only([fi.clone()]),
                ConfirmationRoleWithFactorInstances::override_only(
                    [fi.clone()],
                ),
                TimePeriod::with_days(10),
            )
        };

        let ssofi = SecurityStructureOfFactorInstances::new(SecurityStructureID::sample(), matrix_of_factors, HierarchicalDeterministicFactorInstance::sample_with_key_kind_entity_kind_on_network_and_hardened_index(
            NetworkID::Mainnet,
            CAP26KeyKind::AuthenticationSigning,
            CAP26EntityKind::Identity,
            Hardened::Securified(SecurifiedU30::ZERO),
        )).unwrap();

        persona.set_provisional(
            ProvisionalSecurifiedConfig::FactorInstancesDerived {
                value: ssofi,
            },
        );
        let sut = SUT::new(
            network_id,
            Arc::new(Profile::sample_from(
                FactorSource::sample_all(),
                [],
                [&persona],
            )),
        );
        let next = sut
            .next(fsid, preset.index_agnostic_path_on_network(network_id))
            .unwrap();

        assert_eq!(
            next,
            HDPathComponent::from_local_key_space(1, KeySpace::Securified).ok()
        )
    }

    #[test]
    fn test_next_account_veci_with_single_at_8_is_9() {
        let preset = DerivationPreset::AccountVeci;
        let network_id = NetworkID::Mainnet;
        let sut = SUT::new(
            network_id,
            Arc::new(Profile::sample_from(
                FactorSource::sample_all(),
                [
                    &Account::sample_at(8),
                    &Account::sample_at(2), /* securified, should not interfere */
                ],
                [],
            )),
        );
        let next = sut
            .next(
                FactorSourceIDFromHash::sample_at(10),
                preset.index_agnostic_path_on_network(network_id),
            )
            .unwrap();

        assert_eq!(
            next,
            HDPathComponent::from_local_key_space(
                9,
                KeySpace::Unsecurified { is_hardened: true }
            )
            .ok()
        )
    }

    #[test]
    fn test_next_account_mfa_with_single_at_7_is_8() {
        let preset = DerivationPreset::AccountMfa;
        let network_id = NetworkID::Mainnet;
        let sut = SUT::new(
            network_id,
            Arc::new(Profile::sample_from(
                FactorSource::sample_all(),
                [
                    &Account::sample_at(8), /* unsecurified, should not interfere */
                    &Account::sample_at(7),
                ],
                [],
            )),
        );
        type F = FactorSourceIDFromHash;
        for fid in [
            F::sample_at(2),
            F::sample_at(6),
            F::sample_at(7),
            F::sample_at(8),
            F::sample_at(9),
        ] {
            let next = sut
                .next(fid, preset.index_agnostic_path_on_network(network_id))
                .unwrap();

            assert_eq!(
                next,
                HDPathComponent::from_local_key_space(8, KeySpace::Securified)
                    .ok()
            );
        }
    }

    #[test]
    fn test_next_account_mfa_securified_account_at_7_with_provisional_11_is_12()
    {
        let preset = DerivationPreset::AccountMfa;
        let network_id = NetworkID::Mainnet;
        let mut account_with_provisional = Account::sample_at(7);

        let matrix = unsafe {
            let idx = Hardened::from_local_key_space(11u32, IsSecurified(true))
                .unwrap();
            let general = GeneralRoleWithHierarchicalDeterministicFactorInstances::r7(
                            HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                                CAP26EntityKind::Account,
                                idx,
                            )
                        );

            // we use the threshold factors as override. no worries.
            let factors = general
                .get_threshold_factors()
                .into_iter()
                .map(FactorInstance::from)
                .collect_vec();

            // A completely invalid matrix! but its fine here, we dont test that...
            MatrixOfFactorInstances::unbuilt_with_roles_and_days(
                PrimaryRoleWithFactorInstances::with_factors(
                    0,
                    [],
                    factors.clone(),
                ),
                RecoveryRoleWithFactorInstances::override_only(factors.clone()),
                ConfirmationRoleWithFactorInstances::override_only(
                    factors.clone(),
                ),
                TimePeriod::with_days(123),
            )
        };

        let provisional_security_structure =
            SecurityStructureOfFactorInstances::new(
                SecurityStructureID::sample(),
                matrix,
                HierarchicalDeterministicFactorInstance::sample_auth_signing_account_securified(),
            )
            .unwrap();
        account_with_provisional.set_provisional(
            ProvisionalSecurifiedConfig::FactorInstancesDerived {
                value: provisional_security_structure,
            },
        );
        let sut = SUT::new(
            network_id,
            Arc::new(Profile::sample_from(
                FactorSource::sample_all(),
                [
                    &Account::sample_at(8), /* unsecurified, should not interfere */
                    &account_with_provisional,
                ],
                [],
            )),
        );
        type F = FactorSourceIDFromHash;
        for fid in [
            F::sample_at(2),
            F::sample_at(6),
            F::sample_at(7),
            F::sample_at(8),
            F::sample_at(9),
        ] {
            let next = sut
                .next(fid, preset.index_agnostic_path_on_network(network_id))
                .unwrap();

            assert_eq!(
                next,
                HDPathComponent::from_local_key_space(12, KeySpace::Securified)
                    .ok()
            );
        }
    }

    #[test]
    fn test_next_identity_mfa_with_single_at_7_is_8() {
        let preset = DerivationPreset::IdentityMfa;
        let network_id = NetworkID::Mainnet;
        let sut = SUT::new(
            network_id,
            Arc::new(Profile::sample_from(
                FactorSource::sample_all(),
                [],
                [&Persona::sample_at(7)],
            )),
        );
        type F = FactorSourceIDFromHash;
        for fid in [
            F::sample_at(2),
            F::sample_at(6),
            F::sample_at(7),
            F::sample_at(8),
            F::sample_at(9),
        ] {
            let next = sut
                .next(fid, preset.index_agnostic_path_on_network(network_id))
                .unwrap();

            assert_eq!(
                next,
                HDPathComponent::from_local_key_space(8, KeySpace::Securified)
                    .ok()
            )
        }
    }

    #[test]
    fn test_next_identity_veci_with_single_at_1_is_2() {
        let preset = DerivationPreset::IdentityVeci;
        let network_id = NetworkID::Mainnet;
        let sut = SUT::new(
            network_id,
            Arc::new(Profile::sample_from(
                FactorSource::sample_all(),
                [],
                [
                    &Persona::sample_at(7), /* securified should not interfere */
                    &Persona::sample_at(1),
                ],
            )),
        );
        let next = sut
            .next(
                FactorSourceIDFromHash::sample_at(1),
                preset.index_agnostic_path_on_network(network_id),
            )
            .unwrap();

        assert_eq!(
            next,
            HDPathComponent::from_local_key_space(
                2,
                KeySpace::Unsecurified { is_hardened: true }
            )
            .ok()
        )
    }

    #[test]
    fn test_next_account_veci_with_non_contiguous_at_0_1_7_is_8() {
        let fsid = FactorSourceIDFromHash::sample_at(0);

        let fi0 = HierarchicalDeterministicFactorInstance::new_for_entity(
            fsid,
            CAP26EntityKind::Account,
            Hardened::Unsecurified(
                UnsecurifiedHardened::try_from(0u32).unwrap(),
            ),
        );

        let fi1 = HierarchicalDeterministicFactorInstance::new_for_entity(
            fsid,
            CAP26EntityKind::Account,
            Hardened::Unsecurified(
                UnsecurifiedHardened::try_from(1u32).unwrap(),
            ),
        );

        let fi7 = HierarchicalDeterministicFactorInstance::new_for_entity(
            fsid,
            CAP26EntityKind::Account,
            Hardened::Unsecurified(
                UnsecurifiedHardened::try_from(7u32).unwrap(),
            ),
        );

        let network_id = NetworkID::Mainnet;
        let accounts = [fi0, fi1, fi7]
            .map(|fi| HDFactorInstanceTransactionSigning::new(fi).unwrap())
            .map(|fi| {
                Account::new(fi, DisplayName::sample(), AppearanceID::sample())
            });
        let sut = SUT::new(
            network_id,
            Arc::new(Profile::sample_from(
                FactorSource::sample_all(),
                &accounts,
                [],
            )),
        );
        let next = sut
            .next(
                fsid,
                DerivationPreset::AccountVeci
                    .index_agnostic_path_on_network(network_id),
            )
            .unwrap();

        assert_eq!(
            next,
            HDPathComponent::from_local_key_space(
                8,
                KeySpace::Unsecurified { is_hardened: true }
            )
            .ok()
        )
    }

    #[test]
    fn test_next_account_rola_at_7_is_8() {
        let preset = DerivationPreset::AccountRola;
        let network_id = NetworkID::Mainnet;
        let sut = SUT::new(
            network_id,
            Arc::new(Profile::sample_from(
                FactorSource::sample_all(),
                [
                    &Account::sample_at(8), /* unsecurified, should not interfere */
                    &Account::sample_at(7),
                ],
                [],
            )),
        );
        type F = FactorSourceIDFromHash;
        {
            let fid = F::sample_device();
            let next = sut
                .next(fid, preset.index_agnostic_path_on_network(network_id))
                .unwrap();

            assert_eq!(
                next,
                HDPathComponent::from_local_key_space(8, KeySpace::Securified)
                    .ok()
            );
        }
    }

    #[test]
    fn test_next_identity_rola_at_7_is_8() {
        let preset = DerivationPreset::IdentityRola;
        let network_id = NetworkID::Mainnet;
        let persona = Persona::sample_at(7);
        let profile =
            Profile::sample_from(FactorSource::sample_all(), [], [&persona]);
        let sut = SUT::new(network_id, Arc::new(profile));
        type F = FactorSourceIDFromHash;
        {
            let fid = F::sample_device();
            let next = sut
                .next(fid, preset.index_agnostic_path_on_network(network_id))
                .unwrap();

            assert_eq!(
                next,
                HDPathComponent::from_local_key_space(8, KeySpace::Securified)
                    .ok()
            )
        }
    }

    #[test]
    fn test_next_identity_rola_at_7_other_unsecurified_with_provisional_13_is_14(
    ) {
        let preset = DerivationPreset::IdentityRola;
        let network_id = NetworkID::Mainnet;
        let persona = Persona::sample_at(7);
        type F = FactorSourceIDFromHash;
        let fsid = F::sample_device();
        let mut persona_with_provisional = Persona::sample_mainnet();
        assert!(!persona_with_provisional.is_securified());
        let mut sec_struct_factor_instances =
            SecurityStructureOfFactorInstances::sample();
        sec_struct_factor_instances.authentication_signing_factor_instance =
            HierarchicalDeterministicFactorInstance::new_for_entity_with_key_kind_on_network(
                CAP26KeyKind::AuthenticationSigning,
                network_id,
                fsid,
                CAP26EntityKind::Identity,
                Hardened::Securified(SecurifiedU30::try_from(13u32).unwrap()),
            );

        persona_with_provisional.set_provisional(
            ProvisionalSecurifiedConfig::FactorInstancesDerived {
                value: sec_struct_factor_instances,
            },
        );
        let profile = Profile::sample_from(
            FactorSource::sample_all(),
            [],
            [&persona, &persona_with_provisional],
        );
        let sut = SUT::new(network_id, Arc::new(profile));
        {
            let next = sut
                .next(fsid, preset.index_agnostic_path_on_network(network_id))
                .unwrap();

            assert_eq!(
                next,
                HDPathComponent::from_local_key_space(14, KeySpace::Securified)
                    .ok()
            )
        }
    }

    #[test]
    fn test_next_identity_rola_at_6_other_securified_at_7_with_provisional_13_is_14(
    ) {
        let preset = DerivationPreset::IdentityRola;
        let network_id = NetworkID::Mainnet;
        let persona = Persona::sample_at(6);
        type F = FactorSourceIDFromHash;
        let fsid = F::sample_device();
        let mut persona_with_provisional = Persona::sample_at(7);
        assert!(persona_with_provisional.is_securified());
        let mut sec_struct_factor_instances =
            SecurityStructureOfFactorInstances::sample();
        sec_struct_factor_instances.authentication_signing_factor_instance =
            HierarchicalDeterministicFactorInstance::new_for_entity_with_key_kind_on_network(
                CAP26KeyKind::AuthenticationSigning,
                network_id,
                fsid,
                CAP26EntityKind::Identity,
                Hardened::Securified(SecurifiedU30::try_from(13u32).unwrap()),
            );

        persona_with_provisional.set_provisional(
            ProvisionalSecurifiedConfig::FactorInstancesDerived {
                value: sec_struct_factor_instances,
            },
        );
        let profile = Profile::sample_from(
            FactorSource::sample_all(),
            [],
            [&persona, &persona_with_provisional],
        );
        let sut = SUT::new(network_id, Arc::new(profile));
        {
            let next = sut
                .next(fsid, preset.index_agnostic_path_on_network(network_id))
                .unwrap();

            assert_eq!(
                next,
                HDPathComponent::from_local_key_space(14, KeySpace::Securified)
                    .ok()
            )
        }
    }

    #[test]
    fn test_next_identity_rola_at_6_and_account_securified_at_7_with_provisional_13_is_7(
    ) {
        let preset = DerivationPreset::IdentityRola;
        let network_id = NetworkID::Mainnet;
        let persona = Persona::sample_at(6);
        type F = FactorSourceIDFromHash;
        let fsid = F::sample_device();
        let mut account_with_provisional = Account::sample_at(7);
        assert!(account_with_provisional.is_securified());
        let mut sec_struct_factor_instances =
            SecurityStructureOfFactorInstances::sample();
        sec_struct_factor_instances.authentication_signing_factor_instance =
            HierarchicalDeterministicFactorInstance::new_for_entity_with_key_kind_on_network(
                CAP26KeyKind::AuthenticationSigning,
                network_id,
                fsid,
                CAP26EntityKind::Account,
                Hardened::Securified(SecurifiedU30::try_from(13u32).unwrap()),
            );

        account_with_provisional.set_provisional(
            ProvisionalSecurifiedConfig::FactorInstancesDerived {
                value: sec_struct_factor_instances,
            },
        );
        let profile = Profile::sample_from(
            FactorSource::sample_all(),
            [&account_with_provisional],
            [&persona],
        );
        let sut = SUT::new(network_id, Arc::new(profile));
        {
            let next = sut
                .next(fsid, preset.index_agnostic_path_on_network(network_id))
                .unwrap();

            assert_eq!(
                next,
                HDPathComponent::from_local_key_space(7, KeySpace::Securified)
                    .ok()
            )
        }
    }

    #[test]
    fn test_next_identity_rola_at_7_other_with_provisional_4_is_8() {
        let preset = DerivationPreset::IdentityRola;
        let network_id = NetworkID::Mainnet;
        let persona = Persona::sample_at(7);
        assert!(persona.is_securified());
        type F = FactorSourceIDFromHash;
        let fsid = F::sample_device();
        let mut persona_with_provisional = Persona::sample_mainnet();
        assert!(!persona_with_provisional.is_securified());
        let mut sec_struct_factor_instances =
            SecurityStructureOfFactorInstances::sample();
        sec_struct_factor_instances.authentication_signing_factor_instance =
            HierarchicalDeterministicFactorInstance::new_for_entity_with_key_kind_on_network(
                CAP26KeyKind::AuthenticationSigning,
                network_id,
                fsid,
                CAP26EntityKind::Identity,
                Hardened::Securified(SecurifiedU30::try_from(4u32).unwrap()),
            );

        persona_with_provisional.set_provisional(
            ProvisionalSecurifiedConfig::FactorInstancesDerived {
                value: sec_struct_factor_instances,
            },
        );
        let profile = Profile::sample_from(
            FactorSource::sample_all(),
            [],
            [&persona, &persona_with_provisional],
        );
        let sut = SUT::new(network_id, Arc::new(profile));
        {
            let next = sut
                .next(fsid, preset.index_agnostic_path_on_network(network_id))
                .unwrap();

            assert_eq!(
                next,
                HDPathComponent::from_local_key_space(8, KeySpace::Securified)
                    .ok()
            )
        }
    }

    #[test]
    fn test_next_account_rola_at_7_other_unsecurified_with_provisional_13_is_14(
    ) {
        let preset = DerivationPreset::AccountRola;
        let network_id = NetworkID::Mainnet;
        let account = Account::sample_at(7);
        type F = FactorSourceIDFromHash;
        let fsid = F::sample_device();
        let mut account_with_provisional = Account::sample_mainnet();
        assert!(!account_with_provisional.is_securified());
        let mut sec_struct_factor_instances =
            SecurityStructureOfFactorInstances::sample();
        sec_struct_factor_instances.authentication_signing_factor_instance =
            HierarchicalDeterministicFactorInstance::new_for_entity_with_key_kind_on_network(
                CAP26KeyKind::AuthenticationSigning,
                network_id,
                fsid,
                CAP26EntityKind::Account,
                Hardened::Securified(SecurifiedU30::try_from(13u32).unwrap()),
            );

        account_with_provisional.set_provisional(
            ProvisionalSecurifiedConfig::FactorInstancesDerived {
                value: sec_struct_factor_instances,
            },
        );
        let profile = Profile::sample_from(
            FactorSource::sample_all(),
            [&account, &account_with_provisional],
            [],
        );
        let sut = SUT::new(network_id, Arc::new(profile));
        {
            let next = sut
                .next(fsid, preset.index_agnostic_path_on_network(network_id))
                .unwrap();

            assert_eq!(
                next,
                HDPathComponent::from_local_key_space(14, KeySpace::Securified)
                    .ok()
            )
        }
    }
}
