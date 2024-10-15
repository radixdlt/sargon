use crate::prelude::*;

pub(crate) struct SignaturesCollectorPreprocessor {
    transactions: IndexSet<TXToSign>,
}

pub(crate) fn sort_group_factors(
    used_factor_sources: HashSet<FactorSource>,
) -> IndexSet<FactorSourcesOfKind> {
    let factors_of_kind: HashMap<FactorSourceKind, IndexSet<FactorSource>> =
        used_factor_sources
            .into_iter()
            .into_grouping_map_by(|x| x.factor_source_kind())
            .collect::<IndexSet<_>>();

    let mut factors_of_kind = factors_of_kind
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().sorted().collect::<IndexSet<_>>()))
        .collect::<IndexMap<FactorSourceKind, IndexSet<FactorSource>>>();

    factors_of_kind.sort_keys();

    factors_of_kind
        .into_iter()
        .map(|(k, v)| {
            FactorSourcesOfKind::new(k, v)
                .expect("All factors should be of the same kind, since this is calling iter on a Map, using kind as key. Did you just move around lines of code?")
        })
        .collect::<IndexSet<_>>()
}

pub struct ExtractorOfInstancesRequiredToSignTransactions;
impl ExtractorOfInstancesRequiredToSignTransactions {
    pub fn extract(
        profile: &Profile,
        transactions: Vec<TransactionIntent>,
        for_any_securified_entity_select_role: RoleKind,
    ) -> Result<IndexSet<HierarchicalDeterministicFactorInstance>> {
        let preprocessor =
            SignaturesCollectorPreprocessor::analyzing_transaction_intents(
                profile,
                transactions,
            )?;
        let (petitions, _) = preprocessor.preprocess(
            IndexSet::from_iter(profile.factor_sources.iter()),
            for_any_securified_entity_select_role,
        );

        let factor_instances = petitions
            .txid_to_petition
            .borrow()
            .values()
            .flat_map(|p| {
                p.for_entities
                    .borrow()
                    .values()
                    .flat_map(|p| p.all_factor_instances())
                    .collect::<Vec<_>>()
            })
            .map(|p| p.factor_instance().clone())
            .collect::<IndexSet<HierarchicalDeterministicFactorInstance>>();
        Ok(factor_instances)
    }
}

impl SignaturesCollectorPreprocessor {
    fn analyzing_transaction_intents(
        profile: &Profile,
        transactions: Vec<TransactionIntent>,
    ) -> Result<Self> {
        let transactions = transactions
            .into_iter()
            .map(|i| TXToSign::extracting_from_intent_and_profile(&i, profile))
            .collect::<Result<IndexSet<TXToSign>>>()?;

        Ok(Self::new(transactions))
    }

    pub(super) fn new(transactions: IndexSet<TXToSign>) -> Self {
        Self { transactions }
    }

    pub(super) fn preprocess(
        self,
        profile_factor_sources: IndexSet<FactorSource>,
        role_kind: RoleKind,
    ) -> (Petitions, IndexSet<FactorSourcesOfKind>) {
        let transactions = self.transactions;
        let mut petitions_for_all_transactions =
            IndexMap::<IntentHash, PetitionForTransaction>::new();

        // We care for only the factor sources which are HD based
        let mut all_factor_sources_in_profile =
            HashMap::<FactorSourceIDFromHash, FactorSource>::new();
        profile_factor_sources.into_iter().for_each(|f| {
            if let Some(id) = f.factor_source_id().as_hash() {
                all_factor_sources_in_profile.insert(*id, f);
            }
        });

        let mut factor_to_payloads =
            HashMap::<FactorSourceIDFromHash, IndexSet<IntentHash>>::new();

        let mut used_factor_sources = HashSet::<FactorSource>::new();

        let mut register_factor_in_tx =
            |id: &FactorSourceIDFromHash, txid: &IntentHash| {
                if let Some(ref mut txids) = factor_to_payloads.get_mut(id) {
                    txids.insert(txid.clone());
                } else {
                    factor_to_payloads
                        .insert(*id, IndexSet::just(txid.clone()));
                }

                assert!(!factor_to_payloads.is_empty());

                let factor_source = all_factor_sources_in_profile
                    .get(id)
                    .expect("Should have all factor sources");
                used_factor_sources.insert(factor_source.clone());

                assert!(!used_factor_sources.is_empty());
            };

        for transaction in transactions.into_iter() {
            let mut petitions_for_entities =
                HashMap::<AddressOfAccountOrPersona, PetitionForEntity>::new();

            for entity in transaction.entities_requiring_auth() {
                let address = entity.address();
                let petition = PetitionForEntity::new_from_entity(
                    transaction.intent_hash.clone(),
                    entity,
                    role_kind,
                );

                petition.all_factor_instances().iter().for_each(|f| {
                    register_factor_in_tx(
                        &f.factor_source_id(),
                        &transaction.intent_hash,
                    )
                });
                petitions_for_entities.insert(address, petition);
            }

            let petition_of_tx = PetitionForTransaction::new(
                transaction.intent_hash.clone(),
                petitions_for_entities,
            );

            petitions_for_all_transactions
                .insert(transaction.intent_hash, petition_of_tx);
        }

        let factors_of_kind = sort_group_factors(used_factor_sources);

        let petitions =
            Petitions::new(factor_to_payloads, petitions_for_all_transactions);

        (petitions, factors_of_kind)
    }
}
