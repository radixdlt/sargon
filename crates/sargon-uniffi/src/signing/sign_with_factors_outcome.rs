use crate::prelude::*;
use paste::paste;

macro_rules! decl_sign_with_factors_outcome {
    (
        struct_name: $struct_name:ident,
        sign_response: $sign_response:ident,
    ) => {
        /// A batch of keys (derivation paths) all being factor instances of a HDFactorSource
        /// with id `factor_source_id` to sign a single transaction with, which hash
        /// is `intent_hash`.
        #[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
        pub enum $struct_name {
            /// The user successfully signed with the factor source(s), the associated
            /// value contains the produces signatures and any relevant metadata.
            Signed { produced_signatures: $sign_response },

            /// The factor source got neglected, either due to user explicitly skipping
            /// or due to failure
            Neglected(NeglectedFactors),
        }
    };
    ($signable_id:ty) => {
        paste! {
            use sargon::[< $signable_id >] as [< Internal $signable_id >];

            type [< InternalSignWithFactorsOutcomeOf $signable_id >] =
                sargon::SignWithFactorsOutcome<[< Internal $signable_id >]>;

            decl_sign_with_factors_outcome!(
                struct_name: [< SignWithFactorsOutcomeOf $signable_id >],
                sign_response: [< SignResponseOf $signable_id >],
            );
        }
    };
}

decl_sign_with_factors_outcome!(TransactionIntentHash);
decl_sign_with_factors_outcome!(SubintentHash);
decl_sign_with_factors_outcome!(AuthIntentHash);
