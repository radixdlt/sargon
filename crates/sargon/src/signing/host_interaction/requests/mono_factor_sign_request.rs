use crate::prelude::*;

/// A request to sign a batch of transactions with a single factor source.
#[derive(derive_more::Debug, Clone)]
#[debug("input: {:#?}", input)]
pub struct MonoFactorSignRequest {
    /// The input needed to sign the transactions.
    pub input: MonoFactorSignRequestInput,

    /// A collection of transactions which would be invalid if the user skips
    /// signing with this factor source, or if we fail to sign
    pub invalid_transactions_if_neglected:
        IndexSet<InvalidTransactionIfNeglected>,
}

impl MonoFactorSignRequest {
    pub fn new(
        input: MonoFactorSignRequestInput,
        invalid_transactions_if_neglected: IndexSet<
            InvalidTransactionIfNeglected,
        >,
    ) -> Self {
        Self {
            input,
            invalid_transactions_if_neglected,
        }
    }

    #[allow(unused)]
    pub(crate) fn factor_source_kind(&self) -> FactorSourceKind {
        self.input.factor_source_kind()
    }
}
