use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignRequest<S: Signable> {
    pub factor_source_kind: FactorSourceKind,

    /// Per factor source, a set of transactions to sign, with
    /// multiple derivations paths.
    pub per_factor_source: IndexMap<
        FactorSourceIDFromHash,
        IndexSet<TransactionSignRequestInput<S>>,
    >,

    /// A collection of transactions which would be invalid if the user skips
    /// signing with this factor source.
    pub invalid_transactions_if_neglected:
        IndexSet<InvalidTransactionIfNeglected<S::ID>>,
}

impl<S: Signable> SignRequest<S> {
    /// # Panics
    /// Panics if `per_factor_source` is empty
    ///
    /// Panics if not all factor sources are of the same kind
    pub(crate) fn new(
        factor_source_kind: FactorSourceKind,
        per_factor_source: IndexMap<
            FactorSourceIDFromHash,
            IndexSet<TransactionSignRequestInput<S>>,
        >,
        invalid_transactions_if_neglected: IndexSet<
            InvalidTransactionIfNeglected<S::ID>,
        >,
    ) -> Self {
        assert!(
            !per_factor_source.is_empty(),
            "Invalid input, per_factor_source must not be empty, this is a programmer error."
        );

        assert!(
            per_factor_source
                .keys()
                .all(|f| f.kind == factor_source_kind),
            "Discrepancy! All factor sources must be of the same kind, this is a programmer error."
        );

        Self {
            factor_source_kind,
            per_factor_source,
            invalid_transactions_if_neglected,
        }
    }

    pub fn factor_source_ids(&self) -> IndexSet<FactorSourceIDFromHash> {
        self.per_factor_source.keys().cloned().collect()
    }

    #[allow(unused)]
    pub(crate) fn factor_source_kind(&self) -> FactorSourceKind {
        self.factor_source_kind
    }
}

impl<S: Signable> HasSampleValues for SignRequest<S> {
    fn sample() -> Self {
        Self::new(
            FactorSourceKind::sample(),
            IndexMap::just((
                FactorSourceIDFromHash::sample(),
                IndexSet::from_iter(vec![
                    TransactionSignRequestInput::sample(),
                    TransactionSignRequestInput::sample_other(),
                ]),
            )),
            IndexSet::new(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            FactorSourceKind::sample_other(),
            IndexMap::just((
                FactorSourceIDFromHash::sample_ledger(),
                IndexSet::from_iter(vec![
                    TransactionSignRequestInput::sample(),
                    TransactionSignRequestInput::sample_other(),
                ]),
            )),
            IndexSet::just(
                InvalidTransactionIfNeglected::<S::ID>::sample_other(),
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(clippy::upper_case_acronyms)]
    type SUT = SignRequest<TransactionIntent>;

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
        expected = "Invalid input, per_factor_source must not be empty, this is a programmer error."
    )]
    fn panics_if_per_factor_source_is_empty() {
        SUT::new(FactorSourceKind::Device, IndexMap::new(), IndexSet::new());
    }

    #[test]
    #[should_panic(
        expected = "Discrepancy! All factor sources must be of the same kind, this is a programmer error."
    )]
    fn panics_if_wrong_factor_source_kind() {
        SUT::new(
            FactorSourceKind::ArculusCard,
            IndexMap::just((
                FactorSourceIDFromHash::sample(),
                IndexSet::just(
                    TransactionSignRequestInput::<TransactionIntent>::sample(),
                ),
            )),
            IndexSet::new(),
        );
    }
}
