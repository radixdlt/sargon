use crate::prelude::*;

/// Petition of signatures from an entity in a transaction.
/// Essentially a wrapper around a tuple
/// `{ threshold: PetitionForFactors, override: PetitionForFactors }`
#[derive(derive_more::Debug)]
#[debug("{}", self.debug_str())]
pub struct PetitionForEntity<ID: SignableID> {
    /// The owner of these factors
    pub(crate) entity: AddressOfAccountOrPersona,

    /// Index and hash of transaction
    pub(crate) payload_id: ID,

    /// Petition with threshold factors
    pub(crate) threshold_factors: Option<RwLock<PetitionForFactors<ID>>>,

    /// Petition with override factors
    pub(crate) override_factors: Option<RwLock<PetitionForFactors<ID>>>,
}

impl<ID: SignableID> Clone for PetitionForEntity<ID> {
    fn clone(&self) -> Self {
        Self {
            entity: self.entity,
            payload_id: self.payload_id.clone(),
            threshold_factors: self.all_threshold_factors().map(RwLock::new),
            override_factors: self.all_override_factors().map(RwLock::new),
        }
    }
}

impl<ID: SignableID> PartialEq for PetitionForEntity<ID> {
    fn eq(&self, other: &Self) -> bool {
        self.entity == other.entity
            && self.payload_id == other.payload_id
            && self.all_threshold_factors() == other.all_threshold_factors()
            && self.all_override_factors() == other.all_override_factors()
    }
}

impl<ID: SignableID> Eq for PetitionForEntity<ID> {}

impl<ID: SignableID> PetitionForEntity<ID> {
    pub(super) fn new(
        payload_id: ID,
        entity: AddressOfAccountOrPersona,
        threshold_factors: impl Into<Option<PetitionForFactors<ID>>>,
        override_factors: impl Into<Option<PetitionForFactors<ID>>>,
    ) -> Self {
        let threshold_factors = threshold_factors.into();
        let override_factors = override_factors.into();
        if threshold_factors.is_none() && override_factors.is_none() {
            panic!("Programmer error! Must have at least one factors list.");
        }
        Self {
            entity,
            payload_id,
            threshold_factors: threshold_factors.map(RwLock::new),
            override_factors: override_factors.map(RwLock::new),
        }
    }

    pub(crate) fn new_from_entity(
        payload_id: ID,
        entity: AccountOrPersona,
        purpose: SigningPurpose,
    ) -> Self {
        match entity.entity_security_state() {
            EntitySecurityState::Unsecured { value } => {
                // Transaction signing factor instance is used in both transaction and rola purposes
                let factor_instance = value.transaction_signing;

                Self::new_unsecurified(
                    payload_id,
                    entity.address(),
                    factor_instance,
                )
            }
            EntitySecurityState::Securified { value } => match purpose {
                SigningPurpose::SignTX { role_kind } => {
                    let general_role =
                            GeneralRoleWithHierarchicalDeterministicFactorInstances::try_from(
                                (value.security_structure.matrix_of_factors, role_kind)
                            ).unwrap();
                    Self::new_securified(
                        payload_id,
                        entity.address(),
                        general_role,
                    )
                }
                SigningPurpose::ROLA => {
                    let threshold_factors =
                        PetitionForFactors::<ID>::new_threshold(
                            vec![value.authentication_signing_factor_instance()],
                            1,
                        );
                    Self::new(
                        payload_id,
                        entity.address(),
                        threshold_factors,
                        None,
                    )
                }
            },
        }
    }

    /// Creates a new Petition from an entity which is securified, i.e. has a matrix of factors.
    pub(crate) fn new_securified(
        payload_id: ID,
        entity: AddressOfAccountOrPersona,
        role_with_factor_instances: GeneralRoleWithHierarchicalDeterministicFactorInstances,
    ) -> Self {
        Self::new(
            payload_id,
            entity,
            PetitionForFactors::new_threshold(
                role_with_factor_instances.get_threshold_factors(),
                role_with_factor_instances.get_threshold() as i8,
            ),
            PetitionForFactors::new_override(
                role_with_factor_instances.get_override_factors(),
            ),
        )
    }

    /// Creates a new Petition from an entity which is unsecurified, i.e. has a single factor.
    pub(crate) fn new_unsecurified(
        payload_id: ID,
        entity: AddressOfAccountOrPersona,
        instance: HierarchicalDeterministicFactorInstance,
    ) -> Self {
        Self::new(
            payload_id,
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
    pub fn all_factor_instances(&self) -> IndexSet<OwnedFactorInstance> {
        self.access_both_list_then_form_union(|l| l.factor_instances())
            .into_iter()
            .map(|f| {
                OwnedFactorInstance::owned_factor_instance(
                    self.entity,
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

    /// Returns the aggregate of all signatures from both lists, either threshold or override.
    pub(crate) fn all_signatures(&self) -> IndexSet<HDSignature<ID>> {
        self.access_both_list_then_form_union(|f| f.all_signatures())
    }

    /// Mutates this petition by adding a signature to it. The signature is added to the relevant
    /// list, either threshold or override.
    ///
    /// # Panics
    /// Panics if this factor source has already been neglected or signed with.
    ///
    /// Or panics if the factor source is not known to this petition.
    pub(crate) fn add_signature(&self, signature: HDSignature<ID>) {
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
                    PetitionFactorsStatusFinished::Fail => Some(self.entity),
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

    fn all_threshold_factors(&self) -> Option<PetitionForFactors<ID>> {
        self.threshold_factors.as_ref().map(|lock| {
            lock.read()
                .expect("PetitionForEntity lock should not have been poisoned.")
                .clone()
        })
    }

    fn all_override_factors(&self) -> Option<PetitionForFactors<ID>> {
        self.override_factors.as_ref().map(|lock| {
            lock.read()
                .expect("PetitionForEntity lock should not have been poisoned.")
                .clone()
        })
    }
}

// === Private ===
impl<ID: SignableID> PetitionForEntity<ID> {
    /// Derefs and calls `access` on both lists respectively, if they exist. Then combines the results
    /// of each list access using `combine`.
    ///
    /// This method is useful when you want to read out state for both list and somehow combine
    /// that result, e.g. to form a union of all signatures - but not wanting to juggle `RefCell`
    /// and `Option` repeatedly.
    fn access_both_list<T, U>(
        &self,
        access: impl Fn(&PetitionForFactors<ID>) -> T,
        combine: impl Fn(Option<T>, Option<T>) -> U,
    ) -> U {
        let access_list_if_exists =
            |list: &Option<RwLock<PetitionForFactors<ID>>>| {
                list.as_ref().map(|rwlock| {
                    access(&rwlock.read().expect(
                        "PetitionForEntity lock should not have been poisoned.",
                    ))
                })
            };
        let t = access_list_if_exists(&self.threshold_factors);
        let o = access_list_if_exists(&self.override_factors);
        combine(t, o)
    }

    /// Derefes and calls `access` on both lists respectively, if they exist. The result of the `access`
    /// of each list is then combined together using `IndexSet::union` and returned.
    fn access_both_list_then_form_union<T>(
        &self,
        access: impl Fn(&PetitionForFactors<ID>) -> IndexSet<T>,
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
            .all_threshold_factors()
            .map(|f| format!("threshold_factors {:#?}", f))
            .unwrap_or_default();

        let overr: String = self
            .all_override_factors()
            .map(|f| format!("override_factors {:#?}", f))
            .unwrap_or_default();

        format!(
            "intent_hash: {:#?}, entity: {:#?}, {:#?}{:#?}",
            self.payload_id, self.entity, thres, overr
        )
    }
}

// === SAMPLE VALUES ===
impl<ID: SignableID> PetitionForEntity<ID> {
    fn from_entity_with_role_kind(
        entity: impl Into<AccountOrPersona>,
        id: ID,
        role_kind: RoleKind,
    ) -> Self {
        let entity = entity.into();
        match entity.entity_security_state() {
            EntitySecurityState::Unsecured { value } => {
                Self::new_unsecurified(id, entity.address(), value.transaction_signing)
            }
            EntitySecurityState::Securified { value } => {
                Self::new_securified(
                    id,
                    entity.address(),
                    GeneralRoleWithHierarchicalDeterministicFactorInstances::try_from(
                        (value.security_structure.matrix_of_factors, role_kind)
                    ).unwrap()
                )
            }
        }
    }
}

impl<ID: SignableID + HasSampleValues> HasSampleValues
    for PetitionForEntity<ID>
{
    fn sample() -> Self {
        Self::from_entity_with_role_kind(
            Account::sample_securified_mainnet(
                "Grace",
                6,
                HierarchicalDeterministicFactorInstance::sample_fii10(),
                || {
                    GeneralRoleWithHierarchicalDeterministicFactorInstances::r6(HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                        CAP26EntityKind::Account,
                        Hardened::from_local_key_space(6u32, IsSecurified(true)).unwrap(),
                    ))
                },
            ),
            ID::sample(),
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
            ID::sample_other(),
            RoleKind::Primary,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(clippy::upper_case_acronyms)]
    type SUT = PetitionForEntity<TransactionIntentHash>;

    #[test]
    fn multiple_device_as_override_skipped_both_is_invalid() {
        let d0 = HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(0);
        let d1 = HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_securified_at_index(0);
        assert_eq!(d0.factor_source_id.kind, FactorSourceKind::Device);
        assert_eq!(d1.factor_source_id.kind, FactorSourceKind::Device);

        let matrix =
            GeneralRoleWithHierarchicalDeterministicFactorInstances::with_factors_and_role(RoleKind::Primary, [], 0, [d0.clone(), d1.clone()]).unwrap();
        let entity = AddressOfAccountOrPersona::from(AccountAddress::sample());
        let tx = TransactionIntentHash::new(
            Hash::sample_third(),
            NetworkID::Mainnet,
        );
        let sut = SUT::new_securified(tx.clone(), entity, matrix);
        let invalid =
            sut.invalid_transaction_if_neglected_factors(IndexSet::from_iter(
                [d0.factor_source_id(), d1.factor_source_id()],
            ))
            .unwrap();

        assert_eq!(invalid.clone(), entity);
    }

    #[test]
    fn multiple_device_as_override_skipped_one_is_valid() {
        let d0 = HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(0);
        let d1 = HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_securified_at_index(0);
        assert_eq!(d0.factor_source_id.kind, FactorSourceKind::Device);
        assert_eq!(d1.factor_source_id.kind, FactorSourceKind::Device);

        let matrix =
            GeneralRoleWithHierarchicalDeterministicFactorInstances::with_factors_and_role(RoleKind::Primary, [], 0,
                [d0.clone(), d1.clone()]
            ).unwrap();
        let entity = AddressOfAccountOrPersona::from(AccountAddress::sample());
        let tx = TransactionIntentHash::new(
            Hash::sample_third(),
            NetworkID::Mainnet,
        );
        let sut = SUT::new_securified(tx.clone(), entity, matrix);
        let invalid = sut.invalid_transaction_if_neglected_factors(
            IndexSet::just(d0.factor_source_id()),
        );
        assert!(invalid.is_none());
    }

    #[test]
    fn multiple_device_as_threshold_skipped_both_is_invalid() {
        let d0 = HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(0);
        let d1 = HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_securified_at_index(0);
        assert_eq!(d0.factor_source_id.kind, FactorSourceKind::Device);
        assert_eq!(d1.factor_source_id.kind, FactorSourceKind::Device);

        let matrix = GeneralRoleWithHierarchicalDeterministicFactorInstances::with_factors_and_role(
            RoleKind::Primary,
            [d0.clone(), d1.clone()],
            2,[]
        ).unwrap();

        let entity = AddressOfAccountOrPersona::from(AccountAddress::sample());
        let tx = TransactionIntentHash::new(
            Hash::sample_third(),
            NetworkID::Mainnet,
        );
        let sut = SUT::new_securified(tx.clone(), entity, matrix);
        let invalid =
            sut.invalid_transaction_if_neglected_factors(IndexSet::from_iter(
                [d0.factor_source_id(), d1.factor_source_id()],
            ))
            .unwrap();
        assert_eq!(invalid, entity);
    }

    #[test]
    fn two_device_as_threshold_of_2_skipped_one_is_invalid() {
        let d0 = HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(0);
        let d1 = HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_securified_at_index(0);
        assert_eq!(d0.factor_source_id.kind, FactorSourceKind::Device);
        assert_eq!(d1.factor_source_id.kind, FactorSourceKind::Device);

        let matrix = GeneralRoleWithHierarchicalDeterministicFactorInstances::with_factors_and_role(
            RoleKind::Primary,
            [d0.clone(), d1.clone()],
            2,
            []
        ).unwrap();

        let entity = AddressOfAccountOrPersona::from(AccountAddress::sample());
        let tx = TransactionIntentHash::new(
            Hash::sample_third(),
            NetworkID::Mainnet,
        );
        let sut = SUT::new_securified(tx.clone(), entity, matrix);

        let invalid = sut
            .invalid_transaction_if_neglected_factors(IndexSet::just(
                d1.factor_source_id(),
            ))
            .unwrap();

        assert_eq!(invalid, entity);
    }

    #[test]
    fn two_device_as_threshold_of_1_skipped_one_is_valid() {
        let d0 = HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(0);
        let d1 = HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_securified_at_index(0);
        assert_eq!(d0.factor_source_id.kind, FactorSourceKind::Device);
        assert_eq!(d1.factor_source_id.kind, FactorSourceKind::Device);

        let matrix = GeneralRoleWithHierarchicalDeterministicFactorInstances::with_factors_and_role(
            RoleKind::Primary,
            [d0.clone(), d1.clone()],
            1,
            []
        ).unwrap();

        let entity = AddressOfAccountOrPersona::from(AccountAddress::sample());
        let tx = TransactionIntentHash::new(
            Hash::sample_third(),
            NetworkID::Mainnet,
        );
        let sut = SUT::new_securified(tx.clone(), entity, matrix);

        let invalid = sut.invalid_transaction_if_neglected_factors(
            IndexSet::just(d1.factor_source_id()),
        );

        assert!(invalid.is_none());
    }

    #[test]
    fn debug() {
        assert!(!format!("{:?}", SUT::sample()).is_empty());
    }

    #[test]
    #[should_panic(
        expected = "Programmer error! Must have at least one factors list."
    )]
    fn invalid_empty_factors() {
        SUT::new(
            TransactionIntentHash::sample(),
            AddressOfAccountOrPersona::sample(),
            None,
            None,
        );
    }

    #[test]
    #[should_panic(expected = "Factor source not found in any of the lists.")]
    fn cannot_add_unrelated_signature() {
        let sut = SUT::sample();
        sut.add_signature(HDSignature::sample());
    }

    #[test]
    #[should_panic]
    fn cannot_add_same_signature_twice() {
        let intent_hash = TransactionIntentHash::sample();
        let entity = Account::sample_securified_mainnet(
            "Alice",
            0,
            HierarchicalDeterministicFactorInstance::sample_fii10(),
            || {
                let fi = HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                CAP26EntityKind::Account,
                Hardened::from_local_key_space(U31::ZERO, IsSecurified(true)).unwrap(),
            );
                GeneralRoleWithHierarchicalDeterministicFactorInstances::with_factors_and_role(
                    RoleKind::Primary,
                    [FactorSourceIDFromHash::sample_at(0)].map(&fi),
                    1,
                    [FactorSourceIDFromHash::sample_at(1)].map(&fi),
                )
                .unwrap()
            },
        );
        let sut = SUT::from_entity_with_role_kind(
            entity.clone(),
            intent_hash.clone(),
            RoleKind::Primary,
        );

        let sign_input = HDSignatureInput::new(
            intent_hash,
            OwnedFactorInstance::new(
                AddressOfAccountOrPersona::from(entity.address),
                HierarchicalDeterministicFactorInstance::sample_mainnet_tx_account(
                    Hardened::from_local_key_space(U31::ZERO, IsSecurified(true)).unwrap(),
                    FactorSourceIDFromHash::sample_at(0),
                ),
            ),
        );
        let signature =
            unsafe { HDSignature::produced_signing_with_input(sign_input) };

        sut.add_signature(signature.clone());
        sut.add_signature(signature.clone());
    }

    #[test]
    fn invalid_transactions_if_neglected_success() {
        let sut = SUT::sample();
        let signature = unsafe {
            HDSignature::produced_signing_with_input(
            HDSignatureInput::new(
                sut.payload_id.clone(),
                OwnedFactorInstance::new(
                    sut.entity,
                    HierarchicalDeterministicFactorInstance::sample_mainnet_tx_account(
                        Hardened::from_local_key_space(U31::try_from(6u32).unwrap(), IsSecurified(true)).unwrap(),
                        FactorSourceIDFromHash::sample_at(1),
                    ),
                ),
            )
        )
        };
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
        assert_ne!(SUT::sample(), SUT::sample_other())
    }

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }
}
