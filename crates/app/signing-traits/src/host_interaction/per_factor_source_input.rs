use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerFactorSourceInput<S: Signable> {
    /// The factor source which the interactor should request signatures with
    pub factor_source_id: FactorSourceIDFromHash,

    /// A set of transactions to sign, with multiple derivations paths.
    pub per_transaction: IndexSet<TransactionSignRequestInput<S>>,

    /// A collection of transactions which would be invalid if the user skips
    /// signing with this factor source.
    pub invalid_transactions_if_neglected:
        IndexSet<InvalidTransactionIfNeglected<S::ID>>,
}

impl<S: Signable> PerFactorSourceInput<S> {
    pub fn new(
        factor_source_id: FactorSourceIDFromHash,
        per_transaction: IndexSet<TransactionSignRequestInput<S>>,
        invalid_transactions_if_neglected: IndexSet<
            InvalidTransactionIfNeglected<S::ID>,
        >,
    ) -> Self {
        Self {
            factor_source_id,
            per_transaction,
            invalid_transactions_if_neglected,
        }
    }
}

impl<S: Signable + HasSampleValues> HasSampleValues for PerFactorSourceInput<S>
where
    S::Payload: HasSampleValues,
    S::ID: HasSampleValues,
{
    fn sample() -> Self {
        Self::new(
            FactorSourceIDFromHash::sample(),
            IndexSet::from_iter(vec![
                TransactionSignRequestInput::<S>::sample(),
                TransactionSignRequestInput::<S>::sample_other(),
            ]),
            IndexSet::new(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            FactorSourceIDFromHash::sample_ledger(),
            IndexSet::from_iter(vec![
                TransactionSignRequestInput::<S>::sample(),
            ]),
            IndexSet::just(
                InvalidTransactionIfNeglected::<S::ID>::sample_other(),
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn per_factor_source_input_sample() {
        let sample = PerFactorSourceInput::<TransactionIntent>::sample();
        let sample_other =
            PerFactorSourceInput::<TransactionIntent>::sample_other();
        assert_eq!(sample, sample);
        assert_eq!(sample_other, sample_other);
        assert_ne!(sample, sample_other);
    }
}
