use crate::prelude::*;

/// An analyzer of a `Profile` for some `network_id` (i.e. analyzer of `ProfileNetwork`),
/// reading out the max derivation entity index for Unsecurified/Securified Accounts/Personas
/// for some factor source id.
pub struct NextDerivationEntityIndexProfileAnalyzingAssigner {
    network_id: NetworkID,

    /// might be empty
    unsecurified_accounts_on_network: IndexSet<UnsecurifiedEntity>,

    /// might be empty
    securified_accounts_on_network: IndexSet<SecurifiedAccount>,

    /// might be empty
    unsecurified_personas_on_network: IndexSet<UnsecurifiedEntity>,

    /// might be empty
    securified_personas_on_network: IndexSet<SecurifiedPersona>,
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

        Self {
            network_id,
            unsecurified_accounts_on_network,
            securified_accounts_on_network,
            unsecurified_personas_on_network,
            securified_personas_on_network,
        }
    }

    fn max_entity_veci(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        entities: impl IntoIterator<Item = UnsecurifiedEntity>,
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
            self.unsecurified_accounts_on_network.clone(),
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
            self.unsecurified_personas_on_network.clone(),
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
    fn max_account_mfa(
        &self,
        factor_source_id: FactorSourceIDFromHash,
    ) -> Option<HDPathComponent> {
        self.securified_accounts_on_network
            .clone()
            .into_iter()
            .flat_map(|e: SecurifiedAccount| {
                e.highest_derivation_path_index(
                    factor_source_id,
                    AssertMatches {
                        network_id: self.network_id,
                        key_kind: CAP26KeyKind::TransactionSigning,
                        entity_kind: CAP26EntityKind::Account,
                        key_space: KeySpace::Securified,
                    },
                )
            })
            .max()
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
        self.securified_personas_on_network
            .clone()
            .into_iter()
            .flat_map(|e: SecurifiedPersona| {
                e.highest_derivation_path_index(
                    factor_source_id,
                    AssertMatches {
                        network_id: self.network_id,
                        key_kind: CAP26KeyKind::TransactionSigning,
                        entity_kind: CAP26EntityKind::Identity,
                        key_space: KeySpace::Securified,
                    },
                )
            })
            .max()
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
                expected: self.network_id,
                actual: agnostic_path.network_id,
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
            DerivationPreset::AccountRola => todo!(),
            DerivationPreset::IdentityRola => todo!(),
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
        assert!(matches!(
            sut.next(
                FactorSourceIDFromHash::sample_at(0),
                DerivationPreset::AccountVeci
                    .index_agnostic_path_on_network(NetworkID::Stokenet),
            ),
            Err(CommonError::NetworkDiscrepancy {
                expected: NetworkID::Mainnet,
                actual: NetworkID::Stokenet
            })
        ));
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
}
