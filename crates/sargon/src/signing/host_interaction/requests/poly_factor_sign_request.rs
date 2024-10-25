use crate::prelude::*;

/// A collection of **many** factor sources to use to sign, transactions with multiple keys
/// (derivations paths).
#[derive(derive_more::Debug, Clone)]
#[debug("per_factor_source: {:#?}", per_factor_source)]
pub struct PolyFactorSignRequest<S: Signable> {
    factor_source_kind: FactorSourceKind,

    /// Per factor source, a set of transactions to sign, with
    /// multiple derivations paths.
    pub per_factor_source:
        IndexMap<FactorSourceIDFromHash, MonoFactorSignRequestInput<S>>,

    /// A collection of transactions which would be invalid if the user skips
    /// signing with this factor source.
    pub invalid_transactions_if_neglected:
        IndexSet<InvalidTransactionIfNeglected<S>>,
}

impl<S: Signable> PolyFactorSignRequest<S> {
    /// # Panics
    /// Panics if `per_factor_source` is empty
    ///
    /// Panics if not all factor sources are of the same kind
    pub(crate) fn new(
        factor_source_kind: FactorSourceKind,
        per_factor_source: IndexMap<
            FactorSourceIDFromHash,
            MonoFactorSignRequestInput<S>,
        >,
        invalid_transactions_if_neglected: IndexSet<
            InvalidTransactionIfNeglected<S>,
        >,
    ) -> Self {
        assert!(
            !per_factor_source.is_empty(),
            "Invalid input, per_factor_source must not be empty, this is a programmer error."
        );
        assert!(
            per_factor_source
                .values()
                .all(|f| f.factor_source_id.kind == factor_source_kind),
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

#[cfg(test)]
mod tests {
    use super::*;
    type Sut = PolyFactorSignRequest<TransactionIntent>;

    #[test]
    #[should_panic(
        expected = "Invalid input, per_factor_source must not be empty, this is a programmer error."
    )]
    fn panics_if_per_factor_source_is_empty() {
        Sut::new(FactorSourceKind::Device, IndexMap::new(), IndexSet::new());
    }

    #[test]
    #[should_panic(
        expected = "Discrepancy! All factor sources must be of the same kind, this is a programmer error."
    )]
    fn panics_if_wrong_factor_source_kind() {
        Sut::new(
            FactorSourceKind::ArculusCard,
            IndexMap::just((
                FactorSourceIDFromHash::sample(),
                MonoFactorSignRequestInput::sample(),
            )),
            IndexSet::new(),
        );
    }
}
