#![allow(clippy::non_canonical_partial_ord_impl)]

use crate::prelude::*;

#[derive(derive_more::Debug, PartialEq, Eq)]
#[debug("{}", self.debug_str())]
pub(crate) struct Petitions {
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
    pub(crate) factor_source_to_intent_hashes:
        HashMap<FactorSourceIDFromHash, IndexSet<TransactionIntentHash>>,

    /// Lookup from TXID to signatures builders, sorted according to the order of
    /// transactions passed to the SignaturesBuilder.
    pub(crate) txid_to_petition:
        RefCell<IndexMap<TransactionIntentHash, PetitionForTransaction>>,
}

impl Petitions {
    pub(crate) fn new(
        factor_source_to_intent_hashes: HashMap<
            FactorSourceIDFromHash,
            IndexSet<TransactionIntentHash>,
        >,
        txid_to_petition: IndexMap<
            TransactionIntentHash,
            PetitionForTransaction,
        >,
    ) -> Self {
        Self {
            factor_source_to_intent_hashes,
            txid_to_petition: RefCell::new(txid_to_petition),
        }
    }

    pub(crate) fn outcome(self) -> SignaturesOutcome {
        let txid_to_petition = self.txid_to_petition.into_inner();
        let mut failed_transactions = MaybeSignedTransactions::empty();
        let mut successful_transactions = MaybeSignedTransactions::empty();
        let mut neglected_factor_sources = IndexSet::<NeglectedFactor>::new();
        for (intent_hash, petition_of_transaction) in
            txid_to_petition.into_iter()
        {
            let outcome = petition_of_transaction.outcome();
            let signatures = outcome.signatures;

            if outcome.transaction_valid {
                successful_transactions.add_signatures(intent_hash, signatures);
            } else {
                failed_transactions.add_signatures(intent_hash, signatures);
            }
            neglected_factor_sources.extend(outcome.neglected_factors)
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
        each: impl Fn(&PetitionForTransaction) -> T,
        combine: impl Fn(Vec<T>) -> U,
    ) -> U {
        let for_each = factor_source_ids
            .clone()
            .iter()
            .flat_map(|f| {
                self.factor_source_to_intent_hashes
                    .get(f)
                    .expect("Should be able to lookup intent hash for each factor source, did you call this method with irrelevant factor sources? Or did you recently change the preprocessor logic of the SignaturesCollector, if you did you've missed adding an entry for `factor_source_to_intent_hash`.map")
                    .iter()
                    .map(|intent_hash| {
                        let binding = self.txid_to_petition.borrow();
                        let value = binding.get(intent_hash).expect("Should have a petition for each transaction, did you recently change the preprocessor logic of the SignaturesCollector, if you did you've missed adding an entry for `txid_to_petition`.map");
                        each(value)
                    })
            }).collect_vec();
        combine(for_each)
    }

    pub(crate) fn invalid_transactions_if_neglected_factors(
        &self,
        factor_source_ids: IndexSet<FactorSourceIDFromHash>,
    ) -> IndexSet<InvalidTransactionIfNeglected> {
        self.each_petition(
            factor_source_ids.clone(),
            |p| {
                p.invalid_transaction_if_neglected_factors(
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
    pub(crate) fn input_for_interactor(
        &self,
        factor_source_id: &FactorSourceIDFromHash,
    ) -> MonoFactorSignRequestInput {
        self.each_petition(
            IndexSet::just(*factor_source_id),
            |p| {
                if p.has_tx_failed() {
                    None
                } else {
                    Some(p.input_for_interactor(factor_source_id))
                }
            },
            |i| {
                MonoFactorSignRequestInput::new(
                    *factor_source_id,
                    i.into_iter().flatten().collect::<IndexSet<_>>(),
                )
            },
        )
    }

    pub(crate) fn status(&self) -> PetitionsStatus {
        self.each_petition(
            self.factor_source_to_intent_hashes
                .keys()
                .cloned()
                .collect(),
            |p| p.status_of_each_petition_for_entity(),
            |i| PetitionsStatus::reducing(i.into_iter().flatten()),
        )
    }

    fn add_signature(&self, signature: &HDSignature) {
        let binding = self.txid_to_petition.borrow();
        let petition = binding.get(signature.intent_hash()).expect("Should have a petition for each transaction, did you recently change the preprocessor logic of the SignaturesCollector, if you did you've missed adding an entry for `txid_to_petition`.map");
        petition.add_signature(signature.clone())
    }

    fn neglect_factor_source_with_id(&self, neglected: NeglectedFactor) {
        self.each_petition(
            IndexSet::just(neglected.factor_source_id()),
            |p| p.neglect_factor_source(neglected.clone()),
            |_| (),
        )
    }

    pub(crate) fn process_batch_response(
        &self,
        response: SignWithFactorsOutcome,
    ) {
        match response {
            SignWithFactorsOutcome::Signed {
                produced_signatures,
            } => {
                for (k, v) in produced_signatures.signatures.clone().iter() {
                    info!("Signed with {} (#{} signatures)", k, v.len());
                }
                produced_signatures
                    .signatures
                    .values()
                    .flatten()
                    .for_each(|s| self.add_signature(s));
            }
            SignWithFactorsOutcome::Neglected(neglected_factors) => {
                let reason = neglected_factors.reason;
                for neglected_factor_source_id in
                    neglected_factors.content.iter()
                {
                    info!("Neglected {}", neglected_factor_source_id);
                    self.neglect_factor_source_with_id(NeglectedFactor::new(
                        reason,
                        *neglected_factor_source_id,
                    ))
                }
            }
        }
    }

    #[allow(unused)]
    fn debug_str(&self) -> String {
        self.txid_to_petition
            .borrow()
            .iter()
            .map(|p| format!("Petitions({:#?}: {:#?})", p.0, p.1))
            .join(" + ")
    }
}

impl HasSampleValues for Petitions {
    fn sample() -> Self {
        let p0 = PetitionForTransaction::sample();
        Self::new(
            HashMap::just((
                FactorSourceIDFromHash::sample_at(0),
                IndexSet::just(p0.intent_hash.clone()),
            )),
            IndexMap::just((p0.intent_hash.clone(), p0)),
        )
    }

    fn sample_other() -> Self {
        let p1 = PetitionForTransaction::sample();
        Self::new(
            HashMap::just((
                FactorSourceIDFromHash::sample_at(1),
                IndexSet::just(p1.intent_hash.clone()),
            )),
            IndexMap::just((p1.intent_hash.clone(), p1)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = Petitions;

    #[test]
    fn equality_of_samples() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality_of_samples() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }

    #[test]
    fn debug() {
        assert!(!format!("{:?}", Sut::sample()).is_empty());
    }
}
