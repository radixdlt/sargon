use crate::prelude::*;
use paste::paste;

macro_rules! decl_per_factor_outcome {
    (
        struct_name: $struct_name:ident,
        factor_outcome: $factor_outcome:ident
    ) => {
        #[derive(Clone, PartialEq, Eq, uniffi::Record)]
        pub struct $struct_name {
            pub factor_source_id: FactorSourceIDFromHash,
            pub outcome: $factor_outcome,
        }

        impl $struct_name {
            pub fn new(
                factor_source_id: FactorSourceIDFromHash,
                outcome: $factor_outcome,
            ) -> Self {
                Self {
                    factor_source_id,
                    outcome,
                }
            }
        }
    };
    ($signable_id:ty) => {
        paste! {
            decl_per_factor_outcome!(
                struct_name: [< PerFactorOutcomeOf $signable_id >],
                factor_outcome: [< FactorOutcomeOf $signable_id >]
            );
        }
    };
}

decl_per_factor_outcome!(TransactionIntentHash);
decl_per_factor_outcome!(SubintentHash);
decl_per_factor_outcome!(AuthIntentHash);
