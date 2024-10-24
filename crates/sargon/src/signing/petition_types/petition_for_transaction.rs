use crate::prelude::*;

/// Petition of signatures for a transaction.
/// Essentially a wrapper around `Iterator<Item = PetitionForEntity>`.
#[derive(derive_more::Debug, PartialEq, Eq)]
#[debug("{}", self.debug_str())]
pub(crate) struct PetitionForTransaction {
    /// Transaction to sign
    pub(crate) intent: TransactionIntent,

    pub(crate) for_entities:
        RefCell<HashMap<AddressOfAccountOrPersona, PetitionForEntity>>,
}

impl PetitionForTransaction {
    pub(crate) fn new(
        intent: TransactionIntent,
        for_entities: HashMap<AddressOfAccountOrPersona, PetitionForEntity>,
    ) -> Self {
        Self {
            intent,
            for_entities: RefCell::new(for_entities),
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
    pub(crate) fn outcome(self) -> PetitionTransactionOutcome {
        let for_entities = self
            .for_entities
            .into_inner()
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

        let neglected_factors = for_entities
            .iter()
            .flat_map(|x| x.all_neglected_factor_sources())
            .collect::<IndexSet<NeglectedFactor>>();

        PetitionTransactionOutcome::new(
            transaction_valid,
            self.intent.transaction_intent_hash(),
            signatures,
            neglected_factors,
        )
    }

    pub(crate) fn has_tx_failed(&self) -> bool {
        self.for_entities.borrow().values().any(|p| p.has_failed())
    }

    pub(crate) fn all_relevant_factor_instances_of_source(
        &self,
        factor_source_id: &FactorSourceIDFromHash,
    ) -> IndexSet<OwnedFactorInstance> {
        assert!(!self.has_tx_failed());
        self.for_entities
            .borrow()
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

    pub(crate) fn add_signature(&self, signature: HDSignature) {
        let for_entities = self.for_entities.borrow_mut();
        let for_entity = for_entities
            .get(&signature.owned_factor_instance().owner)
            .expect("Should not have added signature to irrelevant PetitionForTransaction, did you pass the wrong signature to the wrong PetitionForTransaction?");
        for_entity.add_signature(signature.clone());
    }

    pub(crate) fn neglect_factor_source(&self, neglected: NeglectedFactor) {
        let mut for_entities = self.for_entities.borrow_mut();
        for petition in for_entities.values_mut() {
            petition.neglect_if_referenced(neglected.clone())
        }
    }

    pub(crate) fn input_for_interactor(
        &self,
        factor_source_id: &FactorSourceIDFromHash,
    ) -> TransactionSignRequestInput {
        assert!(!self.should_neglect_factors_due_to_irrelevant(
            IndexSet::just(*factor_source_id)
        ));
        assert!(!self.has_tx_failed());
        TransactionSignRequestInput::new(
            self.intent.compile(),
            *factor_source_id,
            self.all_relevant_factor_instances_of_source(factor_source_id),
        )
    }

    pub(crate) fn status_of_each_petition_for_entity(
        &self,
    ) -> Vec<PetitionForFactorsStatus> {
        self.for_entities
            .borrow()
            .values()
            .map(|petition| petition.status())
            .collect()
    }

    pub(crate) fn invalid_transaction_if_neglected_factors(
        &self,
        factor_source_ids: IndexSet<FactorSourceIDFromHash>,
    ) -> Option<InvalidTransactionIfNeglected> {
        if self.has_tx_failed() {
            // No need to display already failed tx.
            return None;
        }
        let entities = self
            .for_entities
            .borrow()
            .iter()
            .filter_map(|(_, petition)| {
                petition.invalid_transaction_if_neglected_factors(
                    factor_source_ids.clone(),
                )
            })
            .collect_vec();

        if entities.is_empty() {
            return None;
        }

        Some(InvalidTransactionIfNeglected::new(
            self.intent.transaction_intent_hash(),
            entities,
        ))
    }

    pub(crate) fn should_neglect_factors_due_to_irrelevant(
        &self,
        factor_source_ids: IndexSet<FactorSourceIDFromHash>,
    ) -> bool {
        self.for_entities
            .borrow()
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
            .borrow()
            .iter()
            .map(|p| format!("PetitionForEntity({:#?})", p.1))
            .join(", ");

        format!("PetitionForTransaction(for_entities: [{}])", entities)
    }
}

impl HasSampleValues for PetitionForTransaction {
    fn sample() -> Self {
        let account = Account::sample_securified_mainnet(
            "Grace",
            AccountAddress::sample_other(),
            || {
                GeneralRoleWithHierarchicalDeterministicFactorInstances::r6(
                    HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                        CAP26EntityKind::Account,
                        HDPathComponent::from(6)
                    )
                )
            },
        );

        let intent =
            TransactionIntent::sample_entities_requiring_auth([&account], []);
        println!("{}", intent.manifest.instructions.to_string());
        Self::new(
            intent.clone(),
            HashMap::just((
                AddressOfAccountOrPersona::from(account.address),
                PetitionForEntity::new(
                    intent.transaction_intent_hash(),
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
        let intent =
            TransactionIntent::sample_entities_requiring_auth([], [&persona]);
        Self::new(
            intent.clone(),
            HashMap::just((
                AddressOfAccountOrPersona::Identity(persona.address),
                PetitionForEntity::new(
                    intent.transaction_intent_hash(),
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

    type Sut = PetitionForTransaction;

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }

    #[test]
    fn debug() {
        assert!(!format!("{:?}", Sut::sample()).is_empty());
    }

    #[test]
    fn all_relevant_factor_instances_of_source_ok() {
        let account = Account::sample_at(5);
        let intent = TransactionIntent::sample_entity_addresses_requiring_auth(
            [account.address],
            [],
        );
        let matrix = match account.security_state {
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

        let sut = Sut::new(
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
        let matrix = match account.security_state {
            EntitySecurityState::Securified { value } => {
                value.security_structure.matrix_of_factors.clone()
            }
            _ => panic!(),
        };
        let petition = PetitionForEntity::new_securified(
            intent_hash.clone(),
            AddressOfAccountOrPersona::from(account.address),
            GeneralRoleWithHierarchicalDeterministicFactorInstances::try_from(
                (matrix, RoleKind::Primary),
            )
            .unwrap(),
        );

        let sut = Sut::new(
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
