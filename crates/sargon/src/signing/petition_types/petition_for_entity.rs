use crate::prelude::*;

/// Petition of signatures from an entity in a transaction.
/// Essentially a wrapper around a tuple
/// `{ threshold: PetitionForFactors, override: PetitionForFactors }`
#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
#[debug("{}", self.debug_str())]
pub(crate) struct PetitionForEntity {
    /// The owner of these factors
    pub(crate) entity: AddressOfAccountOrPersona,

    /// Index and hash of transaction
    pub(crate) intent_hash: IntentHash,

    /// Petition with threshold factors
    pub(crate) threshold_factors: Option<RefCell<PetitionForFactors>>,

    /// Petition with override factors
    pub(crate) override_factors: Option<RefCell<PetitionForFactors>>,
}

impl PetitionForEntity {
    pub(super) fn new(
        intent_hash: IntentHash,
        entity: AddressOfAccountOrPersona,
        threshold_factors: impl Into<Option<PetitionForFactors>>,
        override_factors: impl Into<Option<PetitionForFactors>>,
    ) -> Self {
        let threshold_factors = threshold_factors.into();
        let override_factors = override_factors.into();
        if threshold_factors.is_none() && override_factors.is_none() {
            panic!("Programmer error! Must have at least one factors list.");
        }
        Self {
            entity,
            intent_hash,
            threshold_factors: threshold_factors.map(RefCell::new),
            override_factors: override_factors.map(RefCell::new),
        }
    }

    /// Creates a new Petition from an entity which is securified, i.e. has a matrix of factors.
    pub(crate) fn new_securified(
        intent_hash: IntentHash,
        entity: AddressOfAccountOrPersona,
        matrix: GeneralRoleWithHierarchicalDeterministicFactorInstances,
    ) -> Self {
        Self::new(
            intent_hash,
            entity,
            PetitionForFactors::new_threshold(
                matrix.threshold_factors,
                matrix.threshold as i8,
            ),
            PetitionForFactors::new_override(matrix.override_factors),
        )
    }

    /// Creates a new Petition from an entity which is unsecurified, i.e. has a single factor.
    pub(crate) fn new_unsecurified(
        intent_hash: IntentHash,
        entity: AddressOfAccountOrPersona,
        instance: HierarchicalDeterministicFactorInstance,
    ) -> Self {
        Self::new(
            intent_hash,
            entity,
            PetitionForFactors::new_unsecurified(instance),
            None,
        )
    }

    /// Returns `true` if signatures requirement has been fulfilled, either by
    /// override factors or by threshold factors
    pub(crate) fn has_signatures_requirement_been_fulfilled(&self) -> bool {
        self.status()
            == PetitionForFactorsStatus::Finished(
                PetitionFactorsStatusFinished::Success,
            )
    }

    /// Returns `true` if the transaction of this petition already has failed due
    /// to too many factors neglected
    pub(crate) fn has_failed(&self) -> bool {
        self.status()
            == PetitionForFactorsStatus::Finished(
                PetitionFactorsStatusFinished::Fail,
            )
    }

    /// Returns the aggregate of **all** owned factor instances from both lists, either threshold or override.
    pub(crate) fn all_factor_instances(&self) -> IndexSet<OwnedFactorInstance> {
        self.access_both_list_then_form_union(|l| l.factor_instances())
            .into_iter()
            .map(|f| {
                OwnedFactorInstance::owned_factor_instance(
                    self.entity.clone(),
                    f.clone(),
                )
            })
            .collect::<IndexSet<_>>()
    }

    /// Returns the aggregate of all **neglected** factor instances from both lists, either threshold or override,
    /// that is, all factor instances but filtered out only those from FactorSources which have been neglected.
    pub(crate) fn all_neglected_factor_instances(
        &self,
    ) -> IndexSet<NeglectedFactorInstance> {
        self.access_both_list_then_form_union(|f| f.all_neglected())
    }

    /// Returns the aggregate of all **neglected** factor sources from both lists, either threshold or override.
    pub(crate) fn all_neglected_factor_sources(
        &self,
    ) -> IndexSet<NeglectedFactor> {
        self.all_neglected_factor_instances()
            .into_iter()
            .map(|n| n.as_neglected_factor())
            .collect::<IndexSet<_>>()
    }

    /// Returrns the aggregate of all signatures from both lists, either threshold or override.
    pub(crate) fn all_signatures(&self) -> IndexSet<HDSignature> {
        self.access_both_list_then_form_union(|f| f.all_signatures())
    }

    /// Mutates this petition by adding a signature to it. The signature is added to the relevant
    /// list, either threshold or override.
    ///
    /// # Panics
    /// Panics if this factor source has already been neglected or signed with.
    ///
    /// Or panics if the factor source is not known to this petition.
    pub(crate) fn add_signature(&self, signature: HDSignature) {
        self.access_both_list(|l| l.add_signature_if_relevant(&signature), |t, o| {
            match (t, o) {
                (Some(true), Some(true)) => {
                    unreachable!("Matrix of FactorInstances does not allow for a factor to be present in both threshold and override list, thus this will never happen.")
                }
                (Some(false), Some(false)) => panic!("Factor source not found in any of the lists."),
                (None, None) => panic!("Programmer error! Must have at least one factors list."),
                _ => (),
            }
        })
    }

    /// Queries if the authorization of the entity in this transaction already is irrelevant, since
    /// too many factors have been neglected.
    pub(crate) fn should_neglect_factors_due_to_irrelevant(
        &self,
        factor_source_ids: IndexSet<FactorSourceIDFromHash>,
    ) -> bool {
        assert!(self.references_any_factor_source(&factor_source_ids));
        match self.status() {
            PetitionForFactorsStatus::Finished(
                PetitionFactorsStatusFinished::Fail,
            ) => true,
            PetitionForFactorsStatus::Finished(
                PetitionFactorsStatusFinished::Success,
            ) => false,
            PetitionForFactorsStatus::InProgress => false,
        }
    }

    /// Returns this petitions entity if the transaction would be invalid if the given factor sources
    /// would be neglected.
    pub(crate) fn invalid_transaction_if_neglected_factors(
        &self,
        factor_source_ids: IndexSet<FactorSourceIDFromHash>,
    ) -> Option<AddressOfAccountOrPersona> {
        let status_if_neglected =
            self.status_if_neglected_factors(factor_source_ids);
        match status_if_neglected {
            PetitionForFactorsStatus::Finished(finished_reason) => {
                match finished_reason {
                    PetitionFactorsStatusFinished::Fail => {
                        Some(self.entity.clone())
                    }
                    PetitionFactorsStatusFinished::Success => None,
                }
            }
            PetitionForFactorsStatus::InProgress => None,
        }
    }

    pub(crate) fn status_if_neglected_factors(
        &self,
        factor_source_ids: IndexSet<FactorSourceIDFromHash>,
    ) -> PetitionForFactorsStatus {
        let simulation = self.clone();
        for factor_source_id in factor_source_ids.iter() {
            simulation.neglect_if_referenced(NeglectedFactor::new(
                NeglectFactorReason::Simulation,
                *factor_source_id,
            ))
        }
        simulation.status()
    }

    /// Queries if this petition references any of the factor sources in the set of ids
    /// by checking bot hteh threshold and the override factors list.
    pub(crate) fn references_any_factor_source(
        &self,
        factor_source_ids: &IndexSet<FactorSourceIDFromHash>,
    ) -> bool {
        factor_source_ids
            .iter()
            .any(|f| self.references_factor_source_with_id(f))
    }

    /// Queries if this petition references the factor source with the given id, by
    /// checking both the threshold and override factors list.
    pub(crate) fn references_factor_source_with_id(
        &self,
        id: &FactorSourceIDFromHash,
    ) -> bool {
        self.access_both_list(
            |p| p.references_factor_source_with_id(id),
            |a, b| a.unwrap_or(false) || b.unwrap_or(false),
        )
    }

    /// If this petitions references the neglected factor source, disregarding if it is a threshold
    /// or override factor, it will be neglected. If the factor is not known to any of the lists
    /// nothing happens.
    pub(crate) fn neglect_if_referenced(&self, neglected: NeglectedFactor) {
        self.access_both_list(
            |p| p.neglect_if_referenced(neglected.clone()),
            |_, _| (),
        );
    }

    /// The "aggregated" status of this petition, i.e. the status of the threshold factors
    /// and the status of the override factors merged together. E.g. (Threshold: InProgress, Override: InProgress) ->
    /// Inprogress. And (Threshold: Finished(Fail), Override: InProgress) -> InProgress,
    /// (Threshold: Finished(Fail), Override: Finished(Fail)) -> Finished(Fail) but
    /// (Threshold: Finished(Success), Override: Inprogress) -> Finished(Success) - since
    /// want to be able to finish early if the petition for this entity is already successful.
    pub(crate) fn status(&self) -> PetitionForFactorsStatus {
        use PetitionFactorsStatusFinished::*;
        use PetitionForFactorsStatus::*;

        self.access_both_list(
            |p| p.status(),
            |maybe_threshold, maybe_override| {
                if let Some(t) = &maybe_threshold {
                    trace!("Threshold factor status: {:?}", t);
                }
                if let Some(o) = &maybe_override {
                    trace!("Override factor status: {:?}", o);
                }
                match (maybe_threshold, maybe_override) {
                    (None, None) => {
                        panic!("Programmer error! Should have at least one factors list.")
                    }
                    (Some(threshold), None) => threshold,
                    (None, Some(r#override)) => r#override,
                    (Some(threshold), Some(r#override)) => match (threshold, r#override) {
                        (InProgress, InProgress) => PetitionForFactorsStatus::InProgress,
                        (Finished(Fail), InProgress) => PetitionForFactorsStatus::InProgress,
                        (InProgress, Finished(Fail)) => PetitionForFactorsStatus::InProgress,
                        (Finished(Fail), Finished(Fail)) => {
                            PetitionForFactorsStatus::Finished(Fail)
                        }
                        (Finished(Success), _) => PetitionForFactorsStatus::Finished(Success),
                        (_, Finished(Success)) => PetitionForFactorsStatus::Finished(Success),
                    },
                }
            },
        )
    }
}

// === Private ===
impl PetitionForEntity {
    /// Derefs and calls `access` on both lists respectively, if they exist. Then combines the results
    /// of each list access using `combine`.
    ///
    /// This method is useful when you want to read out state for both list and somehow combine
    /// that result, e.g. to form a union of all signatures - but not wanting to juggle `RefCell`
    /// and `Option` repeatedly.
    fn access_both_list<T, U>(
        &self,
        access: impl Fn(&PetitionForFactors) -> T,
        combine: impl Fn(Option<T>, Option<T>) -> U,
    ) -> U {
        let access_list_if_exists =
            |list: &Option<RefCell<PetitionForFactors>>| {
                list.as_ref().map(|refcell| access(&refcell.borrow()))
            };
        let t = access_list_if_exists(&self.threshold_factors);
        let o = access_list_if_exists(&self.override_factors);
        combine(t, o)
    }

    /// Derefes and calls `access` on both lists respectively, if they exist. The result of the `access`
    /// of each list is then combined together using `IndexSet::union` and returned.
    fn access_both_list_then_form_union<T>(
        &self,
        access: impl Fn(&PetitionForFactors) -> IndexSet<T>,
    ) -> IndexSet<T>
    where
        T: Eq + std::hash::Hash + Clone,
    {
        self.access_both_list(
            |l| access(l),
            |t, o| {
                t.unwrap_or_default()
                    .union(&o.unwrap_or_default())
                    .cloned()
                    .collect::<IndexSet<T>>()
            },
        )
    }

    #[allow(unused)]
    fn debug_str(&self) -> String {
        let thres: String = self
            .threshold_factors
            .clone()
            .map(|f| format!("threshold_factors {:#?}", f.borrow()))
            .unwrap_or_default();

        let overr: String = self
            .override_factors
            .clone()
            .map(|f| format!("override_factors {:#?}", f.borrow()))
            .unwrap_or_default();

        format!(
            "intent_hash: {:#?}, entity: {:#?}, {:#?}{:#?}",
            self.intent_hash, self.entity, thres, overr
        )
    }
}

// === SAMPLE VALUES ===
impl PetitionForEntity {
    fn from_entity_with_role_kind(
        entity: impl Into<AccountOrPersona>,
        intent_hash: IntentHash,
        role_kind: RoleKind,
    ) -> Self {
        let entity = entity.into();
        match entity.entity_security_state() {
            EntitySecurityState::Unsecured { value } => {
                Self::new_unsecurified(intent_hash, entity.address(), value.transaction_signing)
            }
            EntitySecurityState::Securified { value } => {
                Self::new_securified(
                    intent_hash,
                    entity.address(),
                    GeneralRoleWithHierarchicalDeterministicFactorInstances::try_from(
                        (value.security_structure.matrix_of_factors, role_kind)
                    ).unwrap()
                )
            }
        }
    }
}

impl HasSampleValues for PetitionForEntity {
    fn sample() -> Self {
        Self::from_entity_with_role_kind(
            Account::sample_securified_mainnet(
                "Grace",
                AccountAddress::sample_other(),
                || {
                    GeneralRoleWithHierarchicalDeterministicFactorInstances::m6(HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                        CAP26EntityKind::Account,
                        HDPathComponent::from(6)
                    ))
                },
            ),
            IntentHash::sample(),
            RoleKind::Primary,
        )
    }

    fn sample_other() -> Self {
        Self::from_entity_with_role_kind(
            Account::sample_unsecurified_mainnet(
                "Sample Unsec",
                HierarchicalDeterministicFactorInstance::sample_fi0(
                    CAP26EntityKind::Account,
                ),
            ),
            IntentHash::sample_other(),
            RoleKind::Primary,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Sut = PetitionForEntity;

    #[test]
    fn multiple_device_as_override_skipped_both_is_invalid() {
        let d0 = HierarchicalDeterministicFactorInstance::sample_fi0(
            CAP26EntityKind::Account,
        );
        let d1 = HierarchicalDeterministicFactorInstance::sample_fi10(
            CAP26EntityKind::Account,
        );
        assert_eq!(d0.factor_source_id.kind, FactorSourceKind::Device);
        assert_eq!(d1.factor_source_id.kind, FactorSourceKind::Device);

        let matrix =
            GeneralRoleWithHierarchicalDeterministicFactorInstances::override_only([d0.clone(), d1.clone()]);
        let entity =
            AddressOfAccountOrPersona::Account(AccountAddress::sample());
        let tx = IntentHash::new(Hash::sample_third(), NetworkID::Mainnet);
        let sut = Sut::new_securified(tx.clone(), entity.clone(), matrix);
        let invalid =
            sut.invalid_transaction_if_neglected_factors(IndexSet::from_iter(
                [d0.factor_source_id(), d1.factor_source_id()],
            ))
            .unwrap();

        assert_eq!(invalid.clone(), entity);
    }

    #[test]
    fn multiple_device_as_override_skipped_one_is_valid() {
        let d0 = HierarchicalDeterministicFactorInstance::sample_fi0(
            CAP26EntityKind::Account,
        );
        let d1 = HierarchicalDeterministicFactorInstance::sample_fi10(
            CAP26EntityKind::Account,
        );
        assert_eq!(d0.factor_source_id.kind, FactorSourceKind::Device);
        assert_eq!(d1.factor_source_id.kind, FactorSourceKind::Device);

        let matrix =
            GeneralRoleWithHierarchicalDeterministicFactorInstances::override_only(
                [d0.clone(), d1.clone()]
            );
        let entity =
            AddressOfAccountOrPersona::Account(AccountAddress::sample());
        let tx = IntentHash::new(Hash::sample_third(), NetworkID::Mainnet);
        let sut = Sut::new_securified(tx.clone(), entity.clone(), matrix);
        let invalid = sut.invalid_transaction_if_neglected_factors(
            IndexSet::just(d0.factor_source_id()),
        );
        assert!(invalid.is_none());
    }

    #[test]
    fn multiple_device_as_threshold_skipped_both_is_invalid() {
        let d0 = HierarchicalDeterministicFactorInstance::sample_fi0(
            CAP26EntityKind::Account,
        );
        let d1 = HierarchicalDeterministicFactorInstance::sample_fi10(
            CAP26EntityKind::Account,
        );
        assert_eq!(d0.factor_source_id.kind, FactorSourceKind::Device);
        assert_eq!(d1.factor_source_id.kind, FactorSourceKind::Device);

        let matrix = GeneralRoleWithHierarchicalDeterministicFactorInstances::threshold_only(
            [d0.clone(), d1.clone()],
            2,
        );

        let entity =
            AddressOfAccountOrPersona::Account(AccountAddress::sample());
        let tx = IntentHash::new(Hash::sample_third(), NetworkID::Mainnet);
        let sut = Sut::new_securified(tx.clone(), entity.clone(), matrix);
        let invalid =
            sut.invalid_transaction_if_neglected_factors(IndexSet::from_iter(
                [d0.factor_source_id(), d1.factor_source_id()],
            ))
            .unwrap();
        assert_eq!(invalid, entity);
    }

    #[test]
    fn two_device_as_threshold_of_2_skipped_one_is_invalid() {
        let d0 = HierarchicalDeterministicFactorInstance::sample_fi0(
            CAP26EntityKind::Account,
        );
        let d1 = HierarchicalDeterministicFactorInstance::sample_fi10(
            CAP26EntityKind::Account,
        );
        assert_eq!(d0.factor_source_id.kind, FactorSourceKind::Device);
        assert_eq!(d1.factor_source_id.kind, FactorSourceKind::Device);

        let matrix = GeneralRoleWithHierarchicalDeterministicFactorInstances::threshold_only(
            [d0.clone(), d1.clone()],
            2,
        );

        let entity =
            AddressOfAccountOrPersona::Account(AccountAddress::sample());
        let tx = IntentHash::new(Hash::sample_third(), NetworkID::Mainnet);
        let sut = Sut::new_securified(tx.clone(), entity.clone(), matrix);

        let invalid = sut
            .invalid_transaction_if_neglected_factors(IndexSet::just(
                d1.factor_source_id(),
            ))
            .unwrap();

        assert_eq!(invalid, entity);
    }

    #[test]
    fn two_device_as_threshold_of_1_skipped_one_is_valid() {
        let d0 = HierarchicalDeterministicFactorInstance::sample_fi0(
            CAP26EntityKind::Account,
        );
        let d1 = HierarchicalDeterministicFactorInstance::sample_fi10(
            CAP26EntityKind::Account,
        );
        assert_eq!(d0.factor_source_id.kind, FactorSourceKind::Device);
        assert_eq!(d1.factor_source_id.kind, FactorSourceKind::Device);

        let matrix = GeneralRoleWithHierarchicalDeterministicFactorInstances::threshold_only(
            [d0.clone(), d1.clone()],
            1,
        );

        let entity =
            AddressOfAccountOrPersona::Account(AccountAddress::sample());
        let tx = IntentHash::new(Hash::sample_third(), NetworkID::Mainnet);
        let sut = Sut::new_securified(tx.clone(), entity.clone(), matrix);

        let invalid = sut.invalid_transaction_if_neglected_factors(
            IndexSet::just(d1.factor_source_id()),
        );

        assert!(invalid.is_none());
    }

    #[test]
    fn debug() {
        assert!(!format!("{:?}", Sut::sample()).is_empty());
    }

    #[test]
    #[should_panic(
        expected = "Programmer error! Must have at least one factors list."
    )]
    fn invalid_empty_factors() {
        Sut::new(
            IntentHash::sample(),
            AddressOfAccountOrPersona::sample(),
            None,
            None,
        );
    }

    #[test]
    #[should_panic(expected = "Factor source not found in any of the lists.")]
    fn cannot_add_unrelated_signature() {
        let sut = Sut::sample();
        sut.add_signature(HDSignature::sample());
    }

    #[test]
    #[should_panic(
        expected = "A factor MUST NOT be present in both threshold AND override list."
    )]
    fn factor_should_not_be_used_in_both_lists() {
        Account::sample_securified_mainnet(
            "Alice",
            AccountAddress::sample(),
            || {
                let fi = HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                CAP26EntityKind::Account,
                HDPathComponent::from(0)
            );
                GeneralRoleWithHierarchicalDeterministicFactorInstances::new(
                    [FactorSourceIDFromHash::sample_at(0)].map(&fi),
                    1,
                    [FactorSourceIDFromHash::sample_at(0)].map(&fi),
                )
            },
        );
    }

    #[test]
    #[should_panic]
    fn cannot_add_same_signature_twice() {
        let intent_hash = IntentHash::sample();
        let entity = Account::sample_securified_mainnet(
            "Alice",
            AccountAddress::sample(),
            || {
                let fi = HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                CAP26EntityKind::Account,
                HDPathComponent::from(0)
            );
                GeneralRoleWithHierarchicalDeterministicFactorInstances::new(
                    [FactorSourceIDFromHash::sample_at(0)].map(&fi),
                    1,
                    [FactorSourceIDFromHash::sample_at(1)].map(&fi),
                )
            },
        );
        let sut = Sut::from_entity_with_role_kind(
            entity.clone(),
            intent_hash.clone(),
            RoleKind::Primary,
        );

        let signature = FactorSourceIDFromHash::sample_at(0)
            .sample_tx_hd_signature(intent_hash, HDPathComponent::from(0));

        sut.add_signature(signature.clone());
        sut.add_signature(signature.clone());
    }

    #[test]
    fn invalid_transactions_if_neglected_success() {
        let sut = Sut::sample();
        let signature = FactorSourceIDFromHash::sample_at(1)
            .sample_tx_hd_signature(
                sut.intent_hash.clone(),
                HDPathComponent::from(6),
            );
        sut.add_signature(signature);
        let can_skip = |f: FactorSourceIDFromHash| {
            assert!(sut
                // Already signed with override factor `FactorSourceIDFromHash::fs1()`. Thus
                // can skip
                .invalid_transaction_if_neglected_factors(IndexSet::just(f))
                .is_none())
        };
        can_skip(FactorSourceIDFromHash::sample_at(0));
        can_skip(FactorSourceIDFromHash::sample_at(3));
        can_skip(FactorSourceIDFromHash::sample_at(4));
        can_skip(FactorSourceIDFromHash::sample_at(5));
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other())
    }

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }
}
