use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct MaybeSignedTransactions<ID: SignableID> {
    /// Collection of transactions which might be signed or not.
    pub(super) transactions: IndexMap<ID, IndexSet<HDSignature<ID>>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SignedTransaction<ID: SignableID> {
    /// The transaction intent hash.
    pub signable_id: ID,
    /// The signatures for this transaction.
    pub signatures: IndexSet<HDSignature<ID>>,
}

impl<ID: SignableID> SignedTransaction<ID> {
    pub(crate) fn new(
        signable_id: ID,
        signatures: IndexSet<HDSignature<ID>>,
    ) -> Self {
        Self {
            signable_id,
            signatures,
        }
    }
}

impl<ID: SignableID> MaybeSignedTransactions<ID> {
    fn new(transactions: IndexMap<ID, IndexSet<HDSignature<ID>>>) -> Self {
        Self { transactions }
    }

    /// Constructs a new empty `MaybeSignedTransactions` which can be used
    /// as a "builder".
    pub(crate) fn empty() -> Self {
        Self::new(IndexMap::new())
    }

    /// Returns whether or not this `MaybeSignedTransactions` contains
    /// any transactions.
    pub(crate) fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }

    pub(crate) fn transactions(&self) -> Vec<SignedTransaction<ID>> {
        self.transactions
            .clone()
            .into_iter()
            .map(|(k, v)| SignedTransaction::new(k, v))
            .collect_vec()
    }

    /// Validates that all values, all signatures, have the same `intent_hash`
    /// as its key.
    ///
    /// Also validates that the input of every signature is unique - to identify
    /// if the same signer has been used twice, would be a programmer error.
    ///
    /// # Panics
    /// Panics if any signature has a different `intent_hash` than its key.
    fn validate(&self) {
        for (signable_id, signatures) in self.transactions.iter() {
            assert!(
                signatures.iter().all(|s| s.payload_id() == signable_id),
                "Discrepancy between intent hash and signature intent hash."
            );
        }
        let all_signatures = self.all_signatures();
        let all_signatures_count = all_signatures.len();
        let inputs = self
            .all_signatures()
            .iter()
            .map(|s| s.input.clone())
            .collect::<IndexSet<_>>();
        assert_eq!(
            all_signatures_count,
            inputs.len(),
            "Discrepancy, the same signer has been used twice."
        );
    }

    /// Inserts a set of signatures for transaction with `intent_hash`, if
    /// the transaction was already present, the signatures are added to the
    /// existing set, if the transaction was not already present a new set is
    /// created.
    ///
    /// # Panics
    /// Panics if any signature has a different `intent_hash` than its key.
    ///
    /// Panics if any signatures in `signature` is not new, that is, already present
    /// in `transactions`.
    pub(crate) fn add_signatures(
        &mut self,
        signable_id: ID,
        signatures: IndexSet<HDSignature<ID>>,
    ) {
        if let Some(ref mut sigs) = self.transactions.get_mut(&signable_id) {
            let old_count = sigs.len();
            let delta_count = signatures.len();
            sigs.extend(signatures);
            assert_eq!(
                sigs.len(),
                old_count + delta_count,
                "Discrepancy, some signature in signatures to add found in existing set."
            );
        } else {
            self.transactions.insert(signable_id, signatures);
        }
        self.validate();
    }

    /// Returns all the signatures for all the transactions.
    pub(crate) fn all_signatures(&self) -> IndexSet<HDSignature<ID>> {
        self.transactions
            .values()
            .flat_map(|v| v.iter())
            .cloned()
            .collect()
    }
}

impl<ID: SignableID + HasSampleValues> HasSampleValues
    for MaybeSignedTransactions<ID>
{
    fn sample() -> Self {
        let tx_a = ID::sample();

        let tx_a_input_x = HDSignatureInput::new(
            tx_a.clone(),
            OwnedFactorInstance::new(
                AddressOfAccountOrPersona::sample(),
                HierarchicalDeterministicFactorInstance::sample_mainnet_tx_account(
                    Hardened::from_local_key_space_unsecurified(0u32).unwrap(),
                    FactorSourceIDFromHash::sample(),
                ),
            ),
        );
        let tx_a_input_y = HDSignatureInput::new(
            tx_a.clone(),
            OwnedFactorInstance::new(
                AddressOfAccountOrPersona::sample(),
                HierarchicalDeterministicFactorInstance::sample_mainnet_tx_account(
                    Hardened::from_local_key_space_unsecurified(1u32).unwrap(),
                    FactorSourceIDFromHash::sample_other(),
                ),
            ),
        );
        let tx_a_sig_x =
            HDSignature::fake_sign_by_looking_up_mnemonic_amongst_samples(
                tx_a_input_x,
            );
        let tx_a_sig_y =
            HDSignature::fake_sign_by_looking_up_mnemonic_amongst_samples(
                tx_a_input_y,
            );

        let tx_b = ID::sample_other();
        let tx_b_input_x = HDSignatureInput::new(
            tx_b.clone(),
            OwnedFactorInstance::new(
                AddressOfAccountOrPersona::sample(),
                HierarchicalDeterministicFactorInstance::sample_mainnet_tx_account(
                    Hardened::from_local_key_space_unsecurified(2u32).unwrap(),
                    FactorSourceIDFromHash::sample_at(3),
                ),
            ),
        );
        let tx_b_input_y = HDSignatureInput::new(
            tx_b.clone(),
            OwnedFactorInstance::new(
                AddressOfAccountOrPersona::sample(),
                HierarchicalDeterministicFactorInstance::sample_mainnet_tx_account(
                    Hardened::from_local_key_space_unsecurified(3u32).unwrap(),
                    FactorSourceIDFromHash::sample_at(4),
                ),
            ),
        );

        let tx_b_sig_x =
            HDSignature::fake_sign_by_looking_up_mnemonic_amongst_samples(
                tx_b_input_x,
            );
        let tx_b_sig_y =
            HDSignature::fake_sign_by_looking_up_mnemonic_amongst_samples(
                tx_b_input_y,
            );

        Self::new(
            [
                (tx_a, IndexSet::from_iter([tx_a_sig_x, tx_a_sig_y])),
                (tx_b, IndexSet::from_iter([tx_b_sig_x, tx_b_sig_y])),
            ]
            .into_iter()
            .collect::<IndexMap<ID, IndexSet<HDSignature<ID>>>>(),
        )
    }

    fn sample_other() -> Self {
        let tx_a = ID::sample_other();

        let tx_a_input_x = HDSignatureInput::new(
            tx_a.clone(),
            OwnedFactorInstance::new(
                AddressOfAccountOrPersona::sample(),
                HierarchicalDeterministicFactorInstance::sample_mainnet_tx_account(
                    Hardened::from_local_key_space_unsecurified(10u32).unwrap(),
                    FactorSourceIDFromHash::sample(),
                ),
            ),
        );
        let tx_a_input_y = HDSignatureInput::new(
            tx_a.clone(),
            OwnedFactorInstance::new(
                AddressOfAccountOrPersona::sample(),
                HierarchicalDeterministicFactorInstance::sample_mainnet_tx_account(
                    Hardened::from_local_key_space_unsecurified(11u32).unwrap(),
                    FactorSourceIDFromHash::sample_other(),
                ),
            ),
        );
        let tx_a_input_z = HDSignatureInput::new(
            tx_a.clone(),
            OwnedFactorInstance::new(
                AddressOfAccountOrPersona::sample(),
                HierarchicalDeterministicFactorInstance::sample_mainnet_tx_account(
                    Hardened::from_local_key_space_unsecurified(12u32).unwrap(),
                    FactorSourceIDFromHash::sample_at(3),
                ),
            ),
        );
        let tx_a_sig_x =
            HDSignature::fake_sign_by_looking_up_mnemonic_amongst_samples(
                tx_a_input_x,
            );
        let tx_a_sig_y =
            HDSignature::fake_sign_by_looking_up_mnemonic_amongst_samples(
                tx_a_input_y,
            );
        let tx_a_sig_z =
            HDSignature::fake_sign_by_looking_up_mnemonic_amongst_samples(
                tx_a_input_z,
            );

        Self::new(
            [(
                tx_a,
                IndexSet::from_iter([tx_a_sig_x, tx_a_sig_y, tx_a_sig_z]),
            )]
            .into_iter()
            .collect::<IndexMap<ID, IndexSet<HDSignature<ID>>>>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = MaybeSignedTransactions<TransactionIntentHash>;

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
    #[should_panic(
        expected = "Discrepancy, some signature in signatures to add found in existing set."
    )]
    fn panics_when_adding_same_signature() {
        let mut sut = SUT::sample();
        let tx = TransactionIntentHash::sample();
        let input = HDSignatureInput::new(
            tx,
            OwnedFactorInstance::new(
                AddressOfAccountOrPersona::sample(),
                HierarchicalDeterministicFactorInstance::sample_mainnet_tx_account(
                    Hardened::from_local_key_space_unsecurified(U31::ZERO).unwrap(),
                    FactorSourceIDFromHash::sample(),
                ),
            ),
        );
        let signature =
            unsafe { HDSignature::produced_signing_with_input(input) };

        sut.add_signatures(tx, IndexSet::from_iter([signature]));
    }

    #[test]
    #[should_panic(
        expected = "Discrepancy between intent hash and signature intent hash."
    )]
    fn panics_when_intent_hash_key_does_not_match_signature() {
        let mut sut = SUT::sample();
        let tx = TransactionIntentHash::sample();

        let input = HDSignatureInput::new(
            tx,
            OwnedFactorInstance::new(
                AddressOfAccountOrPersona::sample(),
                HierarchicalDeterministicFactorInstance::sample_mainnet_tx_account(
                    Hardened::from_local_key_space_unsecurified(0u32).unwrap(),
                    FactorSourceIDFromHash::sample(),
                ),
            ),
        );
        let signature =
            unsafe { HDSignature::produced_signing_with_input(input) };

        sut.add_signatures(
            TransactionIntentHash::sample_other(),
            IndexSet::from_iter([signature]),
        );
    }

    #[test]
    #[should_panic(
        expected = "Discrepancy, the same signer has been used twice."
    )]
    fn panics_when_same_signer_used_twice() {
        let mut sut = SUT::empty();
        let factor_instance = OwnedFactorInstance::sample();
        let tx = TransactionIntentHash::sample();
        let input = HDSignatureInput::new(tx, factor_instance.clone());
        let sig_a = HDSignature {
            input: input.clone(),
            signature: SignatureWithPublicKey::sample(),
        };
        let sig_b = HDSignature {
            input: input.clone(),
            signature: SignatureWithPublicKey::sample_other(),
        };
        sut.add_signatures(tx, IndexSet::from_iter([sig_a, sig_b]));
    }
}
