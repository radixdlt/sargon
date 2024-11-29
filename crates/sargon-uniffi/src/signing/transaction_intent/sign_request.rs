use crate::prelude::*;
use sargon::SignRequest as InternalSignRequest;
use sargon::TransactionIntent as InternalTransactionIntent;

#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct SignRequestForTransactionIntent {
    pub factor_source_kind: FactorSourceKind,

    /// Per factor source, a set of transactions to sign, with
    /// multiple derivations paths.
    pub per_factor_source: HashMap<
        FactorSourceIDFromHash,
        Vec<TransactionSignRequestInputForTransactionIntent>,
    >,

    /// A collection of transactions which would be invalid if the user skips
    /// signing with this factor source.
    pub invalid_transactions_if_neglected:
        Vec<InvalidTransactionIfNeglectedForTransactionIntent>,
}

impl SignRequestForTransactionIntent {
    pub fn into_internal(
        &self,
    ) -> InternalSignRequest<InternalTransactionIntent> {
        self.clone().into()
    }
}

impl From<InternalSignRequest<InternalTransactionIntent>>
    for SignRequestForTransactionIntent
{
    fn from(value: InternalSignRequest<InternalTransactionIntent>) -> Self {
        Self {
            factor_source_kind: value.factor_source_kind.into(),
            per_factor_source: value
                .per_factor_source
                .into_iter()
                .map(|(id, transactions)| {
                    (
                        id.into(),
                        transactions.into_iter().map(|t| t.into()).collect(),
                    )
                })
                .collect(),
            invalid_transactions_if_neglected: value
                .invalid_transactions_if_neglected
                .into_iter()
                .map(|t| t.into())
                .collect(),
        }
    }
}

impl From<SignRequestForTransactionIntent>
    for InternalSignRequest<InternalTransactionIntent>
{
    fn from(value: SignRequestForTransactionIntent) -> Self {
        Self {
            factor_source_kind: value.factor_source_kind.into_internal(),
            per_factor_source: value
                .per_factor_source
                .iter()
                .map(|(id, transactions)| {
                    (
                        id.into_internal(),
                        transactions
                            .iter()
                            .map(|t| t.into_internal())
                            .collect(),
                    )
                })
                .collect(),
            invalid_transactions_if_neglected: value
                .invalid_transactions_if_neglected
                .iter()
                .map(|t| t.into_internal())
                .collect(),
        }
    }
}