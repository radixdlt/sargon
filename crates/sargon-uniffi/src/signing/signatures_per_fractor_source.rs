use crate::prelude::*;
use paste::paste;

macro_rules! decl_signatures_per_factor_source {
    ($signable_id:ty) => {
        paste! {
            use sargon::[< $signable_id >] as [< Internal $signable_id >];

            #[derive(Clone, PartialEq, Eq, uniffi::Record)]
            pub struct [< SignaturesPerFactorSourceOf $signable_id >] {
                pub factor_source_id: FactorSourceIDFromHash,

                pub hd_signatures: Vec<[< HDSignatureOf $signable_id >]>
            }

            impl [< SignaturesPerFactorSourceOf $signable_id >] {

                pub fn new(
                    factor_source_id: FactorSourceIDFromHash,
                    hd_signatures: Vec<[< HDSignatureOf $signable_id >]>
                ) -> Self {
                    Self {
                        factor_source_id,
                        hd_signatures
                    }
                }

            }
        }
    };
}

decl_signatures_per_factor_source!(TransactionIntentHash);
decl_signatures_per_factor_source!(SubintentHash);