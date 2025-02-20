use std::ops::Deref;

use crate::prelude::*;
pub trait CrossRoleSkipOutcomeAnalyzer<ID: SignableID> {
    fn invalid_transaction_if_neglected_factors(
        &self,
        signable: ID,
        skipped_factor_source_ids: IndexSet<FactorSourceIDFromHash>,
        petitions: Vec<PetitionForEntity<ID>>,
    ) -> Option<InvalidTransactionIfNeglected<ID>>;
}

/// Petition of signatures for a transaction.
/// Essentially a wrapper around `Iterator<Item = PetitionForEntity>`.
#[derive(derive_more::Debug)]
#[debug("{}", self.debug_str())]
pub struct PetitionForTransaction<S: Signable> {
    /// Transaction to sign
    pub(crate) signable: S,

    pub for_entities:
        RwLock<HashMap<AddressOfAccountOrPersona, PetitionForEntity<S::ID>>>,
}

impl<S: Signable> Clone for PetitionForTransaction<S> {
    fn clone(&self) -> Self {
        Self {
            signable: self.signable.clone(),
            for_entities: RwLock::new(
                self.for_entities
                    .read()
                    .expect("PetitionForTransaction lock should not have been poisoned.")
                    .clone(),
            ),
        }
    }
}

impl<S: Signable> PartialEq for PetitionForTransaction<S> {
    fn eq(&self, other: &Self) -> bool {
        self.signable == other.signable
            && self
                .for_entities
                .read()
                .expect("PetitionForTransaction lock should not have been poisoned.")
                .deref()
                == other
                    .for_entities
                    .read()
                    .expect("PetitionForTransaction lock should not have been poisoned.")
                    .deref()
    }
}

impl<S: Signable> Eq for PetitionForTransaction<S> {}

impl<S: Signable> PetitionForTransaction<S> {
    pub(crate) fn new(
        signable: S,
        for_entities: HashMap<
            AddressOfAccountOrPersona,
            PetitionForEntity<S::ID>,
        >,
    ) -> Self {
        Self {
            signable,
            for_entities: RwLock::new(for_entities),
        }
    }

    /// Returns `(true, _)` if this transaction has been successfully signed by
    /// all required factor instances.
    ///
    /// Returns `(false, _)` if not enough factor instances have signed.
    ///
    /// The second value in the tuple `(_, IndexSet<HDSignature>, _)` contains all
    /// the signatures, even if it the transaction was failed, all signatures
    /// will be returned (which might be empty).
    ///
    /// The third value in the tuple `(_, _, IndexSet<FactorSourceIDFromHash>)` contains the
    /// id of all the factor sources which was skipped.
    pub(crate) fn outcome(self) -> PetitionTransactionOutcome<S::ID> {
        let for_entities = self
            .for_entities
            .read()
            .expect(
                "PetitionForTransaction lock should not have been poisoned.",
            )
            .values()
            .map(|x| x.to_owned())
            .collect_vec();

        let transaction_valid = for_entities
            .iter()
            .all(|b| b.has_signatures_requirement_been_fulfilled());

        let signatures = for_entities
            .iter()
            .flat_map(|x| x.all_signatures())
            .collect::<IndexSet<_>>();

        let flat_neglected_factors = for_entities
            .iter()
            .flat_map(|x| x.all_neglected_factor_sources())
            .collect::<IndexSet<NeglectedFactor>>();

        let outcome =
            PetitionTransactionOutcome::new(
                transaction_valid,
                self.signable.get_id(),
                signatures,
                for_entities
                    .iter()
                    .map(|v| (v.entity, v.all_neglected_factor_sources()))
                    .collect::<IndexMap<
                        AddressOfAccountOrPersona,
                        IndexSet<NeglectedFactor>,
                    >>(),
            );
        assert_eq!(flat_neglected_factors, outcome.neglected_factors());
        outcome
    }

    pub(crate) fn has_tx_failed(&self) -> bool {
        self.for_entities
            .read()
            .expect(
                "PetitionForTransaction lock should not have been poisoned.",
            )
            .values()
            .any(|p| p.has_failed())
    }

    pub(crate) fn all_relevant_factor_instances_of_source(
        &self,
        factor_source_id: &FactorSourceIDFromHash,
    ) -> IndexSet<OwnedFactorInstance> {
        assert!(!self.has_tx_failed());
        self.for_entities
            .read()
            .expect(
                "PetitionForTransaction lock should not have been poisoned.",
            )
            .values()
            .filter(|&p| {
                if p.has_failed() {
                    debug!("OMITTING petition since it HAS failed: {:?}", p);
                    false
                } else {
                    debug!(
                        "INCLUDING petition since it has NOT failed: {:?}",
                        p
                    );
                    true
                }
            })
            .cloned()
            .flat_map(|petition| petition.all_factor_instances())
            .filter(|f| f.factor_source_id() == *factor_source_id)
            .collect()
    }

    pub(crate) fn add_signature(&self, signature: HDSignature<S::ID>) {
        let for_entities = self.for_entities.write().expect(
            "PetitionForTransaction lock should not have been poisoned.",
        );
        let for_entity = for_entities
            .get(&signature.owned_factor_instance().owner)
            .expect("Should not have added signature to irrelevant PetitionForTransaction, did you pass the wrong signature to the wrong PetitionForTransaction?");
        for_entity.add_signature(signature.clone());
    }

    pub(crate) fn neglect_factor_source(&self, neglected: NeglectedFactor) {
        let mut for_entities = self.for_entities.write().expect(
            "PetitionForTransaction lock should not have been poisoned.",
        );
        for petition in for_entities.values_mut() {
            petition.neglect_if_referenced(neglected.clone())
        }
    }

    pub(crate) fn input_for_interactor(
        &self,
        factor_source_id: &FactorSourceIDFromHash,
    ) -> TransactionSignRequestInput<S> {
        assert!(!self.should_neglect_factors_due_to_irrelevant(
            IndexSet::just(*factor_source_id)
        ));
        assert!(!self.has_tx_failed());
        TransactionSignRequestInput::new(
            self.signable.get_payload(),
            *factor_source_id,
            self.all_relevant_factor_instances_of_source(factor_source_id),
        )
    }

    pub(crate) fn status_of_each_petition_for_entity(
        &self,
    ) -> Vec<PetitionForFactorsStatus> {
        self.for_entities
            .read()
            .expect(
                "PetitionForTransaction lock should not have been poisoned.",
            )
            .values()
            .map(|petition| petition.status())
            .collect()
    }

    pub(crate) fn invalid_transaction_if_neglected_factors(
        &self,
        cross_role_skip_outcome_analyzer: Arc<
            dyn CrossRoleSkipOutcomeAnalyzer<S::ID>,
        >,
        factor_source_ids: IndexSet<FactorSourceIDFromHash>,
    ) -> Option<InvalidTransactionIfNeglected<S::ID>> {
        if self.has_tx_failed() {
            // No need to display already failed tx.
            return None;
        }

        let petitions = self
            .for_entities
            .read()
            .expect(
                "PetitionForTransaction lock should not have been poisoned.",
            )
            .iter()
            .map(|(_, petition)| petition.clone())
            .collect_vec();

        cross_role_skip_outcome_analyzer
            .invalid_transaction_if_neglected_factors(
                self.signable.get_id(),
                factor_source_ids,
                petitions,
            )
    }

    pub(crate) fn should_neglect_factors_due_to_irrelevant(
        &self,
        factor_source_ids: IndexSet<FactorSourceIDFromHash>,
    ) -> bool {
        self.for_entities
            .read()
            .expect(
                "PetitionForTransaction lock should not have been poisoned.",
            )
            .values()
            .filter(|&p| p.references_any_factor_source(&factor_source_ids))
            .cloned()
            .all(|petition| {
                petition.should_neglect_factors_due_to_irrelevant(
                    factor_source_ids.clone(),
                )
            })
    }

    #[allow(unused)]
    fn debug_str(&self) -> String {
        let entities = self
            .for_entities
            .read()
            .expect(
                "PetitionForTransaction lock should not have been poisoned.",
            )
            .iter()
            .map(|p| format!("PetitionForEntity({:#?})", p.1))
            .join(", ");

        format!("PetitionForTransaction(for_entities: [{}])", entities)
    }
}

impl<S: Signable + ProvidesSamplesByBuildingManifest> HasSampleValues
    for PetitionForTransaction<S>
{
    fn sample() -> Self {
        let account = Account::sample_securified_mainnet(
            "Grace",
            6,
            HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_unsecurified_at_index(0),
            || {
                GeneralRoleWithHierarchicalDeterministicFactorInstances::r6(
                HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                    CAP26EntityKind::Account,
                    Hardened::from_local_key_space(6u32, IsSecurified(true)).unwrap(),
                ))
            },
        );

        let signable = S::sample_entities_requiring_auth([&account], []);
        Self::new(
            signable.clone(),
            HashMap::just((
                AddressOfAccountOrPersona::from(account.address),
                PetitionForEntity::new(
                    signable.get_id(),
                    AddressOfAccountOrPersona::from(account.address),
                    PetitionForFactors::sample(),
                    PetitionForFactors::sample_other(),
                ),
            )),
        )
    }

    fn sample_other() -> Self {
        let persona = Persona::sample_unsecurified_mainnet(
            "Sample Unsec",
            HierarchicalDeterministicFactorInstance::sample_fii0(),
        );
        let signable = S::sample_entities_requiring_auth([], [&persona]);
        Self::new(
            signable.clone(),
            HashMap::just((
                AddressOfAccountOrPersona::Identity(persona.address),
                PetitionForEntity::new(
                    signable.get_id(),
                    AddressOfAccountOrPersona::Identity(persona.address),
                    PetitionForFactors::sample_other(),
                    None,
                ),
            )),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PetitionForTransaction<TransactionIntent>;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn debug() {
        assert!(!format!("{:?}", SUT::sample()).is_empty());
    }

    #[test]
    fn all_relevant_factor_instances_of_source_ok() {
        let account = Account::sample_at(5);
        let intent = TransactionIntent::sample_entity_addresses_requiring_auth(
            [account.address],
            [],
        );
        let matrix = match &account.security_state {
            EntitySecurityState::Securified { value } => {
                value.security_structure.matrix_of_factors.clone()
            }
            _ => panic!(),
        };
        let petition = PetitionForEntity::new_securified(
            intent.transaction_intent_hash(),
            AddressOfAccountOrPersona::from(account.address),
            GeneralRoleWithHierarchicalDeterministicFactorInstances::try_from(
                (matrix, RoleKind::Primary),
            )
            .unwrap(),
        );

        let sut = SUT::new(
            intent,
            HashMap::just((
                AddressOfAccountOrPersona::from(account.address),
                petition,
            )),
        );
        sut.neglect_factor_source(NeglectedFactor::new(
            NeglectFactorReason::Failure,
            FactorSourceIDFromHash::sample_at(1),
        ));

        assert_eq!(
            sut.all_relevant_factor_instances_of_source(
                &FactorSourceIDFromHash::sample_at(4)
            )
            .len(),
            1
        );
    }

    #[test]
    #[should_panic]
    fn all_relevant_factor_instances_of_source_panics_if_invalid() {
        let intent_hash = TransactionIntentHash::sample();

        let account = Account::sample_at(5);
        let intent =
            TransactionIntent::sample_entities_requiring_auth([&account], []);
        let matrix = match &account.security_state {
            EntitySecurityState::Securified { value } => {
                value.security_structure.matrix_of_factors.clone()
            }
            _ => panic!(),
        };
        let petition = PetitionForEntity::new_securified(
            intent_hash,
            AddressOfAccountOrPersona::from(account.address),
            GeneralRoleWithHierarchicalDeterministicFactorInstances::try_from(
                (matrix, RoleKind::Primary),
            )
            .unwrap(),
        );

        let sut = SUT::new(
            intent.clone(),
            HashMap::just((
                AddressOfAccountOrPersona::from(account.address),
                petition,
            )),
        );
        sut.neglect_factor_source(NeglectedFactor::new(
            NeglectFactorReason::Failure,
            FactorSourceIDFromHash::sample_at(1),
        ));
        sut.neglect_factor_source(NeglectedFactor::new(
            NeglectFactorReason::Failure,
            FactorSourceIDFromHash::sample_at(4),
        ));
        let _ = sut.all_relevant_factor_instances_of_source(
            &FactorSourceIDFromHash::sample_at(4),
        );
    }
}
