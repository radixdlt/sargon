use crate::prelude::*;
use paste::paste;

macro_rules! decl_sign_request {
    (
        struct_name: $struct_name:ident,
        internal_struct_name: $internal_struct_name:ident,
        per_factor_source: $per_factor_source:ident,
        invalid_transactions_if_neglected: $invalid_transactions_if_neglected:ident,
    ) => {
        #[derive(Clone, PartialEq, Eq, uniffi::Record)]
        pub struct $struct_name {
            pub factor_source_kind: FactorSourceKind,

            /// Per factor source, a set of transactions to sign, with
            /// multiple derivations paths.
            pub per_factor_source: Vec<$per_factor_source>,

            /// A collection of transactions which would be invalid if the user skips
            /// signing with this factor source.
            pub invalid_transactions_if_neglected:
                Vec<$invalid_transactions_if_neglected>,
        }

        impl $struct_name {
            pub fn into_internal(&self) -> $internal_struct_name {
                self.clone().into()
            }
        }

        impl From<$internal_struct_name> for $struct_name {
            fn from(value: $internal_struct_name) -> Self {
                Self {
                    factor_source_kind: value.factor_source_kind.into(),
                    per_factor_source: value
                        .per_factor_source
                        .into_iter()
                        .map(|(id, transactions)| {
                            $per_factor_source::new(
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

        impl From<$struct_name> for $internal_struct_name
        {
            fn from(value: $struct_name) -> Self {
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

        decl_conversion_tests_for!($struct_name);
    };
    (signable: $signable:ty, signable_id: $signable_id:ty) => {
        paste! {
            use sargon::[< $signable >] as [< Internal $signable >];
            use sargon::[< $signable_id >] as [< Internal $signable_id >];

            type [< InternalSignRequestOf $signable >] =
                sargon::SignRequest<[< Internal $signable >]>;

            decl_sign_request!(
                struct_name: [< SignRequestOf $signable >],
                internal_struct_name: [< InternalSignRequestOf $signable >],
                per_factor_source: [< TransactionToSignPerFactorSourceOf $signable >],
                invalid_transactions_if_neglected: [< InvalidTransactionIfNeglectedOf $signable_id >],
            );
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
