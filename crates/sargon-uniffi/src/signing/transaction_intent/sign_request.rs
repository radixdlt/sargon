use crate::prelude::*;
use sargon::SignRequest as InternalSignRequest;
use sargon::TransactionIntent as InternalTransactionIntent;

type InternalSignRequestForTransactionIntent =
    InternalSignRequest<InternalTransactionIntent>;

#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct SignRequestForTransactionIntent {
    pub factor_source_kind: FactorSourceKind,

    /// Per factor source, a set of transactions to sign, with
    /// multiple derivations paths.
    pub per_factor_source: Vec<TransactionToSignPerFactorSourceOfTransactionIntent>,

    /// A collection of transactions which would be invalid if the user skips
    /// signing with this factor source.
    pub invalid_transactions_if_neglected:
        Vec<InvalidTransactionIfNeglectedOfTransactionIntentHash>,
}

impl SignRequestForTransactionIntent {
    pub fn into_internal(&self) -> InternalSignRequestForTransactionIntent {
        self.clone().into()
    }
}

impl From<InternalSignRequestForTransactionIntent>
    for SignRequestForTransactionIntent
{
    fn from(value: InternalSignRequestForTransactionIntent) -> Self {
        Self {
            factor_source_kind: value.factor_source_kind.into(),
            per_factor_source: value
                .per_factor_source
                .into_iter()
                .map(|(id, transactions)| {
                    TransactionToSignPerFactorSourceOfTransactionIntent::new(
                        id.into(),
                        transactions.into_iter().map(|t| t.into()).collect()
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
    for InternalSignRequestForTransactionIntent
{
    fn from(value: SignRequestForTransactionIntent) -> Self {
        Self {
            factor_source_kind: value.factor_source_kind.into_internal(),
            per_factor_source: value
                .per_factor_source
                .iter()
                .map(|item| {
                    (
                        item.factor_source_id.into_internal(),
                        item.transactions
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

decl_conversion_tests_for!(SignRequestForTransactionIntent);
