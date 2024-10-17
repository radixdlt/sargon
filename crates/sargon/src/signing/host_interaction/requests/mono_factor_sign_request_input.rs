use crate::prelude::*;

/// A batch of transactions each batching over multiple keys (derivation paths)
/// to sign each transaction with.
#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct MonoFactorSignRequestInput {
    /// The ID of the factor source used to sign each per_transaction
    pub factor_source_id: FactorSourceIDFromHash,

    // The `factor_source_id` of each item must match `self.factor_source_id`.
    pub per_transaction: Vec<TransactionSignRequestInput>,
}

impl MonoFactorSignRequestInput {
    /// # Panics
    /// Panics if `per_transaction` is empty
    ///
    /// Also panics if `per_transaction` if the factor source id
    /// of each request does not match `factor_source_id`.
    pub(crate) fn new(
        factor_source_id: FactorSourceIDFromHash,
        per_transaction: IndexSet<TransactionSignRequestInput>,
    ) -> Self {
        assert!(
            !per_transaction.is_empty(),
            "Invalid input. No transaction to sign, this is a programmer error."
        );

        assert!(per_transaction
                    .iter()
                    .all(|f| f.factor_source_id == factor_source_id), "Discprepancy! Input for one of the transactions has a mismatching FactorSourceID, this is a programmer error.");

        Self {
            factor_source_id,
            per_transaction: per_transaction.into_iter().collect(),
        }
    }

    /// Returns the factor source kind of the factor source id.
    #[allow(unused)]
    pub(crate) fn factor_source_kind(&self) -> FactorSourceKind {
        self.factor_source_id.kind
    }
}

impl HasSampleValues for MonoFactorSignRequestInput {
    /// Creates a new MonoFactorSignRequestInput with sample values.
    fn sample() -> Self {
        let input = TransactionSignRequestInput::sample();
        Self::new(input.clone().factor_source_id, IndexSet::just(input))
    }

    /// Creates a new MonoFactorSignRequestInput with sample values.
    fn sample_other() -> Self {
        let input = TransactionSignRequestInput::sample_other();
        Self::new(input.clone().factor_source_id, IndexSet::just(input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Sut = MonoFactorSignRequestInput;

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
        expected = "Invalid input. No transaction to sign, this is a programmer error."
    )]
    fn panics_if_per_transaction_is_empty() {
        Sut::new(FactorSourceIDFromHash::sample(), IndexSet::new());
    }

    #[test]
    #[should_panic(
        expected = "Discprepancy! Input for one of the transactions has a mismatching FactorSourceID, this is a programmer error."
    )]
    fn panics_if_factor_source_mismatch() {
        Sut::new(
            FactorSourceIDFromHash::sample_other(),
            IndexSet::just(TransactionSignRequestInput::sample_other()),
        );
    }
}
