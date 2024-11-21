use crate::prelude::*;

/// A batch of keys (derivation paths) all being factor instances of a HDFactorSource
/// with id `factor_source_id` to sign a single transaction with, which hash
/// is `intent_hash`.
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct TransactionSignRequestInput<S: Signable> {
    /// Compiled Intent
    payload: S::Payload,

    /// ID of factor to use to sign
    pub(crate) factor_source_id: FactorSourceIDFromHash,

    /// The derivation paths to use to derive the private keys to sign with. The
    /// `factor_source_id` of each item must match `factor_source_id`.
    owned_factor_instances: Vec<OwnedFactorInstance>,
}

impl<S: Signable> TransactionSignRequestInput<S> {
    /// # Panics
    /// Panics if any of the owned factor instances does not match the `factor_source_id`.
    ///
    /// Panics if `owned_factor_instances` is empty.
    pub(crate) fn new(
        payload: S::Payload,
        factor_source_id: FactorSourceIDFromHash,
        owned_factor_instances: IndexSet<OwnedFactorInstance>,
    ) -> Self {
        assert!(
            !owned_factor_instances.is_empty(),
            "Invalid input, `owned_factor_instances` must not be empty."
        );
        assert!(owned_factor_instances
                    .iter()
                    .all(|f| f.by_factor_source(factor_source_id)), "Discrepancy! Mismatch between FactorSourceID of owned factor instances and specified FactorSourceID, this is a programmer error.");
        Self {
            payload,
            factor_source_id,
            owned_factor_instances: owned_factor_instances
                .into_iter()
                .collect_vec(),
        }
    }

    #[allow(unused)]
    pub fn signature_inputs(&self) -> IndexSet<HDSignatureInput<S::ID>> {
        let payload_id: S::ID = self.payload.clone().into();
        self.owned_factor_instances
            .clone()
            .into_iter()
            .map(|fi| HDSignatureInput::new(payload_id.clone(), fi))
            .collect()
    }
}

impl<S: Signable> HasSampleValues for TransactionSignRequestInput<S> {
    fn sample() -> Self {
        let owned_factor_instance = OwnedFactorInstance::sample();
        let factor_source_id = &owned_factor_instance.factor_source_id();
        Self::new(
            S::Payload::sample(),
            *factor_source_id,
            IndexSet::just(owned_factor_instance),
        )
    }

    fn sample_other() -> Self {
        let owned_factor_instance = OwnedFactorInstance::sample_other();
        let factor_source_id = &owned_factor_instance.factor_source_id();
        Self::new(
            S::Payload::sample_other(),
            *factor_source_id,
            IndexSet::just(owned_factor_instance),
        )
    }
}

#[cfg(test)]
mod tests_batch_req {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionSignRequestInput<TransactionIntent>;

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
    #[should_panic(
        expected = "Invalid input, `owned_factor_instances` must not be empty."
    )]
    fn panics_if_owned_factors_is_empty() {
        SUT::new(
            CompiledTransactionIntent::sample(),
            FactorSourceIDFromHash::sample(),
            IndexSet::new(),
        );
    }

    #[test]
    #[should_panic(
        expected = "Discrepancy! Mismatch between FactorSourceID of owned factor instances and specified FactorSourceID, this is a programmer error."
    )]
    fn panics_mismatch_factor_source_id() {
        SUT::new(
            CompiledTransactionIntent::sample_other(),
            FactorSourceIDFromHash::sample_other(),
            IndexSet::just(OwnedFactorInstance::sample_other()),
        );
    }
}
