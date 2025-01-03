use crate::prelude::*;
use paste::paste;

macro_rules! decl_factor_outcome {
    (
        struct_name: $struct_name:ident,
        signature: $signature:ident,
    ) => {
        /// The outcome of the signing process for each factor source as collected by the `SignInteractor`.
        #[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
        pub enum $struct_name {
            /// The user successfully signed with the factor source, the associated
            /// value contains the produced signatures and any relevant metadata.
            Signed {
                produced_signatures: Vec<$signature>,
            },

            /// The factor source got neglected, either due to user explicitly skipping
            /// or due to failure
            Neglected(NeglectedFactor),
        }
    };
    ($signable_id:ty) => {
        paste! {
            use sargon::[< $signable_id >] as [< Internal $signable_id >];

            type [< InternalFactorOutcomeOf $signable_id >] =
                sargon::FactorOutcome<[< Internal $signable_id >]>;

            decl_factor_outcome!(
                struct_name: [< FactorOutcomeOf $signable_id >],
                signature: [< HDSignatureOf $signable_id >],
            );
        }
    };
}

decl_factor_outcome!(TransactionIntentHash);
decl_factor_outcome!(SubintentHash);
decl_factor_outcome!(AuthIntentHash);
