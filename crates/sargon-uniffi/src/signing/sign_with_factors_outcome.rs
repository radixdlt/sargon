use crate::prelude::*;
use paste::paste;

macro_rules! decl_sign_with_factors_outcome {
    ($signable_id:ty) => {
        paste! {
            use sargon::[< $signable_id >] as [< Internal $signable_id >];

            type [< InternalSignWithFactorsOutcomeOf $signable_id >] =
                sargon::SignWithFactorsOutcome<[< Internal $signable_id >]>;

            /// A batch of keys (derivation paths) all being factor instances of a HDFactorSource
            /// with id `factor_source_id` to sign a single transaction with, which hash
            /// is `intent_hash`.
            #[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
            pub enum [< SignWithFactorsOutcomeOf $signable_id >] {
                /// The user successfully signed with the factor source(s), the associated
                /// value contains the produces signatures and any relevant metadata.
                Signed {
                    produced_signatures: [< SignResponseOf $signable_id >],
                },

                /// The factor source got neglected, either due to user explicitly skipping
                /// or due to failure
                Neglected(NeglectedFactors),
            }
        }
    };
}

decl_sign_with_factors_outcome!(TransactionIntentHash);
decl_sign_with_factors_outcome!(SubintentHash);
