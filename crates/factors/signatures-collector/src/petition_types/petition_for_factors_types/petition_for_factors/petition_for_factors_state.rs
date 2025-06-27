use super::*;
use crate::prelude::*;

#[derive(derive_more::Debug)]
#[debug("PetitionForFactorsState(signed: {:?}, neglected: {:?})", self.all_signatures(), self.all_neglected())]
pub(crate) struct PetitionForFactorsState<ID: SignableID> {
    /// Factors that have signed.
    signed: RwLock<PetitionForFactorsSubState<HDSignature<ID>>>,

    /// Neglected factors, either due to user explicitly skipping, or due
    /// implicitly neglected to failure.
    neglected: RwLock<PetitionForFactorsSubState<NeglectedFactorInstance>>,
}

impl<ID: SignableID> PartialEq for PetitionForFactorsState<ID> {
    fn eq(&self, other: &Self) -> bool {
        self.all_signatures() == other.all_signatures()
            && self.all_neglected() == other.all_neglected()
    }
}

impl<ID: SignableID> Eq for PetitionForFactorsState<ID> {}

impl<ID: SignableID> PetitionForFactorsState<ID> {
    /// Creates a new `PetitionForFactorsState`.
    pub(super) fn new() -> Self {
        Self {
            signed: RwLock::new(PetitionForFactorsSubState::<_>::new()),
            neglected: RwLock::new(PetitionForFactorsSubState::<_>::new()),
        }
    }

    /// A set of signatures from factors that have been signed with so far.
    pub(crate) fn all_signatures(&self) -> IndexSet<HDSignature<ID>> {
        self.signed
            .read()
            .expect(
                "PetitionForFactorsState lock should not have been poisoned",
            )
            .snapshot()
    }

    /// A set factors have been neglected so far.
    pub(crate) fn all_neglected(&self) -> IndexSet<NeglectedFactorInstance> {
        self.neglected
            .read()
            .expect(
                "PetitionForFactorsState lock should not have been poisoned",
            )
            .snapshot()
    }

    /// # Panics
    /// Panics if this factor source has already been neglected or signed with.
    fn assert_not_referencing_factor_source(
        &self,
        factor_source_id: FactorSourceIDFromHash,
    ) {
        assert!(
            !self.references_factor_source_by_id(factor_source_id),
            "Programmer error! Factor source {:#?} already used, should only be referenced once.",
            factor_source_id,
        );
    }

    /// # Panics
    /// Panics if this factor source has already been neglected or signed and
    /// this is not a simulation.
    pub(crate) fn neglect(&self, neglected: &NeglectedFactorInstance) {
        if neglected.reason != NeglectFactorReason::Simulation {
            self.assert_not_referencing_factor_source(
                neglected.factor_source_id(),
            );
        }
        self.neglected
            .write()
            .expect(
                "PetitionForFactorsState lock should not have been poisoned",
            )
            .insert(neglected);
    }

    /// # Panics
    /// Panics if this factor source has already been neglected or signed with.
    pub(crate) fn add_signature(&self, signature: &HDSignature<ID>) {
        self.assert_not_referencing_factor_source(signature.factor_source_id());
        self.signed
            .write()
            .expect(
                "PetitionForFactorsState lock should not have been poisoned",
            )
            .insert(signature)
    }

    pub(super) fn snapshot(&self) -> PetitionForFactorsStateSnapshot<ID> {
        PetitionForFactorsStateSnapshot::new(
            self.all_signatures(),
            self.all_neglected(),
        )
    }

    pub(super) fn cloned(&self) -> Self {
        Self {
            signed: RwLock::new(
                self.signed
                    .read()
                    .expect("PetitionForFactorsState lock should not have been poisoned")
                    .cloned(),
            ),
            neglected: RwLock::new(
                self.neglected
                    .read()
                    .expect("PetitionForFactorsState lock should not have been poisoned")
                    .cloned(),
            ),
        }
    }

    fn references_factor_source_by_id(
        &self,
        factor_source_id: FactorSourceIDFromHash,
    ) -> bool {
        self.signed
            .read()
            .expect("PetitionForFactorsState lock should not have been poisoned")
            .references_factor_source_by_id(factor_source_id)
            || self
                .neglected
                .read()
                .expect("PetitionForFactorsState lock should not have been poisoned")
                .references_factor_source_by_id(factor_source_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PetitionForFactorsState<TransactionIntentHash>;

    impl PetitionForFactorsState<TransactionIntentHash> {
        fn test_neglect(
            &self,
            id: &HierarchicalDeterministicFactorInstance,
            simulated: bool,
        ) {
            self.neglect(&NeglectedFactorInstance::new(
                if simulated {
                    NeglectFactorReason::Simulation
                } else {
                    NeglectFactorReason::UserExplicitlySkipped
                },
                id.clone(),
            ))
        }
    }

    #[test]
    #[should_panic]
    fn skipping_twice_panics() {
        let sut = SUT::new();
        let fi = HierarchicalDeterministicFactorInstance::sample();
        sut.test_neglect(&fi, false);
        sut.test_neglect(&fi, false);
    }

    #[test]
    #[should_panic]
    fn signing_twice_panics() {
        let sut = SUT::new();
        let sig = HDSignature::sample();
        sut.add_signature(&sig);
        sut.add_signature(&sig);
    }

    #[test]
    #[should_panic]
    fn skipping_already_signed_panics() {
        let sut = SUT::new();

        let intent_hash = TransactionIntentHash::sample();

        let factor_instance =
            HierarchicalDeterministicFactorInstance::sample_mainnet_tx_account(
                Hardened::from_local_key_space_unsecurified(U31::ZERO).unwrap(),
                FactorSourceIDFromHash::sample_at(0),
            );
        let sign_input = HDSignatureInput::new(
            intent_hash,
            OwnedFactorInstance::new(
                AddressOfAccountOrPersona::sample(),
                factor_instance.clone(),
            ),
        );
        let signature =
            unsafe { HDSignature::produced_signing_with_input(sign_input) };

        sut.add_signature(&signature);

        sut.test_neglect(&factor_instance, false);
    }

    #[test]
    #[should_panic]
    fn signing_already_skipped_panics() {
        let sut = SUT::new();

        let intent_hash = TransactionIntentHash::sample();
        let factor_instance =
            HierarchicalDeterministicFactorInstance::sample_mainnet_tx_account(
                Hardened::from_local_key_space_unsecurified(U31::ZERO).unwrap(),
                FactorSourceIDFromHash::sample_at(0),
            );

        sut.test_neglect(&factor_instance, false);

        let sign_input = HDSignatureInput::new(
            intent_hash,
            OwnedFactorInstance::new(
                AddressOfAccountOrPersona::sample(),
                factor_instance.clone(),
            ),
        );

        let signature =
            unsafe { HDSignature::produced_signing_with_input(sign_input) };

        sut.add_signature(&signature);
    }
}
