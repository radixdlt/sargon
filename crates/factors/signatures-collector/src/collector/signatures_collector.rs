use crate::prelude::*;

use super::{
    signatures_collector_dependencies::*, signatures_collector_preprocessor::*,
    signatures_collector_state::*,
};

use SignaturesCollectingContinuation::*;

/// A coordinator which gathers signatures from several factor sources of different
/// kinds, in decreasing friction order, for many transactions and for
/// potentially multiple entities and for many factor instances (derivation paths)
/// for each transaction.
///
/// By decreasing friction order we mean, the quickest and easiest to use FactorSourceKind
/// is last; namely `DeviceFactorSource`, and the most tedious FactorSourceKind is
/// first; namely `LedgerFactorSource`, which user might also lack access to.
pub struct SignaturesCollector<S: Signable> {
    /// Stateless immutable values used by the collector to gather signatures
    /// from factor sources.
    dependencies: SignaturesCollectorDependencies<S>,

    /// Mutable internal state of the collector which builds up the list
    /// of signatures from each used factor source.
    state: RwLock<SignaturesCollectorState<S>>,
}

pub struct NoCrossRoleSkipOutcomeAnalyzer<ID: SignableID>;
impl NoCrossRoleSkipOutcomeAnalyzer {
    pub fn new() -> Arc<dyn CrossRoleSkipOutcomeAnalyzer<ID>> {
        Arc::new(Self)
    }
}
impl CrossRoleSkipOutcomeAnalyzer for NoCrossRoleSkipOutcomeAnalyzer {
    fn invalid_transaction_if_neglected_factors(
        &self,
        signable: ID,
        skipped_factor_source_ids: IndexSet<FactorSourceIDFromHash>,
        petitions: Vec<&PetitionForEntity<ID>>,
    ) -> Option<InvalidTransactionIfNeglected<ID>> {
        None
    }
}

// === PUBLIC ===
impl<S: Signable> SignaturesCollector<S> {
    pub fn new<P: GetEntityByAddress + HasFactorSources>(
        finish_early_strategy: SigningFinishEarlyStrategy,
        transactions: impl IntoIterator<Item = S>,
        interactor: Arc<dyn SignInteractor<S>>,
        proto_profile: &P,
        purpose: SigningPurpose,
    ) -> Result<Self> {
        Self::with_cross_role_skip_outcome_analyzer(
            finish_early_strategy,
            transactions,
            interactor,
            NoCrossRoleSkipOutcomeAnalyzer::new(),
            proto_profile,
            purpose,
        )
    }

    pub fn with_cross_role_skip_outcome_analyzer<
        P: GetEntityByAddress + HasFactorSources,
    >(
        finish_early_strategy: SigningFinishEarlyStrategy,
        transactions: impl IntoIterator<Item = S>,
        interactor: Arc<dyn SignInteractor<S>>,
        cross_role_skip_outcome_analyzer: Arc<
            dyn CrossRoleSkipOutcomeAnalyzer<S::ID>,
        >,
        proto_profile: &P,
        purpose: SigningPurpose,
    ) -> Result<Self> {
        Self::with_signers_extraction(
            finish_early_strategy,
            proto_profile.factor_sources(),
            transactions,
            interactor,
            cross_role_skip_outcome_analyzer,
            purpose,
            |i| {
                SignableWithEntities::extracting_from_profile(&i, proto_profile)
            },
        )
    }

    /// Used by internally by `::new` constructor but made public
    /// so that `SigningManager` can use it.
    pub fn with(
        finish_early_strategy: SigningFinishEarlyStrategy,
        factor_sources: IndexSet<FactorSource>,
        transactions: IdentifiedVecOf<SignableWithEntities<S>>,
        interactor: Arc<dyn SignInteractor<S>>,
        cross_role_skip_outcome_analyzer: Arc<
            dyn CrossRoleSkipOutcomeAnalyzer<S::ID>,
        >,
        purpose: SigningPurpose,
    ) -> Self {
        debug!("Init SignaturesCollector");

        let preprocessor = SignaturesCollectorPreprocessor::new(transactions);

        let (petitions, factors) =
            preprocessor.preprocess(factor_sources, purpose);

        let dependencies = SignaturesCollectorDependencies::new(
            finish_early_strategy,
            interactor,
            cross_role_skip_outcome_analyzer,
            factors,
        );
        let state = SignaturesCollectorState::new(petitions);

        Self {
            dependencies,
            state: RwLock::new(state),
        }
    }

    pub async fn collect_signatures(self) -> Result<SignaturesOutcome<S::ID>> {
        self.sign_with_factors() // in decreasing "friction order"
            .await
            .inspect_err(|e| {
                error!("Failed to use factor sources: {:#?}", e)
            })?;

        Ok(self.outcome())
    }
}

// === INTERNAL ===
impl<S: Signable> SignaturesCollector<S> {
    pub(crate) fn with_signers_extraction<F>(
        finish_early_strategy: SigningFinishEarlyStrategy,
        all_factor_sources_in_profile: IndexSet<FactorSource>,
        transactions: impl IntoIterator<Item = S>,
        interactor: Arc<dyn SignInteractor<S>>,
        cross_role_skip_outcome_analyzer: Arc<
            dyn CrossRoleSkipOutcomeAnalyzer<S::ID>,
        >,
        purpose: SigningPurpose,
        extract_signers: F,
    ) -> Result<Self>
    where
        F: Fn(S) -> Result<SignableWithEntities<S>>,
    {
        let transactions = transactions
            .into_iter()
            .map(extract_signers)
            .collect::<Result<IdentifiedVecOf<SignableWithEntities<S>>>>(
        )?;

        let collector = Self::with(
            finish_early_strategy,
            all_factor_sources_in_profile,
            transactions,
            interactor,
            cross_role_skip_outcome_analyzer,
            purpose,
        );

        Ok(collector)
    }
}

// === PRIVATE ===
impl<S: Signable> SignaturesCollector<S> {
    /// Returning `Continue` means that we should continue collecting signatures.
    ///
    /// Returning `FinishEarly` if it is meaningless to continue collecting signatures,
    /// either since all transactions are valid and this collector is configured
    /// to finish early in that case, or if some transaction is invalid and this
    /// collector is configured to finish early in that case.
    ///
    /// N.B. this method does not concern itself with how many or which
    /// factor sources are left to sign with, that is handled by the main loop,
    /// i.e. this might return `Continue` even though there is not factor sources
    /// left to sign with.
    fn continuation(&self) -> SignaturesCollectingContinuation {
        let finish_early_strategy = self.dependencies.finish_early_strategy;
        let when_all_transactions_are_valid =
            finish_early_strategy.when_all_transactions_are_valid.0;
        let when_some_transaction_is_invalid =
            finish_early_strategy.when_some_transaction_is_invalid.0;

        let petitions_status = self
            .state
            .read()
            .expect("SignaturesCollector lock should not have been poisoned.")
            .petitions
            .read()
            .expect(
                "SignaturesCollectorState lock should not have been poisoned.",
            )
            .status();

        if petitions_status.are_all_valid() {
            if when_all_transactions_are_valid == FinishEarly {
                info!("All valid && should finish early => finish early");
                return FinishEarly;
            } else {
                debug!(
                    "All valid, BUT the collector is configured to NOT finish early => Continue"
                );
            }
        } else if petitions_status.is_some_invalid() {
            if when_some_transaction_is_invalid == FinishEarly {
                info!("Some invalid && should finish early => finish early");
                return FinishEarly;
            } else {
                debug!("Some transactions invalid, BUT the collector is configured to NOT finish early in case of failures => Continue");
            }
        }

        Continue
    }

    fn should_neglect_factors_due_to_irrelevant(
        &self,
        factor_sources_of_kind: &FactorSourcesOfKind,
    ) -> bool {
        let state = self
            .state
            .read()
            .expect("SignaturesCollector lock should not have been poisoned.");
        let petitions = state.petitions.read().expect(
            "SignaturesCollectorState lock should not have been poisoned.",
        );
        petitions
            .should_neglect_factors_due_to_irrelevant(factor_sources_of_kind)
    }

    fn neglected_factors_due_to_irrelevant(
        &self,
        factor_sources_of_kind: &FactorSourcesOfKind,
    ) -> bool {
        if self.should_neglect_factors_due_to_irrelevant(factor_sources_of_kind)
        {
            info!(
                "Neglecting all factors of kind: {} since they are all irrelevant (all TX referencing those factors have already failed)",
                factor_sources_of_kind.kind
            );
            self.process_batch_response(SignResponse::irrelevant(
                factor_sources_of_kind,
            ));
            true
        } else {
            false
        }
    }

    async fn sign_with_factors_of_kind(
        &self,
        factor_sources_of_kind: &FactorSourcesOfKind,
    ) -> Result<()> {
        debug!(
            "Use(?) #{:?} factors of kind: {:?}",
            &factor_sources_of_kind.factor_sources().len(),
            &factor_sources_of_kind.kind
        );

        if factor_sources_of_kind.kind.support_poly_sign() {
            self.sign_poly(factor_sources_of_kind).await
        } else {
            self.sign_mono(factor_sources_of_kind).await
        }
    }

    async fn sign_poly(
        &self,
        factor_sources_of_kind: &FactorSourcesOfKind,
    ) -> Result<()> {
        debug!("Creating poly request for interactor");
        let request = self.request_for_poly_sign(factor_sources_of_kind);

        let invalid_transactions =
            request.invalid_transactions_if_all_factors_neglected();
        if !invalid_transactions.is_empty() {
            info!(
                "If factors {:?} are neglected, invalid TXs: {:?}",
                request.per_factor_source.keys(),
                invalid_transactions
            )
        }
        debug!("Dispatching poly request to interactor: {:?}", request);
        let response = self.dependencies.interactor.sign(request).await?;
        debug!("Got response from poly interactor: {:?}", response);
        self.process_batch_response(response);

        Ok(())
    }

    async fn sign_mono(
        &self,
        factor_sources_of_kind: &FactorSourcesOfKind,
    ) -> Result<()> {
        let factor_sources = factor_sources_of_kind.factor_sources();
        for factor_source in factor_sources {
            // Prepare the request for the interactor
            debug!("Creating mono request for interactor");
            let factor_source_id =
                factor_source.factor_source_id().as_hash().cloned().expect(
                    "Signature Collector only works with HD FactorSources.",
                );

            if let Some(request) = self.request_for_mono_sign(
                factor_sources_of_kind.kind,
                &factor_source_id,
            ) {
                let invalid_transactions = request
                    .invalid_transactions_if_factor_neglected(
                        &factor_source_id,
                    );
                if !invalid_transactions.is_empty() {
                    info!(
                        "If factor {:?} are neglected, invalid TXs: {:?}",
                        factor_source_id, invalid_transactions
                    )
                }

                debug!("Dispatching mono request to interactor: {:?}", request);
                // Produce the results from the interactor
                let response =
                    self.dependencies.interactor.sign(request).await?;
                debug!("Got response from mono interactor: {:?}", response);

                // Report the results back to the collector
                self.process_batch_response(response);

                if self.continuation() == FinishEarly {
                    break;
                }
            }
        }

        Ok(())
    }

    /// In decreasing "friction order"
    async fn sign_with_factors(&self) -> Result<()> {
        let factors_of_kind = self.dependencies.factors_of_kind.clone();
        for factor_sources_of_kind in factors_of_kind.iter() {
            if self.continuation() == FinishEarly {
                break;
            }
            if self.neglected_factors_due_to_irrelevant(factor_sources_of_kind)
            {
                continue;
            }
            self.sign_with_factors_of_kind(factor_sources_of_kind)
                .await?;
        }
        info!("FINISHED WITH ALL FACTORS");
        Ok(())
    }

    fn per_transaction_input(
        &self,
        factor_source_id: &FactorSourceIDFromHash,
    ) -> IndexSet<TransactionSignRequestInput<S>> {
        self.state
            .read()
            .expect("SignaturesCollector lock should not have been poisoned.")
            .petitions
            .read()
            .expect(
                "SignaturesCollectorState lock should not have been poisoned.",
            )
            .per_transaction_input(factor_source_id)
    }

    fn request_for_mono_sign(
        &self,
        factor_source_kind: FactorSourceKind,
        factor_source_id: &FactorSourceIDFromHash,
    ) -> Option<SignRequest<S>> {
        let per_transaction = self.per_transaction_input(factor_source_id);
        if per_transaction.is_empty() {
            return None;
        }

        let invalid_transactions_if_neglected = self
            .invalid_transactions_if_neglected_factor_sources(
                self.dependencies.cross_role_skip_outcome_analyzer.clone(),
                IndexSet::just(*factor_source_id),
            )
            .into_iter()
            .collect::<IndexSet<_>>();

        let per_factor_source_input = PerFactorSourceInput::new(
            *factor_source_id,
            per_transaction,
            invalid_transactions_if_neglected,
        );

        Some(SignRequest::new(
            factor_source_kind,
            IndexMap::just((*factor_source_id, per_factor_source_input)),
        ))
    }

    fn request_for_poly_sign(
        &self,
        factor_sources_of_kind: &FactorSourcesOfKind,
    ) -> SignRequest<S> {
        let factor_source_ids = factor_sources_of_kind
            .factor_sources()
            .iter()
            .map(|f| {
                *f.factor_source_id().as_hash().expect(
                    "Signature Collector only works with HD FactorSources.",
                )
            })
            .collect::<IndexSet<FactorSourceIDFromHash>>();

        let invalid_transactions_if_neglected = self
            .invalid_transactions_if_neglected_factor_sources(
                self.dependencies.cross_role_skip_outcome_analyzer.clone(),
                factor_source_ids.clone(),
            );

        let per_factor_source = factor_source_ids.iter().map(|id| {
            let per_transaction = self.per_transaction_input(id);

            (*id, PerFactorSourceInput::new(
                *id,
                per_transaction,
                invalid_transactions_if_neglected.clone(),
            ))
        }).collect::<IndexMap<FactorSourceIDFromHash, PerFactorSourceInput<S>>>();

        // Prepare the request for the interactor
        SignRequest::new(factor_sources_of_kind.kind, per_factor_source)
    }

    fn invalid_transactions_if_neglected_factor_sources(
        &self,
        cross_role_skip_outcome_analyzer: Arc<
            dyn CrossRoleSkipOutcomeAnalyzer<S::ID>,
        >,
        factor_source_ids: IndexSet<FactorSourceIDFromHash>,
    ) -> IndexSet<InvalidTransactionIfNeglected<S::ID>> {
        self.state
            .read()
            .expect("SignaturesCollector lock should not have been poisoned.")
            .petitions
            .read()
            .expect(
                "SignaturesCollectorState lock should not have been poisoned.",
            )
            .invalid_transactions_if_neglected_factors(
                self.cross_role_skip_outcome_analyzer,
                factor_source_ids,
            )
    }

    fn process_batch_response(&self, response: SignResponse<S::ID>) {
        let state = self
            .state
            .write()
            .expect("SignaturesCollector lock should not have been poisoned.");
        let petitions = state.petitions.write().expect(
            "SignaturesCollectorState lock should not have been poisoned.",
        );
        petitions.process_batch_response(response)
    }

    fn outcome(self) -> SignaturesOutcome<S::ID> {
        let expected_number_of_transactions;
        {
            let state = self.state.write().expect(
                "SignaturesCollector lock should not have been poisoned.",
            );
            let petitions = state.petitions.write().expect(
                "SignaturesCollectorState lock should not have been poisoned.",
            );
            expected_number_of_transactions = petitions
                .txid_to_petition
                .read()
                .expect("Petitions lock is poisoned")
                .len();
        }
        let outcome = self
            .state
            .read()
            .expect("SignaturesCollector lock should not have been poisoned.")
            .petitions
            .read()
            .expect(
                "SignaturesCollectorState lock should not have been poisoned.",
            )
            .outcome();
        assert_eq!(
            outcome.failed_transactions().len()
                + outcome.successful_transactions().len(),
            expected_number_of_transactions
        );
        if !outcome.successful() {
            warn!(
                "Failed to sign, invalid tx: {:?}, petition",
                outcome.failed_transactions()
            )
        }
        outcome
    }
}

pub fn assert_petition<S: Signable>(
    petitions: &Petitions<S>,
    t: &S,
    threshold_factors: HashMap<
        AddressOfAccountOrPersona,
        HashSet<FactorSourceIDFromHash>,
    >,
    override_factors: HashMap<
        AddressOfAccountOrPersona,
        HashSet<FactorSourceIDFromHash>,
    >,
) {
    let petitions_ref = petitions
        .txid_to_petition
        .read()
        .expect("Petitions lock should not have been poisoned");
    let signable_id = t.get_id();
    let petition = petitions_ref.get(&signable_id).unwrap();
    assert_eq!(petition.signable.get_id(), signable_id);

    let mut addresses = threshold_factors.keys().collect::<HashSet<_>>();
    addresses.extend(override_factors.keys().collect::<HashSet<_>>());

    assert_eq!(
        petition
            .for_entities
            .read()
            .expect(
                "PetitionForTransaction lock should not have been poisoned."
            )
            .keys()
            .collect::<HashSet<_>>(),
        addresses
    );

    assert!(petition
        .for_entities
        .read()
        .expect("PetitionForTransaction lock should not have been poisoned.")
        .iter()
        .all(|(a, p)| { p.entity == *a }));

    assert!(petition
        .for_entities
        .read()
        .expect("PetitionForTransaction lock should not have been poisoned.")
        .iter()
        .all(|(_, p)| { p.payload_id == t.get_id() }));

    for (k, v) in petition
        .for_entities
        .read()
        .expect("PetitionForTransaction lock should not have been poisoned.")
        .iter()
    {
        let threshold = threshold_factors.get(k);
        if let Some(actual_threshold) = &v.threshold_factors {
            let threshold = threshold.unwrap().clone();
            assert_eq!(
                actual_threshold
                    .read()
                    .expect(
                        "PetitionForEntity lock should not have been poisoned."
                    )
                    .factor_instances()
                    .into_iter()
                    .map(|f| f.factor_source_id)
                    .collect::<HashSet<_>>(),
                threshold
            );
        } else {
            assert!(threshold.is_none());
        }

        let override_ = override_factors.get(k);
        if let Some(actual_override) = &v.override_factors {
            let override_ = override_.unwrap().clone();
            assert_eq!(
                actual_override
                    .read()
                    .expect(
                        "PetitionForEntity lock should not have been poisoned."
                    )
                    .factor_instances()
                    .into_iter()
                    .map(|f| f.factor_source_id)
                    .collect::<HashSet<_>>(),
                override_
            );
        } else {
            assert!(override_.is_none());
        }
    }
}

#[cfg(debug_assertions)]
impl<S: Signable> SignaturesCollector<S> {
    /// Used by tests
    pub fn petitions(self) -> Petitions<S> {
        self.state
            .read()
            .expect("SignaturesCollector lock should not have been poisoned.")
            .petitions
            .read()
            .expect(
                "SignaturesCollectorState lock should not have been poisoned.",
            )
            .clone()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn factor_source_kinds_order() {
        let kinds = FactorSource::sample_all()
            .into_iter()
            .map(|f| f.factor_source_kind())
            .collect::<IndexSet<_>>();
        let mut kinds = kinds.into_iter().collect_vec();
        kinds.sort();
        let kinds = kinds.into_iter().collect::<IndexSet<_>>();
        assert_eq!(
            kinds,
            IndexSet::<FactorSourceKind>::from_iter([
                FactorSourceKind::LedgerHQHardwareWallet,
                FactorSourceKind::ArculusCard,
                FactorSourceKind::Password,
                FactorSourceKind::SecurityQuestions,
                FactorSourceKind::OffDeviceMnemonic,
                FactorSourceKind::Device,
            ])
        )
    }

    mod single_tx {
        use super::*;

        mod multiple_entities {
            use super::*;

            #[actix_rt::test]
            async fn prudent_user_single_tx_two_accounts_same_factor_source() {
                let collector = SignaturesCollector::test_prudent([
                    SignableWithEntities::<TransactionIntent>::sample([
                        Account::sample_unsecurified_mainnet(
                            "A0",
                            HierarchicalDeterministicFactorInstance::new_for_entity(
                                FactorSourceIDFromHash::sample_at(0),
                                CAP26EntityKind::Account,
                                Hardened::from_local_key_space(U31::ZERO, IsSecurified(false)).unwrap(),
                            ),
                        ),
                        Account::sample_unsecurified_mainnet(
                            "A1",
                            HierarchicalDeterministicFactorInstance::new_for_entity(
                                FactorSourceIDFromHash::sample_at(0),
                                CAP26EntityKind::Account,
                                Hardened::from_local_key_space(U31::ONE, IsSecurified(false)).unwrap(),
                            ),
                        ),
                    ])
                ]);

                let outcome = collector.collect_signatures().await.unwrap();
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 2);
                assert_eq!(
                    signatures
                        .into_iter()
                        .map(|s| s.derivation_path())
                        .collect::<HashSet<_>>(),
                    [
                        DerivationPath::from(AccountPath::new(
                            NetworkID::Mainnet,
                            CAP26KeyKind::TransactionSigning,
                            Hardened::from_local_key_space(
                                U31::ZERO,
                                IsSecurified(false)
                            )
                            .unwrap()
                        )),
                        DerivationPath::from(AccountPath::new(
                            NetworkID::Mainnet,
                            CAP26KeyKind::TransactionSigning,
                            Hardened::from_local_key_space(
                                U31::ONE,
                                IsSecurified(false)
                            )
                            .unwrap()
                        )),
                    ]
                    .into_iter()
                    .collect::<HashSet<_>>()
                )
            }

            #[actix_rt::test]
            async fn prudent_user_single_tx_two_accounts_different_factor_sources(
            ) {
                let collector = SignaturesCollector::test_prudent([
                    SignableWithEntities::<TransactionIntent>::sample([
                        Account::sample_at(0),
                        Account::sample_at(1),
                    ]),
                ]);

                let outcome = collector.collect_signatures().await.unwrap();
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 2);
            }
        }

        mod single_entity {
            use super::*;
            use std::any::TypeId;

            fn sample_at<E: IsEntity + 'static>(
                index: usize,
            ) -> AccountOrPersona {
                if TypeId::of::<Account>() == TypeId::of::<E>() {
                    AccountOrPersona::AccountEntity(Account::sample_at(index))
                } else {
                    AccountOrPersona::PersonaEntity(Persona::sample_at(index))
                }
            }

            fn sample_securified_mainnet<E: IsEntity + 'static>(
                name: impl AsRef<str>,
                rola_index: u32,
                veci: HierarchicalDeterministicFactorInstance,
                make_role: impl Fn() -> GeneralRoleWithHierarchicalDeterministicFactorInstances,
            ) -> AccountOrPersona {
                if TypeId::of::<Account>() == TypeId::of::<E>() {
                    AccountOrPersona::from(Account::sample_securified_mainnet(
                        name, rola_index, veci, make_role,
                    ))
                } else {
                    AccountOrPersona::from(Persona::sample_securified_mainnet(
                        name, rola_index, veci, make_role,
                    ))
                }
            }

            trait EntityTXSigningFI {
                fn transaction_signing_factor_instances(
                    &self,
                ) -> IndexSet<FactorInstance>;
            }

            impl EntityTXSigningFI for AccountOrPersona {
                fn transaction_signing_factor_instances(
                    &self,
                ) -> IndexSet<FactorInstance> {
                    let sec_state: EntitySecurityState = match self {
                        AccountOrPersona::AccountEntity(account) => {
                            account.security_state.clone()
                        }
                        AccountOrPersona::PersonaEntity(persona) => {
                            persona.security_state.clone()
                        }
                    };

                    match sec_state {
                        EntitySecurityState::Unsecured { value } => {
                            IndexSet::from_iter([value
                                .transaction_signing
                                .factor_instance()])
                        }
                        EntitySecurityState::Securified { value } => value
                            .security_structure
                            .matrix_of_factors
                            .all_factors()
                            .into_iter()
                            .cloned()
                            .collect::<IndexSet<_>>(),
                    }
                }
            }

            async fn prudent_user_single_tx_e0<E: IsEntity + 'static>() {
                let collector = SignaturesCollector::test_prudent([
                    SignableWithEntities::<TransactionIntent>::sample([
                        sample_at::<E>(0),
                    ]),
                ]);
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 1);
            }

            async fn prudent_user_single_tx_e0_assert_correct_intent_hash_is_signed<
                E: IsEntity + 'static,
            >() {
                let sample = sample_at::<E>(0);
                let tx = SignableWithEntities::<TransactionIntent>::sample([
                    sample.clone(),
                ]);
                let collector = SignaturesCollector::test_prudent([tx.clone()]);
                let signature = &collector
                    .collect_signatures()
                    .await
                    .unwrap()
                    .all_signatures()[0];
                assert_eq!(
                    signature.payload_id(),
                    &tx.signable.transaction_intent_hash()
                );

                if sample.is_account_entity() {
                    assert_eq!(
                        signature
                            .derivation_path()
                            .as_account()
                            .unwrap()
                            .get_entity_kind(),
                        CAP26EntityKind::Account
                    );
                } else {
                    assert_eq!(
                        signature
                            .derivation_path()
                            .as_identity()
                            .unwrap()
                            .get_entity_kind(),
                        CAP26EntityKind::Identity
                    );
                }
            }

            async fn prudent_user_single_tx_e0_assert_correct_owner_has_signed<
                E: IsEntity + 'static,
            >() {
                let entity = sample_at::<E>(0);
                let tx = SignableWithEntities::<TransactionIntent>::sample([
                    entity.clone(),
                ]);
                let collector = SignaturesCollector::test_prudent([tx.clone()]);
                let signature = &collector
                    .collect_signatures()
                    .await
                    .unwrap()
                    .all_signatures()[0];
                assert_eq!(
                    signature.owned_factor_instance().owner,
                    entity.address()
                );
            }

            async fn prudent_user_single_tx_e0_assert_correct_owner_factor_instance_signed<
                E: IsEntity + 'static,
            >() {
                let entity = sample_at::<E>(0);
                let tx = SignableWithEntities::<TransactionIntent>::sample([
                    entity.clone(),
                ]);
                let collector = SignaturesCollector::test_prudent([tx.clone()]);
                let signature = &collector
                    .collect_signatures()
                    .await
                    .unwrap()
                    .all_signatures()[0];

                assert_eq!(
                    signature
                        .owned_factor_instance()
                        .factor_instance()
                        .factor_instance(),
                    entity
                        .transaction_signing_factor_instances()
                        .first()
                        .unwrap()
                        .clone()
                );
            }

            async fn prudent_user_single_tx_e1<E: IsEntity + 'static>() {
                let collector = SignaturesCollector::test_prudent([
                    SignableWithEntities::<TransactionIntent>::sample([
                        sample_at::<E>(1),
                    ]),
                ]);
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 1);
            }

            async fn prudent_user_single_tx_e2<E: IsEntity + 'static>() {
                let collector = SignaturesCollector::test_prudent([
                    SignableWithEntities::<TransactionIntent>::sample([
                        sample_at::<E>(2),
                    ]),
                ]);
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 1);
            }

            async fn prudent_user_single_tx_e3<E: IsEntity + 'static>() {
                let collector = SignaturesCollector::test_prudent([
                    SignableWithEntities::<TransactionIntent>::sample([
                        sample_at::<E>(3),
                    ]),
                ]);
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 1);
            }

            async fn prudent_user_single_tx_e4<E: IsEntity + 'static>() {
                let collector = SignaturesCollector::test_prudent([
                    SignableWithEntities::<TransactionIntent>::sample([
                        sample_at::<E>(4),
                    ]),
                ]);
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 2);
            }

            async fn prudent_user_single_tx_e5<E: IsEntity + 'static>() {
                let collector = SignaturesCollector::test_prudent([
                    SignableWithEntities::<TransactionIntent>::sample([
                        sample_at::<E>(5),
                    ]),
                ]);
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 1);
            }

            async fn prudent_user_single_tx_e6<E: IsEntity + 'static>() {
                let collector = SignaturesCollector::test_prudent([
                    SignableWithEntities::<TransactionIntent>::sample([
                        sample_at::<E>(6),
                    ]),
                ]);
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 1);
            }

            async fn prudent_user_single_tx_e7<E: IsEntity + 'static>() {
                let collector = SignaturesCollector::test_prudent([
                    SignableWithEntities::<TransactionIntent>::sample([
                        sample_at::<E>(7),
                    ]),
                ]);
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();

                assert_eq!(signatures.len(), 5);
            }

            async fn lazy_sign_minimum_user_single_tx_e0<
                E: IsEntity + 'static,
            >() {
                let collector =
                    SignaturesCollector::test_lazy_sign_minimum_no_failures([
                        SignableWithEntities::<TransactionIntent>::sample([
                            sample_at::<E>(0),
                        ]),
                    ]);
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 1);
            }

            async fn lazy_sign_minimum_user_single_tx_e1<
                E: IsEntity + 'static,
            >() {
                let collector =
                    SignaturesCollector::test_lazy_sign_minimum_no_failures([
                        SignableWithEntities::<TransactionIntent>::sample([
                            sample_at::<E>(1),
                        ]),
                    ]);
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 1);
            }

            async fn lazy_sign_minimum_user_single_tx_e2<
                E: IsEntity + 'static,
            >() {
                let collector =
                    SignaturesCollector::test_lazy_sign_minimum_no_failures([
                        SignableWithEntities::<TransactionIntent>::sample([
                            sample_at::<E>(2),
                        ]),
                    ]);
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 1);
            }

            async fn lazy_sign_minimum_user_e3<E: IsEntity + 'static>() {
                let collector =
                    SignaturesCollector::test_lazy_sign_minimum_no_failures([
                        SignableWithEntities::<TransactionIntent>::sample([
                            sample_at::<E>(3),
                        ]),
                    ]);
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 1);
            }

            async fn lazy_sign_minimum_user_e4<E: IsEntity + 'static>() {
                let collector =
                    SignaturesCollector::test_lazy_sign_minimum_no_failures([
                        SignableWithEntities::<TransactionIntent>::sample([
                            sample_at::<E>(4),
                        ]),
                    ]);
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 2);
            }

            async fn lazy_sign_minimum_user_e5<E: IsEntity + 'static>() {
                let collector =
                    SignaturesCollector::test_lazy_sign_minimum_no_failures([
                        SignableWithEntities::<TransactionIntent>::sample([
                            sample_at::<E>(5),
                        ]),
                    ]);
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 1);
            }

            async fn lazy_sign_minimum_user_e6<E: IsEntity + 'static>() {
                let collector =
                    SignaturesCollector::test_lazy_sign_minimum_no_failures([
                        SignableWithEntities::<TransactionIntent>::sample([
                            sample_at::<E>(6),
                        ]),
                    ]);
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();

                assert_eq!(signatures.len(), 2);
            }

            async fn lazy_sign_minimum_user_e7<E: IsEntity + 'static>() {
                let collector =
                    SignaturesCollector::test_lazy_sign_minimum_no_failures([
                        SignableWithEntities::<TransactionIntent>::sample([
                            sample_at::<E>(7),
                        ]),
                    ]);
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();

                assert_eq!(signatures.len(), 5);
            }

            async fn lazy_sign_minimum_user_e5_last_factor_used<
                E: IsEntity + 'static,
            >() {
                let entity = sample_at::<E>(5);
                let collector =
                    SignaturesCollector::test_lazy_sign_minimum_no_failures([
                        SignableWithEntities::<TransactionIntent>::sample([
                            entity.clone(),
                        ]),
                    ]);
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(signatures.len(), 1);

                let signature = &signatures[0];

                assert_eq!(
                    signature
                        .owned_factor_instance()
                        .factor_instance()
                        .factor_source_id,
                    FactorSourceIDFromHash::sample_at(4)
                );

                assert_eq!(
                    outcome.ids_of_neglected_factor_sources(),
                    IndexSet::just(FactorSourceIDFromHash::sample_at(1))
                )
            }

            async fn lazy_sign_minimum_all_known_factors_used_as_override_factors_signed_with_device_for_entity<
                E: IsEntity + 'static,
            >() {
                let collector =
                    SignaturesCollector::test_lazy_sign_minimum_no_failures([
                        SignableWithEntities::<TransactionIntent>::sample([
                            sample_securified_mainnet::<E>(
                                "Alice",
                                0,
                                if E::entity_kind() == CAP26EntityKind::Identity
                                {
                                    HierarchicalDeterministicFactorInstance::sample_fii10()
                                } else {
                                    HierarchicalDeterministicFactorInstance::sample_fia10()
                                },
                                || {
                                    GeneralRoleWithHierarchicalDeterministicFactorInstances::with_factors_and_role(
                                    RoleKind::Primary, [], 0,
                                    FactorSource::sample_all().into_iter().map(|f| {
                                        HierarchicalDeterministicFactorInstance::new_for_entity(
                                            *f.factor_source_id().as_hash().unwrap(),
                                             E::entity_kind(),
                                            Hardened::from_local_key_space(U31::ZERO, IsSecurified(true)).unwrap(),
                                        )
                                    }),
                                ).unwrap()
                                },
                            ),
                        ]),
                    ]);
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(outcome.successful());
                let signatures = outcome.all_signatures();
                assert_eq!(
                    signatures.len(),
                    if FactorSourceKind::Device.support_poly_sign() {
                        2
                    } else {
                        1
                    }
                );

                assert!(signatures
                    .into_iter()
                    .all(|s| s.factor_source_id().kind
                        == FactorSourceKind::Device));
            }

            async fn lazy_always_skip_user_single_tx_e0<
                E: IsEntity + 'static,
            >() {
                let collector = SignaturesCollector::test_lazy_always_skip([
                    SignableWithEntities::<TransactionIntent>::sample([
                        sample_at::<E>(0),
                    ]),
                ]);
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(!outcome.successful());
                let signatures = outcome.all_signatures();
                assert!(signatures.is_empty());
            }

            async fn fail_get_neglected_e0<E: IsEntity + 'static>() {
                let failing =
                    IndexSet::<_>::just(FactorSourceIDFromHash::sample_at(0));
                let collector = SignaturesCollector::test_prudent_with_failures(
                    [SignableWithEntities::<TransactionIntent>::sample([
                        sample_at::<E>(0),
                    ])],
                    SimulatedFailures::with_simulated_failures(failing.clone()),
                );
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(!outcome.successful());
                let neglected = outcome.ids_of_neglected_factor_sources();
                assert_eq!(neglected, failing);
            }

            async fn lazy_always_skip_user_single_tx_e1<
                E: IsEntity + 'static,
            >() {
                let collector = SignaturesCollector::test_lazy_always_skip([
                    SignableWithEntities::<TransactionIntent>::sample([
                        sample_at::<E>(1),
                    ]),
                ]);
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(!outcome.successful());
                let signatures = outcome.all_signatures();
                assert!(signatures.is_empty());
            }

            async fn lazy_always_skip_user_single_tx_e2<
                E: IsEntity + 'static,
            >() {
                let collector = SignaturesCollector::test_lazy_always_skip([
                    SignableWithEntities::<TransactionIntent>::sample([
                        sample_at::<E>(2),
                    ]),
                ]);
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(!outcome.successful());
                let signatures = outcome.all_signatures();
                assert!(signatures.is_empty());
            }

            async fn lazy_always_skip_user_e3<E: IsEntity + 'static>() {
                let collector = SignaturesCollector::test_lazy_always_skip([
                    SignableWithEntities::<TransactionIntent>::sample([
                        sample_at::<E>(3),
                    ]),
                ]);
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(!outcome.successful());
                let signatures = outcome.all_signatures();
                assert!(signatures.is_empty());
            }

            async fn lazy_always_skip_user_e4<E: IsEntity + 'static>() {
                let collector = SignaturesCollector::test_lazy_always_skip([
                    SignableWithEntities::<TransactionIntent>::sample([
                        sample_at::<E>(4),
                    ]),
                ]);
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(!outcome.successful());
                let signatures = outcome.all_signatures();
                assert!(signatures.is_empty());
            }

            async fn lazy_always_skip_user_e5<E: IsEntity + 'static>() {
                let collector = SignaturesCollector::test_lazy_always_skip([
                    SignableWithEntities::<TransactionIntent>::sample([
                        sample_at::<E>(5),
                    ]),
                ]);
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(!outcome.successful());
                let signatures = outcome.all_signatures();
                assert!(signatures.is_empty());
            }

            async fn lazy_always_skip_user_e6<E: IsEntity + 'static>() {
                let collector = SignaturesCollector::test_lazy_always_skip([
                    SignableWithEntities::<TransactionIntent>::sample([
                        sample_at::<E>(6),
                    ]),
                ]);
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(!outcome.successful());
                let signatures = outcome.all_signatures();
                assert!(signatures.is_empty());
            }

            async fn lazy_always_skip_user_e7<E: IsEntity + 'static>() {
                let collector = SignaturesCollector::test_lazy_always_skip([
                    SignableWithEntities::<TransactionIntent>::sample([
                        sample_at::<E>(7),
                    ]),
                ]);
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(!outcome.successful());
                let signatures = outcome.all_signatures();
                assert!(signatures.is_empty());
            }

            async fn failure_e0<E: IsEntity + 'static>() {
                let collector = SignaturesCollector::test_prudent_with_failures(
                    [SignableWithEntities::<TransactionIntent>::sample([
                        sample_at::<E>(0),
                    ])],
                    SimulatedFailures::with_simulated_failures([
                        FactorSourceIDFromHash::sample_at(0),
                    ]),
                );
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(!outcome.successful());
                assert_eq!(
                    outcome
                        .ids_of_neglected_factor_sources_failed()
                        .into_iter()
                        .collect_vec(),
                    vec![FactorSourceIDFromHash::sample_at(0)]
                );
                assert!(outcome
                    .ids_of_neglected_factor_sources_skipped_by_user()
                    .is_empty())
            }

            async fn failure_e5<E: IsEntity + 'static>() {
                let collector = SignaturesCollector::new_test(
                    SigningFinishEarlyStrategy::r#continue(),
                    FactorSource::sample_all(),
                    [SignableWithEntities::<TransactionIntent>::sample([
                        sample_at::<E>(5),
                    ])],
                    SimulatedUser::prudent_with_failures(
                        SimulatedFailures::with_simulated_failures([
                            FactorSourceIDFromHash::sample_at(4),
                        ]),
                    ),
                    SigningPurpose::sign_transaction_primary(),
                );

                let outcome = collector.collect_signatures().await.unwrap();
                assert!(outcome.successful());
                assert_eq!(
                    outcome
                        .ids_of_neglected_factor_sources_failed()
                        .into_iter()
                        .collect_vec(),
                    vec![FactorSourceIDFromHash::sample_at(4)]
                );
                assert!(outcome
                    .ids_of_neglected_factor_sources_skipped_by_user()
                    .is_empty());
            }

            async fn building_can_succeed_even_if_one_factor_source_fails_assert_ids_of_successful_tx_e4<
                E: IsEntity + 'static,
            >() {
                let collector = SignaturesCollector::test_prudent_with_failures(
                    [SignableWithEntities::<TransactionIntent>::sample([
                        sample_at::<E>(4),
                    ])],
                    SimulatedFailures::with_simulated_failures([
                        FactorSourceIDFromHash::sample_at(3),
                    ]),
                );
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(outcome.successful());
                assert_eq!(
                    outcome
                        .signatures_of_successful_transactions()
                        .into_iter()
                        .map(|f| f.factor_source_id())
                        .collect::<IndexSet<_>>(),
                    IndexSet::<_>::from_iter([
                        FactorSourceIDFromHash::sample_at(0),
                        FactorSourceIDFromHash::sample_at(5)
                    ])
                );
            }

            async fn building_can_succeed_even_if_one_factor_source_fails_assert_ids_of_failed_tx_e4<
                E: IsEntity + 'static,
            >() {
                let collector = SignaturesCollector::test_prudent_with_failures(
                    [SignableWithEntities::<TransactionIntent>::sample([
                        sample_at::<E>(4),
                    ])],
                    SimulatedFailures::with_simulated_failures([
                        FactorSourceIDFromHash::sample_at(3),
                    ]),
                );
                let outcome = collector.collect_signatures().await.unwrap();
                assert!(outcome.successful());
                assert_eq!(
                    outcome.ids_of_neglected_factor_sources(),
                    IndexSet::<_>::just(FactorSourceIDFromHash::sample_at(3))
                );
            }

            mod account {
                use super::*;
                type E = Account;

                #[actix_rt::test]
                async fn prudent_user_single_tx_a0() {
                    prudent_user_single_tx_e0::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_a0_assert_correct_intent_hash_is_signed(
                ) {
                    prudent_user_single_tx_e0_assert_correct_intent_hash_is_signed::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_a0_assert_correct_owner_has_signed(
                ) {
                    prudent_user_single_tx_e0_assert_correct_owner_has_signed::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_a0_assert_correct_owner_factor_instance_signed(
                ) {
                    prudent_user_single_tx_e0_assert_correct_owner_factor_instance_signed::<E>()
                        .await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_a1() {
                    prudent_user_single_tx_e1::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_a2() {
                    prudent_user_single_tx_e2::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_a3() {
                    prudent_user_single_tx_e3::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_a4() {
                    prudent_user_single_tx_e4::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_a5() {
                    prudent_user_single_tx_e5::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_a6() {
                    prudent_user_single_tx_e6::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_a7() {
                    prudent_user_single_tx_e7::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_single_tx_a0() {
                    lazy_sign_minimum_user_single_tx_e0::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_single_tx_a1() {
                    lazy_sign_minimum_user_single_tx_e1::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_single_tx_a2() {
                    lazy_sign_minimum_user_single_tx_e2::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_a3() {
                    lazy_sign_minimum_user_e3::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_a4() {
                    lazy_sign_minimum_user_e4::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_a5() {
                    lazy_sign_minimum_user_e5::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_a6() {
                    lazy_sign_minimum_user_e6::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_a7() {
                    lazy_sign_minimum_user_e7::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_a5_last_factor_used() {
                    lazy_sign_minimum_user_e5_last_factor_used::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_all_known_factors_used_as_override_factors_signed_with_device_for_account(
                ) {
                    lazy_sign_minimum_all_known_factors_used_as_override_factors_signed_with_device_for_entity::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_single_tx_a0() {
                    lazy_always_skip_user_single_tx_e0::<E>().await
                }

                #[actix_rt::test]
                async fn fail_get_skipped_a0() {
                    fail_get_neglected_e0::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_single_tx_a1() {
                    lazy_always_skip_user_single_tx_e1::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_single_tx_a2() {
                    lazy_always_skip_user_single_tx_e2::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_a3() {
                    lazy_always_skip_user_e3::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_a4() {
                    lazy_always_skip_user_e4::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_a5() {
                    lazy_always_skip_user_e5::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_a6() {
                    lazy_always_skip_user_e6::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_a7() {
                    lazy_always_skip_user_e7::<E>().await
                }

                #[actix_rt::test]
                async fn failure_a0() {
                    failure_e0::<E>().await
                }

                #[actix_rt::test]
                async fn failure_a5() {
                    failure_e5::<E>().await
                }

                #[actix_rt::test]
                async fn building_can_succeed_even_if_one_factor_source_fails_assert_ids_of_successful_tx(
                ) {
                    building_can_succeed_even_if_one_factor_source_fails_assert_ids_of_successful_tx_e4::<E>()
                        .await
                }

                #[actix_rt::test]
                async fn building_can_succeed_even_if_one_factor_source_fails_assert_ids_of_failed_tx(
                ) {
                    building_can_succeed_even_if_one_factor_source_fails_assert_ids_of_failed_tx_e4::<E>().await
                }
            }

            mod persona {
                use super::*;
                type E = Persona;

                #[actix_rt::test]
                async fn prudent_user_single_tx_p0() {
                    prudent_user_single_tx_e0::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_p0_assert_correct_intent_hash_is_signed(
                ) {
                    prudent_user_single_tx_e0_assert_correct_intent_hash_is_signed::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_p0_assert_correct_owner_has_signed(
                ) {
                    prudent_user_single_tx_e0_assert_correct_owner_has_signed::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_p0_assert_correct_owner_factor_instance_signed(
                ) {
                    prudent_user_single_tx_e0_assert_correct_owner_factor_instance_signed::<E>()
                        .await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_p1() {
                    prudent_user_single_tx_e1::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_p2() {
                    prudent_user_single_tx_e2::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_p3() {
                    prudent_user_single_tx_e3::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_p4() {
                    prudent_user_single_tx_e4::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_p5() {
                    prudent_user_single_tx_e5::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_p6() {
                    prudent_user_single_tx_e6::<E>().await
                }

                #[actix_rt::test]
                async fn prudent_user_single_tx_p7() {
                    prudent_user_single_tx_e7::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_single_tx_p0() {
                    lazy_sign_minimum_user_single_tx_e0::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_single_tx_p1() {
                    lazy_sign_minimum_user_single_tx_e1::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_single_tx_p2() {
                    lazy_sign_minimum_user_single_tx_e2::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_p3() {
                    lazy_sign_minimum_user_e3::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_p4() {
                    lazy_sign_minimum_user_e4::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_p5() {
                    lazy_sign_minimum_user_e5::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_p6() {
                    lazy_sign_minimum_user_e6::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_p7() {
                    lazy_sign_minimum_user_e7::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_user_p5_last_factor_used() {
                    lazy_sign_minimum_user_e5_last_factor_used::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_sign_minimum_all_known_factors_used_as_override_factors_signed_with_device_for_account(
                ) {
                    lazy_sign_minimum_all_known_factors_used_as_override_factors_signed_with_device_for_entity::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_single_tx_p0() {
                    lazy_always_skip_user_single_tx_e0::<E>().await
                }

                #[actix_rt::test]
                async fn fail_get_skipped_p0() {
                    fail_get_neglected_e0::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_single_tx_p1() {
                    lazy_always_skip_user_single_tx_e1::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_single_tx_p2() {
                    lazy_always_skip_user_single_tx_e2::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_p3() {
                    lazy_always_skip_user_e3::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_p4() {
                    lazy_always_skip_user_e4::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_p5() {
                    lazy_always_skip_user_e5::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_p6() {
                    lazy_always_skip_user_e6::<E>().await
                }

                #[actix_rt::test]
                async fn lazy_always_skip_user_p7() {
                    lazy_always_skip_user_e7::<E>().await
                }

                #[actix_rt::test]
                async fn failure_p0() {
                    failure_e0::<E>().await
                }

                #[actix_rt::test]
                async fn failure_p5() {
                    failure_e5::<E>().await
                }

                #[actix_rt::test]
                async fn building_can_succeed_even_if_one_factor_source_fails_assert_ids_of_successful_tx(
                ) {
                    building_can_succeed_even_if_one_factor_source_fails_assert_ids_of_successful_tx_e4::<E>()
                        .await
                }

                #[actix_rt::test]
                async fn building_can_succeed_even_if_one_factor_source_fails_assert_ids_of_failed_tx(
                ) {
                    building_can_succeed_even_if_one_factor_source_fails_assert_ids_of_failed_tx_e4::<E>().await
                }
            }
        }
    }
}
