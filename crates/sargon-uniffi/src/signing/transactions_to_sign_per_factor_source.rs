use crate::prelude::*;
use paste::paste;

#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct TransactionToSignPerFactorSource {
    pub factor_source_id: FactorSourceIDFromHash,

    pub transactions: Vec<TransactionSignRequestInputOfSubintent>
}

impl TransactionToSignPerFactorSource {
    pub fn new(
        factor_source_id: FactorSourceIDFromHash,
        transactions: Vec<TransactionSignRequestInputOfSubintent>
    ) -> Self {
        Self {
            factor_source_id,
            transactions
        }
    }
}

macro_rules! decl_transaction_to_sign_per_factor_source {
    (signable: $signable:ty, signable_id: $signable_id:ty) => {
        paste! {
            use sargon::[< $signable >] as [< Internal $signable >];
            use sargon::[< $signable_id >] as [< Internal $signable_id >];

            #[derive(Clone, PartialEq, Eq, uniffi::Record)]
            pub struct [< TransactionToSignPerFactorSourceOf $signable >] {
                pub factor_source_id: FactorSourceIDFromHash,

                pub transactions: Vec<[< TransactionSignRequestInputOf $signable >]>
            }

            impl [< TransactionToSignPerFactorSourceOf $signable >] {
                pub fn new(
                    factor_source_id: FactorSourceIDFromHash,
                    transactions: Vec<[< TransactionSignRequestInputOf $signable >]>
                ) -> Self {
                    Self {
                        factor_source_id,
                        transactions
                    }
                }
            }
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