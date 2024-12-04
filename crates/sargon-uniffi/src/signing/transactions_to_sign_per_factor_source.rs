use crate::prelude::*;
use paste::paste;

#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct TransactionToSignPerFactorSource {
    pub factor_source_id: FactorSourceIDFromHash,

    pub transactions: Vec<TransactionSignRequestInputOfSubintent>,
}

impl TransactionToSignPerFactorSource {
    pub fn new(
        factor_source_id: FactorSourceIDFromHash,
        transactions: Vec<TransactionSignRequestInputOfSubintent>,
    ) -> Self {
        Self {
            factor_source_id,
            transactions,
        }
    }
}

macro_rules! decl_transaction_to_sign_per_factor_source {
    (
        struct_name: $struct_name:ident,
        per_transaction: $per_transaction:ident,
    ) => {
        #[derive(Clone, PartialEq, Eq, uniffi::Record)]
        pub struct $struct_name {
            pub factor_source_id: FactorSourceIDFromHash,

            pub transactions: Vec<$per_transaction>,
        }

        impl $struct_name {
            pub fn new(
                factor_source_id: FactorSourceIDFromHash,
                transactions: Vec<$per_transaction>,
            ) -> Self {
                Self {
                    factor_source_id,
                    transactions,
                }
            }
        }
    };
    (signable: $signable:ty, signable_id: $signable_id:ty) => {
        paste! {
            use sargon::[< $signable >] as [< Internal $signable >];
            use sargon::[< $signable_id >] as [< Internal $signable_id >];

            decl_transaction_to_sign_per_factor_source!(
                struct_name: [< TransactionToSignPerFactorSourceOf $signable >],
                per_transaction: [< TransactionSignRequestInputOf $signable >],
            );
        }
    };
}

decl_transaction_to_sign_per_factor_source!(
    signable: TransactionIntent,
    signable_id: TransactionIntentHash
);
decl_transaction_to_sign_per_factor_source!(
    signable: Subintent,
    signable_id: SubintentHash
);
