use crate::prelude::*;

/// A batch of keys (derivation paths) all being factor instances of a HDFactorSource
/// with id `factor_source_id` to sign a single transaction with, which hash
/// is `intent_hash`.
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct TransactionSignRequestInput<SP: SignablePayload> {
    /// Compiled Intent
    payload: SP,

    /// ID of factor to use to sign
    pub(crate) factor_source_id: FactorSourceIDFromHash,

    /// The derivation paths to use to derive the private keys to sign with. The
    /// `factor_source_id` of each item must match `factor_source_id`.
    owned_factor_instances: Vec<OwnedFactorInstance>,
}

impl <SP: SignablePayload> TransactionSignRequestInput<SP> {
    /// # Panics
    /// Panics if any of the owned factor instances does not match the `factor_source_id`.
    ///
    /// Panics if `owned_factor_instances` is empty.
    pub(crate) fn new(
        payload: SP,
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
    pub fn signature_inputs(&self) -> IndexSet<HDSignatureInput<SP::PayloadId>> {
        let payload_id = self.payload.get_payload_id();
        self.owned_factor_instances
            .clone()
            .into_iter()
            .map(|fi| HDSignatureInput::new(payload_id.clone(), fi))
            .collect()
    }
}

impl HasSampleValues for TransactionSignRequestInput<CompiledTransactionIntent> {
    fn sample() -> Self {
        let owned_factor_instance = OwnedFactorInstance::sample();
        let factor_source_id = &owned_factor_instance.factor_source_id();
        Self::new(
            CompiledTransactionIntent::sample(),
            *factor_source_id,
            IndexSet::just(owned_factor_instance),
        )
    }

    fn sample_other() -> Self {
        let owned_factor_instance = OwnedFactorInstance::sample_other();
        let factor_source_id = &owned_factor_instance.factor_source_id();
        Self::new(
            CompiledTransactionIntent::sample_other(),
            *factor_source_id,
            IndexSet::just(owned_factor_instance),
        )
    }
}

#[cfg(test)]
mod tests_batch_req {
    use super::*;

    type Sut = TransactionSignRequestInput<CompiledTransactionIntent>;

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
    #[should_panic(
        expected = "Invalid input, `owned_factor_instances` must not be empty."
    )]
    fn panics_if_owned_factors_is_empty() {
        Sut::new(
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
        Sut::new(
            CompiledTransactionIntent::sample_other(),
            FactorSourceIDFromHash::sample_other(),
            IndexSet::just(OwnedFactorInstance::sample_other()),
        );
    }
}
