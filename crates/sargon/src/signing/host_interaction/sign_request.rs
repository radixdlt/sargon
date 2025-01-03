use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignRequest<S: Signable> {
    pub factor_source_kind: FactorSourceKind,

    /// Per factor source, a set of inputs that contain information on what signables need signing,
    /// and what signables will fail if such factor source is neglected.
    pub per_factor_source:
        IndexMap<FactorSourceIDFromHash, PerFactorSourceInput<S>>,
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
            PerFactorSourceInput<S>,
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
        }
    }

    pub fn factor_source_ids(&self) -> IndexSet<FactorSourceIDFromHash> {
        self.per_factor_source.keys().cloned().collect()
    }

    pub fn invalid_transactions_if_all_factors_neglected(
        &self,
    ) -> IndexSet<InvalidTransactionIfNeglected<S::ID>> {
        let mut invalid_transactions_for_all_factors =
            IndexSet::<InvalidTransactionIfNeglected<S::ID>>::new();

        self.per_factor_source.values().for_each(|input| {
            invalid_transactions_for_all_factors
                .extend(input.invalid_transactions_if_neglected.clone())
        });

        invalid_transactions_for_all_factors
    }

    pub fn invalid_transactions_if_factor_neglected(
        &self,
        factor_source_id: &FactorSourceIDFromHash,
    ) -> IndexSet<InvalidTransactionIfNeglected<S::ID>> {
        self.per_factor_source
            .get(factor_source_id)
            .map_or(IndexSet::new(), |i| {
                i.invalid_transactions_if_neglected.clone()
            })
    }

    #[allow(unused)]
    pub(crate) fn factor_source_kind(&self) -> FactorSourceKind {
        self.factor_source_kind
    }
}

impl<S: Signable + HasSampleValues> HasSampleValues for SignRequest<S>
where
    S::Payload: HasSampleValues,
    S::ID: HasSampleValues,
{
    fn sample() -> Self {
        Self::new(
            FactorSourceKind::sample(),
            IndexMap::just((
                FactorSourceIDFromHash::sample(),
                PerFactorSourceInput::sample(),
            )),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            FactorSourceKind::sample_other(),
            IndexMap::just((
                FactorSourceIDFromHash::sample_ledger(),
                PerFactorSourceInput::sample_other(),
            )),
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
        SUT::new(FactorSourceKind::Device, IndexMap::new());
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
                PerFactorSourceInput::<TransactionIntent>::sample(),
            )),
        );
    }
}
