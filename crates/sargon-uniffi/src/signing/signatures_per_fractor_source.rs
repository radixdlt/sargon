use crate::prelude::*;
use paste::paste;

macro_rules! decl_signatures_per_factor_source {
    (
        struct_name: $struct_name:ident,
        of: $item:ident
    ) => {
        #[derive(Clone, PartialEq, Eq, uniffi::Record)]
        pub struct $struct_name {
            pub factor_source_id: FactorSourceIDFromHash,

            pub hd_signatures: Vec<$item>,
        }

        impl $struct_name {
            pub fn new(
                factor_source_id: FactorSourceIDFromHash,
                hd_signatures: Vec<$item>,
            ) -> Self {
                Self {
                    factor_source_id,
                    hd_signatures,
                }
            }
        }
    };
    ($signable_id:ty) => {
        paste! {
            use sargon::[< $signable_id >] as [< Internal $signable_id >];

            decl_signatures_per_factor_source!(
                struct_name: [< SignaturesPerFactorSourceOf $signable_id >],
                of: [< HDSignatureOf $signable_id >]
            );
        }
    };
}

decl_signatures_per_factor_source!(TransactionIntentHash);
decl_signatures_per_factor_source!(SubintentHash);
decl_signatures_per_factor_source!(AuthIntentHash);
