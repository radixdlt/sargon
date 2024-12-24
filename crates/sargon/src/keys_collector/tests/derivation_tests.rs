#![cfg(test)]
use crate::prelude::*;

impl DerivationPath {
    pub fn for_entity(
        network_id: NetworkID,
        entity_kind: CAP26EntityKind,
        key_kind: CAP26KeyKind,
        hardened: Hardened,
    ) -> Self {
        match entity_kind {
            CAP26EntityKind::Account => DerivationPath::account(
                AccountPath::new(network_id, key_kind, hardened),
            ),
            CAP26EntityKind::Identity => DerivationPath::identity(
                IdentityPath::new(network_id, key_kind, hardened),
            ),
        }
    }

    pub fn hardening_global_index(
        network_id: NetworkID,
        entity_kind: CAP26EntityKind,
        key_kind: CAP26KeyKind,
        global_key_space: u32,
    ) -> Self {
        let index = Hardened::from_global_key_space(global_key_space).unwrap();
        Self::for_entity(network_id, entity_kind, key_kind, index)
    }

    pub fn unsecurified_hardening_base_index(
        network_id: NetworkID,
        entity_kind: CAP26EntityKind,
        key_kind: CAP26KeyKind,
        index: u32,
    ) -> Self {
        let index = U30::try_from(index).unwrap();
        let index = Hardened::Unsecurified(UnsecurifiedHardened::from(index));
        match entity_kind {
            CAP26EntityKind::Account => DerivationPath::account(
                AccountPath::new(network_id, key_kind, index),
            ),
            CAP26EntityKind::Identity => DerivationPath::identity(
                IdentityPath::new(network_id, key_kind, index),
            ),
        }
    }

    pub fn hardening_global_index_account_tx(
        network_id: NetworkID,
        global_key_space: u32,
    ) -> Self {
        Self::hardening_global_index(
            network_id,
            CAP26EntityKind::Account,
            CAP26KeyKind::TransactionSigning,
            global_key_space,
        )
    }

    pub fn hardening_global_index_account_rola(
        network_id: NetworkID,
        global_key_space: u32,
    ) -> Self {
        Self::hardening_global_index(
            network_id,
            CAP26EntityKind::Account,
            CAP26KeyKind::AuthenticationSigning,
            global_key_space,
        )
    }

    pub fn hardening_global_index_identity_tx(
        network_id: NetworkID,
        global_key_space: u32,
    ) -> Self {
        Self::hardening_global_index(
            network_id,
            CAP26EntityKind::Identity,
            CAP26KeyKind::TransactionSigning,
            global_key_space,
        )
    }

    pub fn hardening_global_index_identity_rola(
        network_id: NetworkID,
        global_key_space: u32,
    ) -> Self {
        Self::hardening_global_index(
            network_id,
            CAP26EntityKind::Identity,
            CAP26KeyKind::AuthenticationSigning,
            global_key_space,
        )
    }

    pub fn account_tx_unsecurified_hardening_base_index(
        network_id: NetworkID,
        index: u32,
    ) -> Self {
        Self::unsecurified_hardening_base_index(
            network_id,
            CAP26EntityKind::Account,
            CAP26KeyKind::TransactionSigning,
            index,
        )
    }
}

mod key_derivation_tests {

    use super::CAP26EntityKind::*;
    use super::CAP26KeyKind::*;
    use super::NetworkID::*;
    use super::*;

    #[actix_rt::test]
    async fn failure_unknown_factor() {
        let res = KeysCollector::new(
            IndexSet::new(),
            IndexMap::just((
                FactorSourceIDFromHash::sample_at(0),
                IndexSet::just(DerivationPath::account(AccountPath::new(
                    Mainnet,
                    TransactionSigning,
                    Hardened::Securified(SecurifiedU30::ZERO),
                ))),
            )),
            Arc::new(TestDerivationInteractor::default()),
            DerivationPurpose::CreatingNewAccount,
        );
        assert!(matches!(
            res,
            Err(CommonError::ProfileDoesNotContainFactorSourceWithID {
                bad_value: _
            })
        ));
    }

    #[actix_rt::test]
    async fn failure_from_interactor() {
        let factor_source = FactorSource::sample_at(0);
        let paths = [0, 1, 2]
            .into_iter()
            .map(|i| {
                DerivationPath::unsecurified_hardening_base_index(
                    Mainnet,
                    Account,
                    TransactionSigning,
                    i,
                )
            })
            .collect::<IndexSet<_>>();
        let collector = KeysCollector::new(
            FactorSource::sample_all(),
            [(factor_source.id_from_hash(), paths.clone())]
                .into_iter()
                .collect::<IndexMap<FactorSourceIDFromHash, IndexSet<DerivationPath>>>(),
            Arc::new(TestDerivationInteractor::fail()),
            DerivationPurpose::CreatingNewAccount
        )
        .unwrap();
        let outcome = collector.collect_keys().await;
        assert!(outcome.all_factors().factor_instances().is_empty())
    }
    mod multi_key {
        use super::*;

        #[actix_rt::test]
        async fn multi_keys_same_factor_source_different_indices() {
            let factor_source = FactorSource::sample_at(0);
            let paths = [0, 1, 2]
                .into_iter()
                .map(|i| {
                    DerivationPath::unsecurified_hardening_base_index(
                        Mainnet,
                        Account,
                        TransactionSigning,
                        i,
                    )
                })
                .collect::<IndexSet<_>>();
            let collector = KeysCollector::new_test(
                [(factor_source.id_from_hash(), paths.clone())],
                DerivationPurpose::CreatingNewAccount,
            );
            let outcome = collector.collect_keys().await;
            assert_eq!(
                outcome
                    .all_factors()
                    .into_iter()
                    .map(|f| f.derivation_path())
                    .collect::<IndexSet<_>>(),
                paths
            );

            assert!(outcome
                .all_factors()
                .into_iter()
                .all(|f| f.factor_source_id == factor_source.id_from_hash()));
        }

        #[actix_rt::test]
        async fn multi_keys_multi_factor_sources_single_index_per() {
            let path =
                DerivationPath::account_tx_unsecurified_hardening_base_index(
                    Mainnet, 0,
                );
            let paths = IndexSet::just(path);
            let factor_sources = FactorSource::sample_all();

            let collector = KeysCollector::new_test(
                factor_sources
                    .iter()
                    .map(|f| (f.id_from_hash(), paths.clone()))
                    .collect_vec(),
                DerivationPurpose::CreatingNewAccount,
            );
            let outcome = collector.collect_keys().await;
            assert_eq!(
                outcome
                    .all_factors()
                    .into_iter()
                    .map(|f| f.derivation_path())
                    .collect::<IndexSet<_>>(),
                paths
            );

            assert_eq!(
                outcome
                    .all_factors()
                    .into_iter()
                    .map(|f| f.factor_source_id)
                    .collect::<HashSet::<_>>(),
                factor_sources
                    .into_iter()
                    .map(|f| f.id_from_hash())
                    .collect::<HashSet::<_>>()
            );
        }

        #[actix_rt::test]
        async fn multi_keys_multi_factor_sources_multi_paths() {
            let paths = [0, 1, 2]
                .into_iter()
                .map(|i| {
                    DerivationPath::unsecurified_hardening_base_index(
                        Mainnet,
                        Account,
                        TransactionSigning,
                        i,
                    )
                })
                .collect::<IndexSet<_>>();

            let factor_sources = FactorSource::sample_all();

            let collector = KeysCollector::new_test(
                factor_sources
                    .iter()
                    .map(|f| (f.id_from_hash(), paths.clone()))
                    .collect_vec(),
                DerivationPurpose::CreatingNewAccount,
            );
            let outcome = collector.collect_keys().await;

            assert_eq!(
                outcome
                    .all_factors()
                    .into_iter()
                    .map(|f| f.derivation_path())
                    .collect::<IndexSet<_>>(),
                paths
            );

            assert_eq!(
                outcome
                    .all_factors()
                    .into_iter()
                    .map(|f| f.factor_source_id)
                    .collect::<HashSet::<_>>(),
                factor_sources
                    .into_iter()
                    .map(|f| f.id_from_hash())
                    .collect::<HashSet::<_>>()
            );
        }

        #[actix_rt::test]
        async fn multi_keys_multi_factor_sources_multi_paths_complex() {
            let mut paths = IndexSet::new();

            paths.extend([0, 1, 2].into_iter().map(|i| {
                DerivationPath::unsecurified_hardening_base_index(
                    Mainnet,
                    Account,
                    TransactionSigning,
                    i,
                )
            }));

            paths.extend([0, 1, 2].into_iter().map(|i| {
                DerivationPath::unsecurified_hardening_base_index(
                    Stokenet,
                    Account,
                    TransactionSigning,
                    i,
                )
            }));

            paths.extend([0, 1, 2].into_iter().map(|i| {
                DerivationPath::unsecurified_hardening_base_index(
                    Mainnet,
                    Identity,
                    TransactionSigning,
                    i,
                )
            }));

            paths.extend([0, 1, 2].into_iter().map(|i| {
                DerivationPath::unsecurified_hardening_base_index(
                    Stokenet,
                    Identity,
                    TransactionSigning,
                    i,
                )
            }));

            paths.extend([0, 1, 2].into_iter().map(|i| {
                DerivationPath::unsecurified_hardening_base_index(
                    Mainnet,
                    Account,
                    AuthenticationSigning,
                    i,
                )
            }));

            paths.extend([0, 1, 2].into_iter().map(|i| {
                DerivationPath::unsecurified_hardening_base_index(
                    Stokenet,
                    Account,
                    AuthenticationSigning,
                    i,
                )
            }));

            paths.extend([0, 1, 2].into_iter().map(|i| {
                DerivationPath::unsecurified_hardening_base_index(
                    Mainnet,
                    Identity,
                    AuthenticationSigning,
                    i,
                )
            }));

            paths.extend([0, 1, 2].into_iter().map(|i| {
                DerivationPath::unsecurified_hardening_base_index(
                    Stokenet,
                    Identity,
                    AuthenticationSigning,
                    i,
                )
            }));

            paths.extend(
                [
                    0,
                    1,
                    2,
                    RELATIVELY_LOCAL_OFFSET_SECURIFIED,
                    RELATIVELY_LOCAL_OFFSET_SECURIFIED + 1,
                    RELATIVELY_LOCAL_OFFSET_SECURIFIED + 2,
                ]
                .into_iter()
                .map(|i| i + GLOBAL_OFFSET_HARDENED)
                .map(|i| {
                    DerivationPath::hardening_global_index_account_tx(
                        NetworkID::Mainnet,
                        i,
                    )
                }),
            );

            paths.extend(
                [
                    0,
                    1,
                    2,
                    RELATIVELY_LOCAL_OFFSET_SECURIFIED,
                    RELATIVELY_LOCAL_OFFSET_SECURIFIED + 1,
                    RELATIVELY_LOCAL_OFFSET_SECURIFIED + 2,
                ]
                .into_iter()
                .map(|i| i + GLOBAL_OFFSET_HARDENED)
                .map(|i| {
                    DerivationPath::hardening_global_index_account_tx(
                        NetworkID::Stokenet,
                        i,
                    )
                }),
            );

            paths.extend(
                [
                    0,
                    1,
                    2,
                    RELATIVELY_LOCAL_OFFSET_SECURIFIED,
                    RELATIVELY_LOCAL_OFFSET_SECURIFIED + 1,
                    RELATIVELY_LOCAL_OFFSET_SECURIFIED + 2,
                ]
                .into_iter()
                .map(|i| i + GLOBAL_OFFSET_HARDENED)
                .map(|i| {
                    DerivationPath::hardening_global_index_identity_tx(
                        NetworkID::Mainnet,
                        i,
                    )
                }),
            );

            paths.extend(
                [
                    0,
                    1,
                    2,
                    RELATIVELY_LOCAL_OFFSET_SECURIFIED,
                    RELATIVELY_LOCAL_OFFSET_SECURIFIED + 1,
                    RELATIVELY_LOCAL_OFFSET_SECURIFIED + 2,
                ]
                .into_iter()
                .map(|i| i + GLOBAL_OFFSET_HARDENED)
                .map(|i| {
                    DerivationPath::hardening_global_index_identity_tx(
                        NetworkID::Stokenet,
                        i,
                    )
                }),
            );

            paths.extend(
                [
                    0,
                    1,
                    2,
                    RELATIVELY_LOCAL_OFFSET_SECURIFIED,
                    RELATIVELY_LOCAL_OFFSET_SECURIFIED + 1,
                    RELATIVELY_LOCAL_OFFSET_SECURIFIED + 2,
                ]
                .into_iter()
                .map(|i| i + GLOBAL_OFFSET_HARDENED)
                .map(|i| {
                    DerivationPath::hardening_global_index_account_rola(
                        NetworkID::Mainnet,
                        i,
                    )
                }),
            );

            paths.extend(
                [
                    0,
                    1,
                    2,
                    RELATIVELY_LOCAL_OFFSET_SECURIFIED,
                    RELATIVELY_LOCAL_OFFSET_SECURIFIED + 1,
                    RELATIVELY_LOCAL_OFFSET_SECURIFIED + 2,
                ]
                .into_iter()
                .map(|i| i + GLOBAL_OFFSET_HARDENED)
                .map(|i| {
                    DerivationPath::hardening_global_index_account_rola(
                        NetworkID::Stokenet,
                        i,
                    )
                }),
            );

            paths.extend(
                [
                    0,
                    1,
                    2,
                    RELATIVELY_LOCAL_OFFSET_SECURIFIED,
                    RELATIVELY_LOCAL_OFFSET_SECURIFIED + 1,
                    RELATIVELY_LOCAL_OFFSET_SECURIFIED + 2,
                ]
                .into_iter()
                .map(|i| i + GLOBAL_OFFSET_HARDENED)
                .map(|i| {
                    DerivationPath::hardening_global_index_identity_rola(
                        NetworkID::Mainnet,
                        i,
                    )
                }),
            );

            paths.extend(
                [
                    0,
                    1,
                    2,
                    RELATIVELY_LOCAL_OFFSET_SECURIFIED,
                    RELATIVELY_LOCAL_OFFSET_SECURIFIED + 1,
                    RELATIVELY_LOCAL_OFFSET_SECURIFIED + 2,
                ]
                .into_iter()
                .map(|i| i + GLOBAL_OFFSET_HARDENED)
                .map(|i| {
                    DerivationPath::hardening_global_index_identity_rola(
                        NetworkID::Stokenet,
                        i,
                    )
                }),
            );

            let factor_sources = FactorSource::sample_all();

            let collector = KeysCollector::new_test(
                factor_sources
                    .iter()
                    .map(|f| (f.id_from_hash(), paths.clone()))
                    .collect_vec(),
                DerivationPurpose::CreatingNewAccount,
            );
            let outcome = collector.collect_keys().await;

            assert_eq!(
                outcome
                    .all_factors()
                    .into_iter()
                    .map(|f| f.derivation_path())
                    .collect::<IndexSet<_>>(),
                paths
            );

            assert!(outcome.all_factors().factor_instances().len() > 300);

            assert_eq!(
                outcome
                    .all_factors()
                    .into_iter()
                    .map(|f| f.factor_source_id)
                    .collect::<HashSet::<_>>(),
                factor_sources
                    .into_iter()
                    .map(|f| f.id_from_hash())
                    .collect::<HashSet::<_>>()
            );
        }
    }
    mod single_key {
        use super::*;

        struct Expected {
            index: Hardened,
        }

        async fn do_test(
            key_space: KeySpace,
            factor_source: &FactorSource,
            network_id: NetworkID,
            entity_kind: CAP26EntityKind,
            key_kind: CAP26KeyKind,
            expected: Expected,
        ) {
            let collector = KeysCollector::with(
                factor_source,
                network_id,
                key_kind,
                entity_kind,
                key_space,
            );

            let outcome = collector.collect_keys().await;
            let factors = outcome.all_factors().factor_instances();
            assert_eq!(factors.len(), 1);
            let factor = factors.first().unwrap();
            assert_eq!(
                factor.derivation_path(),
                DerivationPath::for_entity(
                    network_id,
                    entity_kind,
                    key_kind,
                    expected.index
                )
            );
            assert_eq!(factor.factor_source_id, factor_source.id_from_hash());
        }

        mod securified {
            use super::*;

            async fn test(
                factor_source: &FactorSource,
                network_id: NetworkID,
                entity_kind: CAP26EntityKind,
                key_kind: CAP26KeyKind,
            ) {
                do_test(
                    KeySpace::Securified,
                    factor_source,
                    network_id,
                    entity_kind,
                    key_kind,
                    Expected {
                        index: Hardened::from_local_key_space(
                            0u32,
                            IsSecurified(true),
                        )
                        .unwrap(),
                    },
                )
                .await
            }

            mod account {
                use super::*;

                async fn each_factor(
                    network_id: NetworkID,
                    key_kind: CAP26KeyKind,
                ) {
                    for factor_source in FactorSource::sample_all().iter() {
                        test(factor_source, network_id, Account, key_kind).await
                    }
                }

                #[actix_rt::test]
                async fn single_first_account_mainnet_t9n() {
                    each_factor(Mainnet, TransactionSigning).await
                }
            }
        }

        mod unsecurified {
            use super::*;

            async fn test(
                factor_source: &FactorSource,
                network_id: NetworkID,
                entity_kind: CAP26EntityKind,
                key_kind: CAP26KeyKind,
            ) {
                do_test(
                    KeySpace::Unsecurified { is_hardened: true },
                    factor_source,
                    network_id,
                    entity_kind,
                    key_kind,
                    Expected {
                        index: Hardened::from_local_key_space(
                            0u32,
                            IsSecurified(false),
                        )
                        .unwrap(),
                    },
                )
                .await
            }

            mod account {
                use super::*;

                async fn each_factor(
                    network_id: NetworkID,
                    key_kind: CAP26KeyKind,
                ) {
                    for factor_source in FactorSource::sample_all().iter() {
                        test(factor_source, network_id, Account, key_kind).await
                    }
                }

                #[actix_rt::test]
                async fn single_first_account_mainnet_t9n() {
                    each_factor(Mainnet, TransactionSigning).await
                }

                #[actix_rt::test]
                async fn single_first_account_stokenet_t9n() {
                    each_factor(Mainnet, TransactionSigning).await
                }

                #[actix_rt::test]
                async fn single_first_account_mainnet_rola() {
                    each_factor(Mainnet, AuthenticationSigning).await
                }

                #[actix_rt::test]
                async fn single_first_account_stokenet_rola() {
                    each_factor(Stokenet, AuthenticationSigning).await
                }
            }

            mod persona {
                use super::*;

                async fn each_factor(
                    network_id: NetworkID,
                    key_kind: CAP26KeyKind,
                ) {
                    for factor_source in FactorSource::sample_all().iter() {
                        test(factor_source, network_id, Identity, key_kind)
                            .await
                    }
                }

                #[actix_rt::test]
                async fn single_first_persona_mainnet_t9n() {
                    each_factor(Mainnet, TransactionSigning).await
                }

                #[actix_rt::test]
                async fn single_first_persona_stokenet_t9n() {
                    each_factor(Mainnet, TransactionSigning).await
                }

                #[actix_rt::test]
                async fn single_first_persona_mainnet_rola() {
                    each_factor(Mainnet, AuthenticationSigning).await
                }

                #[actix_rt::test]
                async fn single_first_persona_stokenet_rola() {
                    each_factor(Stokenet, AuthenticationSigning).await
                }
            }
        }
    }
}
