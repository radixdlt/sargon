use crate::prelude::*;
use paste::paste;

macro_rules! decl_sign_request {
    (signable: $signable:ty, signable_id: $signable_id:ty) => {
        paste! {
            use sargon::[< $signable >] as [< Internal $signable >];
            use sargon::[< $signable_id >] as [< Internal $signable_id >];

            type [< InternalSignRequestOf $signable >] =
                sargon::SignRequest<[< Internal $signable >]>;

            #[derive(Clone, PartialEq, Eq, uniffi::Record)]
            pub struct [< SignRequestOf $signable >] {
                pub factor_source_kind: FactorSourceKind,

                /// Per factor source, a set of transactions to sign, with
                /// multiple derivations paths.
                pub per_factor_source: Vec<[< TransactionToSignPerFactorSourceOf $signable >]>,

                /// A collection of transactions which would be invalid if the user skips
                /// signing with this factor source.
                pub invalid_transactions_if_neglected:
                    Vec<[< InvalidTransactionIfNeglectedOf $signable_id >]>,
            }

            impl [< SignRequestOf $signable >] {
                pub fn into_internal(&self) -> [< InternalSignRequestOf $signable >] {
                    self.clone().into()
                }
            }

            impl From<[< InternalSignRequestOf $signable >]>
                for [< SignRequestOf $signable >]
            {
                fn from(value: [< InternalSignRequestOf $signable >]) -> Self {
                    Self {
                        factor_source_kind: value.factor_source_kind.into(),
                        per_factor_source: value
                            .per_factor_source
                            .into_iter()
                            .map(|(id, transactions)| {
                                [< TransactionToSignPerFactorSourceOf $signable >]::new(
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

            impl From<[< SignRequestOf $signable >]>
                for [< InternalSignRequestOf $signable >]
            {
                fn from(value: [< SignRequestOf $signable >]) -> Self {
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

            decl_conversion_tests_for!([< SignRequestOf $signable >]);
        }
    };
}

decl_sign_request!(
    signable: TransactionIntent,
    signable_id: TransactionIntentHash
);
decl_sign_request!(
    signable: Subintent,
    signable_id: SubintentHash
);
