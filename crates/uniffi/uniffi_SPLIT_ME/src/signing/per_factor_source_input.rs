use crate::prelude::*;
use paste::paste;

macro_rules! decl_per_factor_source_input {
    (
        struct_name: $struct_name:ident,
        per_transaction: $per_transaction:ident,
        invalid_transaction_if_neglected: $invalid_transaction_if_neglected:ident,
    ) => {
        #[derive(Clone, PartialEq, Eq, uniffi::Record)]
        pub struct $struct_name {
            /// The factor source which the interactor should request signatures with
            pub factor_source_id: FactorSourceIDFromHash,

            /// A set of transactions to sign, with multiple derivations paths.
            pub per_transaction: Vec<$per_transaction>,

            /// A collection of transactions which would be invalid if the user skips
            /// signing with this factor source.
            pub invalid_transactions_if_neglected: Vec<$invalid_transaction_if_neglected>
        }

        impl $struct_name {
            pub fn new(
                factor_source_id: FactorSourceIDFromHash,
                per_transaction: Vec<$per_transaction>,
                invalid_transactions_if_neglected: Vec<$invalid_transaction_if_neglected>
            ) -> Self {
                Self {
                    factor_source_id,
                    per_transaction,
                    invalid_transactions_if_neglected
                }
            }
        }
    };
    (signable: $signable:ty, signable_id: $signable_id:ty) => {
        paste! {
            use sargon::[< $signable >] as [< Internal $signable >];
            use sargon::[< $signable_id >] as [< Internal $signable_id >];

            decl_per_factor_source_input!(
                struct_name: [< PerFactorSourceInputOf $signable >],
                per_transaction: [< TransactionSignRequestInputOf $signable >],
                invalid_transaction_if_neglected: [< InvalidTransactionIfNeglectedOf $signable_id >],
            );
        }
    };
}

decl_per_factor_source_input!(
    signable: TransactionIntent,
    signable_id: TransactionIntentHash
);
decl_per_factor_source_input!(
    signable: Subintent,
    signable_id: SubintentHash
);
decl_per_factor_source_input!(
    signable: AuthIntent,
    signable_id: AuthIntentHash
);
