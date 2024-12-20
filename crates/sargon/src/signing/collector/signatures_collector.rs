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

// === PUBLIC ===
impl<S: Signable> SignaturesCollector<S> {
    pub fn new(
        finish_early_strategy: SigningFinishEarlyStrategy,
        transactions: impl IntoIterator<Item = S>,
        interactor: Arc<dyn SignInteractor<S>>,
        profile: &Profile,
        purpose: SigningPurpose,
    ) -> Result<Self> {
        Self::with_signers_extraction(
            finish_early_strategy,
            IndexSet::from_iter(profile.factor_sources.iter()),
            transactions,
            interactor,
            purpose,
            |i| SignableWithEntities::extracting_from_profile(&i, profile),
        )
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
    pub(crate) fn with(
        finish_early_strategy: SigningFinishEarlyStrategy,
        profile_factor_sources: IndexSet<FactorSource>,
        transactions: IdentifiedVecOf<SignableWithEntities<S>>,
        interactor: Arc<dyn SignInteractor<S>>,
        purpose: SigningPurpose,
    ) -> Self {
        debug!("Init SignaturesCollector");
        let preprocessor = SignaturesCollectorPreprocessor::new(transactions);
        let (petitions, factors) =
            preprocessor.preprocess(profile_factor_sources, purpose);

        let dependencies = SignaturesCollectorDependencies::new(
            finish_early_strategy,
            interactor,
            factors,
        );
        let state = SignaturesCollectorState::new(petitions);

        Self {
            dependencies,
            state: RwLock::new(state),
        }
    }

    pub(crate) fn with_signers_extraction<F>(
        finish_early_strategy: SigningFinishEarlyStrategy,
        all_factor_sources_in_profile: IndexSet<FactorSource>,
        transactions: impl IntoIterator<Item = S>,
        interactor: Arc<dyn SignInteractor<S>>,
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
            self.process_batch_response(SignWithFactorsOutcome::irrelevant(
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

        let kind = factor_sources_of_kind.kind;
        if kind == FactorSourceKind::Device {
            debug!("Creating poly request for interactor");
            let request =
                self.request_for_parallel_interactor(factor_sources_of_kind);
            if !request.invalid_transactions_if_neglected.is_empty() {
                info!(
                    "If factors {:?} are neglected, invalid TXs: {:?}",
                    request.per_factor_source.keys(),
                    request.invalid_transactions_if_neglected
                )
            }
            debug!("Dispatching poly request to interactor: {:?}", request);
            let response = self.dependencies.interactor.sign(request).await?;
            debug!("Got response from poly interactor: {:?}", response);
            self.process_batch_response(response);
        } else {
            let factor_sources = factor_sources_of_kind.factor_sources();
            for factor_source in factor_sources {
                // Prepare the request for the interactor
                debug!("Creating mono request for interactor");
                let factor_source_id =
                    factor_source.factor_source_id().as_hash().cloned().expect(
                        "Signature Collector only works with HD FactorSources.",
                    );

                let request =
                    self.request_for_serial_interactor(kind, &factor_source_id);

                if !request.invalid_transactions_if_neglected.is_empty() {
                    info!(
                        "If factor {:?} are neglected, invalid TXs: {:?}",
                        factor_source_id,
                        request.invalid_transactions_if_neglected
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

    fn input_for_interactor(
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
            .input_for_interactor(factor_source_id)
    }

    fn request_for_serial_interactor(
        &self,
        factor_source_kind: FactorSourceKind,
        factor_source_id: &FactorSourceIDFromHash,
    ) -> SignRequest<S> {
        let batch_signing_request = self.input_for_interactor(factor_source_id);

        let invalid_transactions_if_neglected = self
            .invalid_transactions_if_neglected_factor_sources(IndexSet::just(
                *factor_source_id,
            ))
            .into_iter()
            .collect::<IndexSet<_>>();

        SignRequest::new(
            factor_source_kind,
            IndexMap::just((*factor_source_id, batch_signing_request)),
            invalid_transactions_if_neglected,
        )
    }

    fn request_for_parallel_interactor(
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
        let per_factor_source = factor_source_ids
            .clone()
            .iter()
            .map(|fid| (*fid, self.input_for_interactor(fid)))
            .collect::<IndexMap<
                FactorSourceIDFromHash,
                IndexSet<TransactionSignRequestInput<S>>,
            >>();

        let invalid_transactions_if_neglected = self
            .invalid_transactions_if_neglected_factor_sources(
                factor_source_ids,
            );

        // Prepare the request for the interactor
        SignRequest::new(
            factor_sources_of_kind.kind,
            per_factor_source,
            invalid_transactions_if_neglected,
        )
    }

    fn invalid_transactions_if_neglected_factor_sources(
        &self,
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
            .invalid_transactions_if_neglected_factors(factor_source_ids)
    }

    fn process_batch_response(&self, response: SignWithFactorsOutcome<S::ID>) {
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
#[cfg(test)]
mod tests {
    use std::iter;

    use super::*;

    impl<S: Signable> SignaturesCollector<S> {
        /// Used by tests
        pub(crate) fn petitions(self) -> Petitions<S> {
            self.state
                .read()
                .expect("SignaturesCollector lock should not have been poisoned.")
                .petitions
                .read()
                .expect("SignaturesCollectorState lock should not have been poisoned.")
                .clone()
        }
    }

    fn assert_petition<S: Signable>(
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
                .expect("PetitionForTransaction lock should not have been poisoned.")
                .keys()
                .collect::<HashSet<_>>(),
            addresses
        );

        assert!(petition
            .for_entities
            .read()
            .expect(
                "PetitionForTransaction lock should not have been poisoned."
            )
            .iter()
            .all(|(a, p)| { p.entity == *a }));

        assert!(petition
            .for_entities
            .read()
            .expect(
                "PetitionForTransaction lock should not have been poisoned."
            )
            .iter()
            .all(|(_, p)| { p.payload_id == t.get_id() }));

        for (k, v) in petition
            .for_entities
            .read()
            .expect(
                "PetitionForTransaction lock should not have been poisoned.",
            )
            .iter()
        {
            let threshold = threshold_factors.get(k);
            if let Some(actual_threshold) = &v.threshold_factors {
                let threshold = threshold.unwrap().clone();
                assert_eq!(
                    actual_threshold
                        .read()
                        .expect("PetitionForEntity lock should not have been poisoned.")
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
                        .expect("PetitionForEntity lock should not have been poisoned.")
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

    #[test]
    fn profile_with_unknown_account() {
        let res = SignaturesCollector::new(
            SigningFinishEarlyStrategy::default(),
            [TransactionIntent::sample_entities_requiring_auth(
                [&Account::sample_at(0)],
                [],
            )],
            Arc::new(TestSignInteractor::new(SimulatedUser::prudent_no_fail())),
            &Profile::sample_from(IndexSet::new(), [], []),
            SigningPurpose::sign_transaction_primary(),
        );
        assert!(res.is_ok());
    }

    #[test]
    fn invalid_profile_unknown_persona() {
        let res = SignaturesCollector::new(
            SigningFinishEarlyStrategy::default(),
            [TransactionIntent::sample_entities_requiring_auth(
                [],
                [&Persona::sample_at(0)],
            )],
            Arc::new(TestSignInteractor::new(SimulatedUser::prudent_no_fail())),
            &Profile::sample_from(IndexSet::new(), [], []),
            SigningPurpose::sign_transaction_primary(),
        );
        assert!(matches!(res, Err(CommonError::UnknownPersona)));
    }

    #[actix_rt::test]
    async fn valid_profile() {
        let factors_sources = FactorSource::sample_all();
        let persona = Persona::sample_at(0);

        let collector = SignaturesCollector::new(
            SigningFinishEarlyStrategy::default(),
            [TransactionIntent::sample_entities_requiring_auth(
                [],
                [&persona],
            )],
            Arc::new(TestSignInteractor::new(SimulatedUser::prudent_no_fail())),
            &Profile::sample_from(factors_sources, [], [&persona]),
            SigningPurpose::sign_transaction_primary(),
        )
        .unwrap();
        let outcome = collector.collect_signatures().await.unwrap();
        assert!(outcome.successful())
    }

    #[actix_rt::test]
    async fn continues_even_with_failed_tx_when_configured_to() {
        let factor_sources = &FactorSource::sample_all();
        let a0 = &Account::sample_at(0);
        let a1 = &Account::sample_at(1);

        let t0 = TransactionIntent::sample_entities_requiring_auth([a1], []);
        let t1 = TransactionIntent::sample_entities_requiring_auth([a0], []);

        let profile =
            Profile::sample_from(factor_sources.clone(), [a0, a1], []);

        let collector = SignaturesCollector::new(
            SigningFinishEarlyStrategy::new(
                WhenAllTransactionsAreValid(FinishEarly),
                WhenSomeTransactionIsInvalid(Continue),
            ),
            [t0.clone(), t1.clone()],
            Arc::new(TestSignInteractor::new(
                SimulatedUser::prudent_with_failures(
                    SimulatedFailures::with_simulated_failures([
                        FactorSourceIDFromHash::sample_at(1),
                    ]),
                ),
            )),
            &profile,
            SigningPurpose::sign_transaction_primary(),
        )
        .unwrap();

        let outcome = collector.collect_signatures().await.unwrap();
        assert!(!outcome.successful());
        assert_eq!(outcome.failed_transactions().len(), 1);
        assert_eq!(outcome.successful_transactions().len(), 1);
    }

    #[actix_rt::test]
    async fn continues_even_when_all_valid_if_configured_to() {
        let test = async move |when_all_valid: WhenAllTransactionsAreValid,
                               expected_sig_count: usize| {
            let factor_sources = &FactorSource::sample_all();
            let a5 = &Account::sample_at(5);

            let t0 =
                TransactionIntent::sample_entities_requiring_auth([a5], []);

            let profile =
                Profile::sample_from(factor_sources.clone(), [a5], []);

            let collector = SignaturesCollector::new(
                SigningFinishEarlyStrategy::new(
                    when_all_valid,
                    WhenSomeTransactionIsInvalid::default(),
                ),
                [t0.clone()],
                Arc::new(TestSignInteractor::new(
                    SimulatedUser::prudent_no_fail(),
                )),
                &profile,
                SigningPurpose::sign_transaction_primary(),
            )
            .unwrap();

            let outcome = collector.collect_signatures().await.unwrap();
            assert!(outcome.successful());
            assert_eq!(
                outcome.signatures_of_successful_transactions().len(),
                expected_sig_count
            );
        };

        test(WhenAllTransactionsAreValid(FinishEarly), 1).await;
        test(WhenAllTransactionsAreValid(Continue), 2).await;
    }

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

    #[test]
    fn test_profile() {
        let factor_sources = &FactorSource::sample_all();
        let a0 = &Account::sample_at(0);
        let a1 = &Account::sample_at(1);
        let a2 = &Account::sample_at(2);
        let a6 = &Account::sample_at(6);

        let p0 = &Persona::sample_at(0);
        let p1 = &Persona::sample_at(1);
        let p2 = &Persona::sample_at(2);
        let p6 = &Persona::sample_at(6);

        let t0 = TransactionIntent::sample_entities_requiring_auth(
            [a0, a1],
            [p0, p1],
        );
        let t1 =
            TransactionIntent::sample_entities_requiring_auth([a0, a1, a2], []);
        let t2 =
            TransactionIntent::sample_entities_requiring_auth([], [p0, p1, p2]);
        let t3 = TransactionIntent::sample_entities_requiring_auth([a6], [p6]);

        let profile = Profile::sample_from(
            factor_sources.clone(),
            [a0, a1, a2, a6],
            [p0, p1, p2, p6],
        );

        let collector = SignaturesCollector::new(
            SigningFinishEarlyStrategy::default(),
            [t0.clone(), t1.clone(), t2.clone(), t3.clone()],
            Arc::new(TestSignInteractor::new(SimulatedUser::prudent_no_fail())),
            &profile,
            SigningPurpose::sign_transaction_primary(),
        )
        .unwrap();

        let petitions = collector.petitions();

        assert_eq!(
            petitions
                .txid_to_petition
                .read()
                .expect("Petitions lock should not have been poisoned")
                .len(),
            4
        );

        {
            let petitions_ref = petitions
                .txid_to_petition
                .read()
                .expect("Petitions lock should not have been poisoned");
            let petition =
                petitions_ref.get(&t3.transaction_intent_hash()).unwrap();
            let for_entities = petition
                .for_entities
                .read()
                .expect("PetitionForTransaction lock should not have been poisoned.")
                .clone();
            let pet6 = for_entities.get(&a6.address.into()).unwrap();

            let paths6 = pet6
                .all_factor_instances()
                .iter()
                .map(|f| f.factor_instance().derivation_path())
                .collect_vec();

            pretty_assertions::assert_eq!(
                paths6,
                iter::repeat_n(
                    DerivationPath::from(AccountPath::new(
                        NetworkID::Mainnet,
                        CAP26KeyKind::TransactionSigning,
                        Hardened::from_local_key_space(6, IsSecurified(true))
                            .unwrap()
                    )),
                    5
                )
                .collect_vec()
            );
        }

        assert_petition(
            &petitions,
            &t0,
            HashMap::from_iter([
                (
                    a0.address.into(),
                    HashSet::just(FactorSourceIDFromHash::sample_at(0)),
                ),
                (
                    a1.address.into(),
                    HashSet::just(FactorSourceIDFromHash::sample_at(1)),
                ),
                (
                    p0.address.into(),
                    HashSet::just(FactorSourceIDFromHash::sample_at(0)),
                ),
                (
                    p1.address.into(),
                    HashSet::just(FactorSourceIDFromHash::sample_at(1)),
                ),
            ]),
            HashMap::new(),
        );

        assert_petition(
            &petitions,
            &t1,
            HashMap::from_iter([
                (
                    a0.address.into(),
                    HashSet::just(FactorSourceIDFromHash::sample_at(0)),
                ),
                (
                    a1.address.into(),
                    HashSet::just(FactorSourceIDFromHash::sample_at(1)),
                ),
                (
                    a2.address.into(),
                    HashSet::just(FactorSourceIDFromHash::sample_at(0)),
                ),
            ]),
            HashMap::new(),
        );

        assert_petition(
            &petitions,
            &t2,
            HashMap::from_iter([
                (
                    p0.address.into(),
                    HashSet::just(FactorSourceIDFromHash::sample_at(0)),
                ),
                (
                    p1.address.into(),
                    HashSet::just(FactorSourceIDFromHash::sample_at(1)),
                ),
                (
                    p2.address.into(),
                    HashSet::just(FactorSourceIDFromHash::sample_at(0)),
                ),
            ]),
            HashMap::new(),
        );

        assert_petition(
            &petitions,
            &t3,
            HashMap::from_iter([
                (
                    a6.address.into(),
                    HashSet::from_iter([
                        FactorSourceIDFromHash::sample_at(0),
                        FactorSourceIDFromHash::sample_at(3),
                        FactorSourceIDFromHash::sample_at(5),
                    ]),
                ),
                (
                    p6.address.into(),
                    HashSet::from_iter([
                        FactorSourceIDFromHash::sample_at(0),
                        FactorSourceIDFromHash::sample_at(3),
                        FactorSourceIDFromHash::sample_at(5),
                    ]),
                ),
            ]),
            HashMap::from_iter([
                (
                    a6.address.into(),
                    HashSet::from_iter([
                        FactorSourceIDFromHash::sample_at(1),
                        FactorSourceIDFromHash::sample_at(4),
                    ]),
                ),
                (
                    p6.address.into(),
                    HashSet::from_iter([
                        FactorSourceIDFromHash::sample_at(1),
                        FactorSourceIDFromHash::sample_at(4),
                    ]),
                ),
            ]),
        );
    }

    mod multi_tx {
        use super::*;

        async fn multi_accounts_multi_personas_all_single_factor_controlled_with_sim_user(
            sim: SimulatedUser<TransactionIntent>,
        ) {
            let factor_sources = &FactorSource::sample_all();
            let a0 = Account::sample_at(0);
            let a1 = Account::sample_at(1);
            let a2 = Account::sample_at(2);

            let p0 = Persona::sample_at(0);
            let p1 = Persona::sample_at(1);
            let p2 = Persona::sample_at(2);

            let t0 = TransactionIntent::sample_entities_requiring_auth(
                [&a0, &a1],
                [&p0, &p1],
            );
            let t1 = TransactionIntent::sample_entities_requiring_auth(
                [&a0, &a1, &a2],
                [],
            );
            let t2 = TransactionIntent::sample_entities_requiring_auth(
                [],
                [&p0, &p1, &p2],
            );

            let profile = Profile::sample_from(
                factor_sources.clone(),
                [&a0, &a1, &a2],
                [&p0, &p1, &p2],
            );

            let collector = SignaturesCollector::new(
                SigningFinishEarlyStrategy::default(),
                [t0.clone(), t1.clone(), t2.clone()],
                Arc::new(TestSignInteractor::new(sim)),
                &profile,
                SigningPurpose::sign_transaction_primary(),
            )
            .unwrap();

            let outcome = collector.collect_signatures().await.unwrap();
            assert!(outcome.successful());
            assert!(outcome.failed_transactions().is_empty());
            assert_eq!(
                outcome.signatures_of_successful_transactions().len(),
                10
            );
            assert_eq!(
                outcome
                    .successful_transactions()
                    .into_iter()
                    .map(|t| t.signable_id)
                    .collect::<HashSet<_>>(),
                HashSet::from_iter([
                    t0.clone().transaction_intent_hash(),
                    t1.clone().transaction_intent_hash(),
                    t2.clone().transaction_intent_hash(),
                ])
            );
            let st0 = outcome
                .successful_transactions()
                .into_iter()
                .find(|st| st.signable_id == t0.transaction_intent_hash())
                .unwrap();

            assert_eq!(
                st0.signatures
                    .clone()
                    .into_iter()
                    .map(|s| s.owned_factor_instance().owner)
                    .collect::<HashSet<_>>(),
                HashSet::from_iter([
                    AddressOfAccountOrPersona::from(a0.address),
                    AddressOfAccountOrPersona::from(a1.address),
                    AddressOfAccountOrPersona::from(p0.address),
                    AddressOfAccountOrPersona::from(p1.address),
                ])
            );

            let st1 = outcome
                .successful_transactions()
                .into_iter()
                .find(|st| st.signable_id == t1.transaction_intent_hash())
                .unwrap();

            assert_eq!(
                st1.signatures
                    .clone()
                    .into_iter()
                    .map(|s| s.owned_factor_instance().owner)
                    .collect::<HashSet<_>>(),
                HashSet::from_iter([
                    AddressOfAccountOrPersona::from(a0.address),
                    AddressOfAccountOrPersona::from(a1.address),
                    AddressOfAccountOrPersona::from(a2.address),
                ])
            );

            let st2 = outcome
                .successful_transactions()
                .into_iter()
                .find(|st| st.signable_id == t2.transaction_intent_hash())
                .unwrap();

            assert_eq!(
                st2.signatures
                    .clone()
                    .into_iter()
                    .map(|s| s.owned_factor_instance().owner)
                    .collect::<HashSet<_>>(),
                HashSet::from_iter([
                    AddressOfAccountOrPersona::from(p0.address),
                    AddressOfAccountOrPersona::from(p1.address),
                    AddressOfAccountOrPersona::from(p2.address),
                ])
            );

            // Assert sorted in increasing "friction order".
            assert_eq!(
                outcome
                    .signatures_of_successful_transactions()
                    .iter()
                    .map(|f| { f.factor_source_id().kind })
                    .collect::<IndexSet::<FactorSourceKind>>(),
                IndexSet::<FactorSourceKind>::from_iter([
                    FactorSourceKind::Device,
                    FactorSourceKind::LedgerHQHardwareWallet
                ])
            );
        }

        #[derive(Clone, Debug)]
        struct Vector {
            simulated_user: SimulatedUser<TransactionIntent>,
            expected: Expected,
        }
        #[derive(Clone, Debug, PartialEq, Eq)]
        struct Expected {
            successful_txs_signature_count: usize,
            signed_factor_source_kinds: IndexSet<FactorSourceKind>,
            expected_neglected_factor_source_count: usize,
        }
        async fn multi_securified_entities_with_sim_user(vector: Vector) {
            let factor_sources = &FactorSource::sample_all();

            let a4 = &Account::sample_at(4);
            let a5 = &Account::sample_at(5);
            let a6 = &Account::sample_at(6);

            let p4 = &Persona::sample_at(4);
            let p5 = &Persona::sample_at(5);
            let p6 = &Persona::sample_at(6);

            let t0 =
                TransactionIntent::sample_entities_requiring_auth([a5], [p5]);
            let t1 = TransactionIntent::sample_entities_requiring_auth(
                [a4, a5, a6],
                [],
            );
            let t2 = TransactionIntent::sample_entities_requiring_auth(
                [a4, a6],
                [p4, p6],
            );
            let t3 = TransactionIntent::sample_entities_requiring_auth(
                [],
                [p4, p5, p6],
            );

            let profile = Profile::sample_from(
                factor_sources.clone(),
                [a4, a5, a6],
                [p4, p5, p6],
            );

            let collector = SignaturesCollector::new(
                SigningFinishEarlyStrategy::default(),
                [t0.clone(), t1.clone(), t2.clone(), t3.clone()],
                Arc::new(TestSignInteractor::new(vector.simulated_user)),
                &profile,
                SigningPurpose::sign_transaction_primary(),
            )
            .unwrap();

            let outcome = collector.collect_signatures().await.unwrap();

            assert_eq!(
                outcome.neglected_factor_sources().len(),
                vector.expected.expected_neglected_factor_source_count
            );

            assert!(outcome.successful());
            assert!(outcome.failed_transactions().is_empty());
            assert_eq!(
                outcome.signatures_of_successful_transactions().len(),
                vector.expected.successful_txs_signature_count
            );
            assert_eq!(
                outcome
                    .successful_transactions()
                    .into_iter()
                    .map(|t| t.signable_id)
                    .collect::<HashSet<_>>(),
                HashSet::from_iter([
                    t0.clone().transaction_intent_hash(),
                    t1.clone().transaction_intent_hash(),
                    t2.clone().transaction_intent_hash(),
                    t3.clone().transaction_intent_hash(),
                ])
            );

            // Assert sorted in increasing "friction order".
            assert_eq!(
                outcome
                    .signatures_of_successful_transactions()
                    .iter()
                    .map(|f| { f.factor_source_id().kind })
                    .collect::<IndexSet::<FactorSourceKind>>(),
                vector.expected.signed_factor_source_kinds
            );
        }

        mod with_failure {
            use std::rc::Rc;

            use super::*;

            #[actix_rt::test]
            async fn multi_securified_entities() {
                multi_securified_entities_with_sim_user(Vector {
                    simulated_user: SimulatedUser::prudent_with_failures(
                        SimulatedFailures::with_simulated_failures([
                            FactorSourceIDFromHash::sample_at(1),
                        ]),
                    ),
                    expected: Expected {
                        successful_txs_signature_count: 24,
                        // We always end early
                        // `Device` FactorSourceKind never got used since it
                        // we are done after Passphrase.
                        signed_factor_source_kinds:
                            IndexSet::<FactorSourceKind>::from_iter([
                                FactorSourceKind::ArculusCard,
                                FactorSourceKind::Password,
                            ]),
                        expected_neglected_factor_source_count: 1,
                    },
                })
                .await;
            }

            #[actix_rt::test]
            async fn failed_threshold_successful_override() {
                let factor_sources = &FactorSource::sample_all();
                let a9 = &Account::sample_at(9);
                let tx0 =
                    TransactionIntent::sample_entities_requiring_auth([a9], []);

                let all_transactions = [tx0.clone()];

                let profile =
                    Profile::sample_from(factor_sources.clone(), [a9], []);

                let collector = SignaturesCollector::new(
                    SigningFinishEarlyStrategy::default(),
                    all_transactions,
                    Arc::new(TestSignInteractor::new(
                        SimulatedUser::prudent_with_failures(
                            SimulatedFailures::with_simulated_failures([
                                FactorSourceIDFromHash::sample_at(1),
                            ]),
                        ),
                    )),
                    &profile,
                    SigningPurpose::sign_transaction_primary(),
                )
                .unwrap();

                let outcome = collector.collect_signatures().await.unwrap();
                assert!(outcome.successful());
                assert_eq!(
                    outcome
                        .successful_transactions()
                        .into_iter()
                        .map(|t| t.signable_id.clone())
                        .collect_vec(),
                    vec![tx0.clone().transaction_intent_hash()]
                );
                assert_eq!(
                    outcome
                        .all_signatures()
                        .into_iter()
                        .map(|s| s.factor_source_id())
                        .collect_vec(),
                    vec![FactorSourceIDFromHash::sample_at(8)]
                );
            }

            #[actix_rt::test]
            async fn many_failing_tx() {
                let factor_sources = &FactorSource::sample_all();
                let a0 = &Account::sample_at(0);
                let p3 = &Persona::sample_at(3);
                let tx =
                    TransactionIntent::sample_entities_requiring_auth([], [p3]);

                // In need of different PublicKeyHashes so the IntentHash of each transaction is different
                let make_random_pk_hash = || {
                    let private_key = Ed25519PrivateKey::generate();
                    PublicKeyHash::hash(private_key.public_key())
                };
                let failing_transactions = (0..100)
                    .map(|_| {
                        TransactionIntent::sample_entity_addresses_with_pub_key_hashes_requiring_auth(
                            [(a0.address, make_random_pk_hash())],
                            [],
                        )
                    })
                    .collect::<Vec<_>>();
                let mut all_transactions = failing_transactions.clone();
                all_transactions.push(tx.clone());

                let profile =
                    Profile::sample_from(factor_sources.clone(), [a0], [p3]);

                let collector = SignaturesCollector::new(
                    SigningFinishEarlyStrategy::default(),
                    all_transactions,
                    Arc::new(TestSignInteractor::new(
                        SimulatedUser::prudent_with_failures(
                            SimulatedFailures::with_simulated_failures([
                                FactorSourceIDFromHash::sample_at(0),
                            ]),
                        ),
                    )),
                    &profile,
                    SigningPurpose::sign_transaction_primary(),
                )
                .unwrap();

                let outcome = collector.collect_signatures().await.unwrap();
                assert!(!outcome.successful());
                assert_eq!(
                    outcome
                        .failed_transactions()
                        .iter()
                        .map(|t| t.signable_id.clone())
                        .collect_vec(),
                    failing_transactions
                        .iter()
                        .map(|t| t.transaction_intent_hash().clone())
                        .collect_vec()
                );

                assert_eq!(
                    outcome
                        .ids_of_neglected_factor_sources_failed()
                        .into_iter()
                        .collect_vec(),
                    vec![FactorSourceIDFromHash::sample_at(0)]
                );

                assert!(outcome
                    .ids_of_neglected_factor_sources_skipped_by_user()
                    .is_empty());

                assert_eq!(
                    outcome
                        .successful_transactions()
                        .into_iter()
                        .map(|t| t.signable_id)
                        .collect_vec(),
                    vec![tx.transaction_intent_hash()]
                )
            }

            #[actix_rt::test]
            async fn same_tx_is_not_shown_to_user_in_case_of_already_failure() {
                let factor_sources = FactorSource::sample_all();

                let a7 = Account::sample_at(7);
                let a0 = Account::sample_at(0);

                let tx0 = TransactionIntent::sample_entities_requiring_auth(
                    [&a7, &a0],
                    [],
                );
                let tx1 = TransactionIntent::sample_entities_requiring_auth(
                    [&a0],
                    [],
                );

                let profile = Profile::sample_from(
                    factor_sources.clone(),
                    [&a7, &a0],
                    [],
                );

                type Tuple = (
                    FactorSourceKind,
                    IndexSet<
                        InvalidTransactionIfNeglected<TransactionIntentHash>,
                    >,
                );
                type Tuples = Vec<Tuple>;
                let tuples =
                    Rc::<RefCell<Tuples>>::new(RefCell::new(Tuples::default()));
                let tuples_clone = tuples.clone();
                let collector = SignaturesCollector::new(
                    SigningFinishEarlyStrategy::default(),
                    [tx0.clone(), tx1.clone()],
                    Arc::new(TestSignInteractor::new(SimulatedUser::with_spy(
                        move |kind, invalid| {
                            let tuple = (kind, invalid);
                            let mut x = RefCell::borrow_mut(&tuples_clone);
                            x.push(tuple)
                        },
                        SimulatedUserMode::Prudent,
                        SimulatedFailures::with_simulated_failures([
                            FactorSourceIDFromHash::sample_at(2), // will cause any TX with a7 to fail
                        ]),
                    ))),
                    &profile,
                    SigningPurpose::sign_transaction_primary(),
                )
                .unwrap();

                let outcome = collector.collect_signatures().await.unwrap();

                let tuples = tuples.borrow().clone();
                assert_eq!(
                    tuples,
                    vec![
                        (
                            FactorSourceKind::LedgerHQHardwareWallet,
                            IndexSet::just(InvalidTransactionIfNeglected::new(
                                tx0.clone().transaction_intent_hash(),
                                [a7.address.into()]
                            ))
                        ),
                        // Important that we do NOT display any mentioning of `tx0` here again!
                        (
                            FactorSourceKind::Device,
                            IndexSet::just(InvalidTransactionIfNeglected::new(
                                tx1.clone().transaction_intent_hash(),
                                [a0.address.into()]
                            ))
                        ),
                    ]
                );

                assert!(!outcome.successful());
                assert_eq!(
                    outcome.ids_of_neglected_factor_sources_failed(),
                    IndexSet::<FactorSourceIDFromHash>::just(
                        FactorSourceIDFromHash::sample_at(2)
                    )
                );
                assert_eq!(
                    outcome.ids_of_neglected_factor_sources_irrelevant(),
                    IndexSet::<FactorSourceIDFromHash>::from_iter([
                        FactorSourceIDFromHash::sample_at(6),
                        FactorSourceIDFromHash::sample_at(7),
                        FactorSourceIDFromHash::sample_at(8),
                        FactorSourceIDFromHash::sample_at(9)
                    ])
                );
                assert_eq!(
                    outcome
                        .successful_transactions()
                        .into_iter()
                        .map(|t| t.signable_id)
                        .collect_vec(),
                    vec![tx1.transaction_intent_hash().clone()]
                );

                assert_eq!(
                    outcome
                        .failed_transactions()
                        .into_iter()
                        .map(|t| t.signable_id)
                        .collect_vec(),
                    vec![tx0.transaction_intent_hash().clone()]
                );

                assert_eq!(outcome.all_signatures().len(), 1);

                assert!(outcome
                    .all_signatures()
                    .into_iter()
                    .map(|s| s.payload_id().clone())
                    .all(|i| i == tx1.transaction_intent_hash()));

                assert_eq!(
                    outcome
                        .all_signatures()
                        .into_iter()
                        .map(|s| s.derivation_path())
                        .collect_vec(),
                    vec![DerivationPath::from(AccountPath::new(
                        NetworkID::Mainnet,
                        CAP26KeyKind::TransactionSigning,
                        Hardened::from_local_key_space(0, IsSecurified(false)) // unsecurified account at `0`.
                            .unwrap()
                    ))]
                )
            }
        }

        mod no_fail {
            use super::*;

            #[actix_rt::test]
            async fn multi_accounts_multi_personas_all_single_factor_controlled(
            ) {
                multi_accounts_multi_personas_all_single_factor_controlled_with_sim_user(
                    SimulatedUser::prudent_no_fail(),
                )
                    .await;

                // Same result with lazy user, not able to skip without failures.
                multi_accounts_multi_personas_all_single_factor_controlled_with_sim_user(
                    SimulatedUser::lazy_sign_minimum([]),
                )
                    .await
            }

            #[actix_rt::test]
            async fn multi_securified_entities() {
                multi_securified_entities_with_sim_user(Vector {
                    simulated_user: SimulatedUser::prudent_no_fail(),
                    expected: Expected {
                        successful_txs_signature_count: 32,
                        // We always end early
                        // `Device` FactorSourceKind never got used since it
                        // we are done after YubiKey.
                        signed_factor_source_kinds:
                            IndexSet::<FactorSourceKind>::from_iter([
                                FactorSourceKind::LedgerHQHardwareWallet,
                                FactorSourceKind::ArculusCard,
                                FactorSourceKind::Password,
                            ]),
                        expected_neglected_factor_source_count: 0,
                    },
                })
                .await;

                multi_securified_entities_with_sim_user(Vector {
                    simulated_user: SimulatedUser::lazy_sign_minimum([]),
                    expected: Expected {
                        successful_txs_signature_count: 24,
                        // We always end early, this lazy user was able to skip
                        // Ledger.
                        signed_factor_source_kinds:
                            IndexSet::<FactorSourceKind>::from_iter([
                                FactorSourceKind::ArculusCard,
                                FactorSourceKind::Password,
                                FactorSourceKind::Device,
                            ]),
                        expected_neglected_factor_source_count: 2,
                    },
                })
                .await;
            }
        }
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
                                Hardened::from_local_key_space(0, IsSecurified(false)).unwrap(),
                            ),
                        ),
                        Account::sample_unsecurified_mainnet(
                            "A1",
                            HierarchicalDeterministicFactorInstance::new_for_entity(
                                FactorSourceIDFromHash::sample_at(0),
                                CAP26EntityKind::Account,
                                Hardened::from_local_key_space(1, IsSecurified(false)).unwrap(),
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
                                0,
                                IsSecurified(false)
                            )
                            .unwrap()
                        )),
                        DerivationPath::from(AccountPath::new(
                            NetworkID::Mainnet,
                            CAP26KeyKind::TransactionSigning,
                            Hardened::from_local_key_space(
                                1,
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

            impl AccountOrPersona {
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
                                            Hardened::from_local_key_space(0, IsSecurified(true)).unwrap(),
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
                assert_eq!(signatures.len(), 2);

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

    mod rola {
        use super::*;

        #[actix_rt::test]
        async fn test_petitions_for() {
            let factor_sources = &FactorSource::sample_all();

            let a0 = &Account::sample_at(0);
            let a1 = &Account::sample_at(1);
            let a6 = &Account::sample_at(6);

            let p0 = &Persona::sample_at(0);
            let p1 = &Persona::sample_at(1);
            let p6 = &Persona::sample_at(6);

            let entities_to_sign = vec![
                AddressOfAccountOrPersona::Account(a0.address),
                AddressOfAccountOrPersona::Account(a1.address),
                AddressOfAccountOrPersona::Account(a6.address),
                AddressOfAccountOrPersona::Identity(p0.address),
                AddressOfAccountOrPersona::Identity(p1.address),
                AddressOfAccountOrPersona::Identity(p6.address),
            ];

            let auth_intent = AuthIntent::new_from_request(
                DappToWalletInteractionAuthChallengeNonce::sample(),
                DappToWalletInteractionMetadata::sample(),
                entities_to_sign,
            )
            .unwrap();

            let profile = Profile::sample_from(
                factor_sources.clone(),
                [a0, a1, a6],
                [p0, p1, p6],
            );

            let collector = SignaturesCollector::new(
                SigningFinishEarlyStrategy::default(),
                [auth_intent.clone()],
                Arc::new(TestSignInteractor::new(
                    SimulatedUser::prudent_no_fail(),
                )),
                &profile,
                SigningPurpose::ROLA,
            )
            .unwrap();

            let petitions = collector.petitions();

            assert_eq!(
                petitions
                    .txid_to_petition
                    .read()
                    .expect("Petitions lock should not have been poisoned")
                    .len(),
                1
            );

            assert_petition(
                &petitions,
                &auth_intent,
                HashMap::from_iter([
                    (
                        a0.address.into(),
                        HashSet::just(FactorSourceIDFromHash::sample_at(0)),
                    ),
                    (
                        a1.address.into(),
                        HashSet::just(FactorSourceIDFromHash::sample_at(1)),
                    ),
                    (
                        a6.address.into(),
                        HashSet::from_iter([
                            // Only device factor source is used for signing auth for securified entity
                            FactorSourceIDFromHash::sample_at(0),
                        ]),
                    ),
                    (
                        p0.address.into(),
                        HashSet::just(FactorSourceIDFromHash::sample_at(0)),
                    ),
                    (
                        p1.address.into(),
                        HashSet::just(FactorSourceIDFromHash::sample_at(1)),
                    ),
                    (
                        p6.address.into(),
                        HashSet::from_iter([
                            // Only device factor source is used for signing auth for securified entity
                            FactorSourceIDFromHash::sample_at(0),
                        ]),
                    ),
                ]),
                HashMap::from_iter([]),
            );
        }
    }
}
