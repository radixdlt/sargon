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

impl SignaturesCollectorPreprocessor {
    pub(super) fn new(transactions: IndexSet<TXToSign>) -> Self {
        Self { transactions }
    }

    pub(super) fn preprocess(
        self,
        profile_factor_sources: IndexSet<FactorSource>,
        role_kind: RoleKind
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
                match entity.entity_security_state() {
                    EntitySecurityState::Unsecured { value } => {
                        let factor_instance = value.transaction_signing; // TODO should change according to payload

                        let factor_source_id = factor_instance.factor_source_id;
                        register_factor_in_tx(
                            &factor_source_id,
                            &transaction.intent_hash,
                        );

                        let petition = PetitionForEntity::new_unsecurified(
                            transaction.intent_hash.clone(),
                            address.clone(),
                            factor_instance,
                        );
                        petitions_for_entities
                            .insert(address.clone(), petition);
                    }
                    EntitySecurityState::Securified { value } => {
                        let general_role =
                            GeneralRoleWithHierarchicalDeterministicFactorInstances::try_from(
                                (value.security_structure.matrix_of_factors, role_kind.clone())
                            ).unwrap();

                        let mut add = |factors: Vec<
                            HierarchicalDeterministicFactorInstance,
                        >| {
                            factors.into_iter().for_each(|f| {
                                let factor_source_id = f.factor_source_id;
                                register_factor_in_tx(
                                    &factor_source_id,
                                    &transaction.intent_hash,
                                );
                            })
                        };

                        add(general_role.override_factors.clone());
                        add(general_role.threshold_factors.clone());
                        let petition = PetitionForEntity::new_securified(
                            transaction.intent_hash.clone(),
                            address.clone(),
                            general_role.into(),
                        );
                        petitions_for_entities
                            .insert(address.clone(), petition);
                    }
                }
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
