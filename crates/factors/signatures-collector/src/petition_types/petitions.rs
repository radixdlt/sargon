#![allow(clippy::non_canonical_partial_ord_impl)]

use std::ops::Deref;

use crate::prelude::*;

#[derive(derive_more::Debug)]
#[debug("{}", self.debug_str())]
pub struct Petitions<S: Signable> {
    /// Lookup from factor to TXID.
    ///
    ///
    /// The same HDFactorSource might be required by many payloads
    /// and per payload might be required by many entities, e.g. transactions
    /// `t0` and `t1`, where
    /// `t0` is signed by accounts: A and B
    /// `t1` is signed by accounts: A, C and D,
    ///
    /// Where A, B, C and D, all use the factor source, e.g. some arculus
    /// card which the user has setup as a factor (source) for all these accounts.
    pub(crate) factor_source_to_signable_id:
        HashMap<FactorSourceIDFromHash, IndexSet<S::ID>>,

    /// Lookup from TXID to signatures builders, sorted according to the order of
    /// transactions passed to the SignaturesBuilder.
    pub txid_to_petition: RwLock<IndexMap<S::ID, PetitionForTransaction<S>>>,
}

impl<S: Signable> Clone for Petitions<S> {
    fn clone(&self) -> Self {
        Self {
            factor_source_to_signable_id: self
                .factor_source_to_signable_id
                .clone(),
            txid_to_petition: RwLock::new(
                self.txid_to_petition
                    .read()
                    .expect("Petitions lock should not have been poisoned.")
                    .clone(),
            ),
        }
    }
}

impl<S: Signable> PartialEq for Petitions<S> {
    fn eq(&self, other: &Self) -> bool {
        self.factor_source_to_signable_id == other.factor_source_to_signable_id
            && self
                .txid_to_petition
                .read()
                .expect("Petitions lock should not have been poisoned")
                .deref()
                == other
                    .txid_to_petition
                    .read()
                    .expect("Petitions lock should not have been poisoned")
                    .deref()
    }
}

impl<S: Signable> Eq for Petitions<S> {}

impl<S: Signable> Petitions<S> {
    pub(crate) fn new(
        factor_source_to_signable_ids: HashMap<
            FactorSourceIDFromHash,
            IndexSet<S::ID>,
        >,
        txid_to_petition: IndexMap<S::ID, PetitionForTransaction<S>>,
    ) -> Self {
        Self {
            factor_source_to_signable_id: factor_source_to_signable_ids,
            txid_to_petition: RwLock::new(txid_to_petition),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FailedTransactions<ID: SignableID> {
    /// Collection of transactions which might be signed or not.
    pub(super) transactions: IndexMap<ID, PetitionTransactionOutcome<ID>>,
}
impl<ID: SignableID + HasSampleValues> HasSampleValues
    for FailedTransactions<ID>
{
    fn sample() -> Self {
        Self {
            transactions: [
                PetitionTransactionOutcome::<ID>::sample(),
                PetitionTransactionOutcome::<ID>::sample_other(),
            ]
            .into_iter()
            .map(|p| (p.signable_id.clone(), p))
            .collect(),
        }
    }

    fn sample_other() -> Self {
        Self {
            transactions: [PetitionTransactionOutcome::<ID>::sample_other()]
                .into_iter()
                .map(|p| (p.signable_id.clone(), p))
                .collect(),
        }
    }
}

impl<ID: SignableID> FailedTransactions<ID> {
    pub fn empty() -> Self {
        Self {
            transactions: IndexMap::new(),
        }
    }

    pub fn outcomes(&self) -> Vec<PetitionTransactionOutcome<ID>> {
        self.transactions.values().cloned().collect_vec()
    }

    /// Returns all the signatures for all the transactions.
    pub(crate) fn all_signatures(&self) -> IndexSet<HDSignature<ID>> {
        self.transactions
            .values()
            .flat_map(|v| v.signatures.iter())
            .cloned()
            .collect()
    }

    pub(crate) fn transactions(&self) -> Vec<SignedTransaction<ID>> {
        self.transactions
            .clone()
            .into_iter()
            .map(|(k, v)| SignedTransaction::new(k, v.signatures))
            .collect_vec()
    }

    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }

    pub fn failure_hashes(&self) -> IndexSet<ID> {
        self.transactions.keys().cloned().collect::<IndexSet<_>>()
    }

    pub(crate) fn add_outcome(
        &mut self,
        id: ID,
        outcome: PetitionTransactionOutcome<ID>,
    ) {
        assert!(!outcome.transaction_valid);
        self.transactions.insert(id, outcome);
    }
}

impl<S: Signable> Petitions<S> {
    pub(crate) fn outcome(&self) -> SignaturesOutcome<S::ID> {
        let txid_to_petition = self
            .txid_to_petition
            .read()
            .expect("Petitions lock should not have been poisoned.");
        let mut failed_transactions = FailedTransactions::empty();
        let mut successful_transactions = MaybeSignedTransactions::empty();
        let mut neglected_factor_sources = IndexSet::<NeglectedFactor>::new();

        for (intent_hash, petition_of_transaction) in txid_to_petition.iter() {
            let outcome = petition_of_transaction.clone().outcome();
            let intent_hash = intent_hash.clone();
            neglected_factor_sources.extend(outcome.neglected_factors());

            if outcome.transaction_valid {
                successful_transactions
                    .add_signatures(intent_hash, outcome.signatures);
            } else {
                failed_transactions.add_outcome(intent_hash, outcome);
            }
        }

        SignaturesOutcome::new(
            successful_transactions,
            failed_transactions,
            neglected_factor_sources,
        )
    }

    pub(crate) fn each_petition<T, U>(
        &self,
        factor_source_ids: IndexSet<FactorSourceIDFromHash>,
        each: impl Fn(&PetitionForTransaction<S>) -> T,
        combine: impl Fn(Vec<T>) -> U,
    ) -> U {
        let for_each = factor_source_ids
            .clone()
            .iter()
            .flat_map(|f| {
                self.factor_source_to_signable_id
                    .get(f)
                    .expect("Should be able to lookup intent hash for each factor source, did you call this method with irrelevant factor sources? Or did you recently change the preprocessor logic of the SignaturesCollector, if you did you've missed adding an entry for `factor_source_to_intent_hash`.map")
                    .iter()
                    .map(|intent_hash| {
                        let binding = self.txid_to_petition
                            .read()
                            .expect("Petitions lock should not have been poisoned.");
                        let value = binding.get(intent_hash).expect("Should have a petition for each transaction, did you recently change the preprocessor logic of the SignaturesCollector, if you did you've missed adding an entry for `txid_to_petition`.map");
                        each(value)
                    })
            }).collect_vec();
        combine(for_each)
    }

    pub(crate) fn invalid_transactions_if_neglected_factors(
        &self,
        cross_role_skip_outcome_analyzer: Arc<
            dyn CrossRoleSkipOutcomeAnalyzer<S::ID>,
        >,
        factor_source_ids: IndexSet<FactorSourceIDFromHash>,
    ) -> IndexSet<InvalidTransactionIfNeglected<S::ID>> {
        self.each_petition(
            factor_source_ids.clone(),
            |p| {
                p.invalid_transaction_if_neglected_factors(
                    cross_role_skip_outcome_analyzer.clone(),
                    factor_source_ids.clone(),
                )
            },
            |i| i.into_iter().flatten().collect(),
        )
    }

    pub(crate) fn should_neglect_factors_due_to_irrelevant(
        &self,
        factor_sources_of_kind: &FactorSourcesOfKind,
    ) -> bool {
        let ids = factor_sources_of_kind
            .factor_sources()
            .iter()
            .map(|f| {
                *f.factor_source_id().as_hash().expect(
                    "Signature Collector only works with HD FactorSources.",
                )
            })
            .collect::<IndexSet<_>>();
        self.each_petition(
            ids.clone(),
            |p| p.should_neglect_factors_due_to_irrelevant(ids.clone()),
            |i| i.into_iter().all(|x| x),
        )
    }

    /// # Panics
    /// Panics if no petition deem usage of `FactorSource` with id
    /// `factor_source_id` relevant. We SHOULD have checked this already with
    /// `should_neglect_factors_due_to_irrelevant` from SignatureCollector main
    /// loop, i.e. we should not have called this method from SignaturesCollector
    /// if `should_neglect_factors_due_to_irrelevant` returned true.
    pub(crate) fn per_transaction_input(
        &self,
        factor_source_id: &FactorSourceIDFromHash,
    ) -> IndexSet<TransactionSignRequestInput<S>> {
        self.each_petition(
            IndexSet::just(*factor_source_id),
            |p| {
                if p.has_tx_failed() {
                    None
                } else {
                    Some(p.input_for_interactor(factor_source_id))
                }
            },
            |i| i.into_iter().flatten().collect::<IndexSet<_>>(),
        )
    }

    pub(crate) fn status(&self) -> PetitionsStatus {
        self.each_petition(
            self.factor_source_to_signable_id.keys().cloned().collect(),
            |p| p.status_of_each_petition_for_entity(),
            |i| PetitionsStatus::reducing(i.into_iter().flatten()),
        )
    }

    fn add_signature(&self, signature: &HDSignature<S::ID>) {
        let binding = self
            .txid_to_petition
            .read()
            .expect("Petitions lock should not have been poisoned.");
        let petition = binding.get(signature.payload_id()).expect("Should have a petition for each transaction, did you recently change the preprocessor logic of the SignaturesCollector, if you did you've missed adding an entry for `txid_to_petition`.map");
        petition.add_signature(signature.clone())
    }

    fn neglect_factor_source_with_id(&self, neglected: NeglectedFactor) {
        self.each_petition(
            IndexSet::just(neglected.factor_source_id()),
            |p| p.neglect_factor_source(neglected.clone()),
            |_| (),
        )
    }

    pub(crate) fn process_batch_response(&self, response: SignResponse<S::ID>) {
        response
            .per_factor_outcome
            .iter()
            .for_each(|(id, outcome)| match outcome {
                FactorOutcome::Signed {
                    produced_signatures,
                    ..
                } => {
                    info!(
                        "Signed with {} (#{} signatures)",
                        id.clone(),
                        produced_signatures.len()
                    );
                    produced_signatures
                        .iter()
                        .for_each(|s| self.add_signature(s));
                }
                FactorOutcome::Neglected(neglected_factor) => {
                    self.neglect_factor_source_with_id(neglected_factor.clone())
                }
            })
    }

    #[allow(unused)]
    fn debug_str(&self) -> String {
        self.txid_to_petition
            .read()
            .expect("Petitions lock should not have been poisoned.")
            .iter()
            .map(|p| format!("Petitions({:#?}: {:#?})", p.0, p.1))
            .join(" + ")
    }
}

impl<S: Signable + ProvidesSamplesByBuildingManifest> HasSampleValues
    for Petitions<S>
{
    fn sample() -> Self {
        let p0 = PetitionForTransaction::<S>::sample();
        Self::new(
            HashMap::just((
                FactorSourceIDFromHash::sample_at(0),
                IndexSet::just(p0.signable.get_id()),
            )),
            IndexMap::just((p0.signable.get_id(), p0)),
        )
    }

    fn sample_other() -> Self {
        let p1 = PetitionForTransaction::<S>::sample();
        Self::new(
            HashMap::just((
                FactorSourceIDFromHash::sample_at(1),
                IndexSet::just(p1.signable.get_id()),
            )),
            IndexMap::just((p1.signable.get_id(), p1)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Petitions<TransactionIntent>;

    #[test]
    fn equality_of_samples() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality_of_samples() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn debug() {
        assert!(!format!("{:?}", SUT::sample()).is_empty());
    }
}
