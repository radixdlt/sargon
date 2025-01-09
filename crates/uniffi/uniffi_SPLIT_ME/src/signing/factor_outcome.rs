use crate::prelude::*;
use paste::paste;

macro_rules! decl_factor_outcome {
    (
        struct_name: $struct_name:ident,
        internal_struct_name: $internal_struct_name:ident,
        signature: $signature:ident,
        new_signed: $new_signed:ident,
        new_failure: $new_failure:ident,
        new_skipped: $new_skipped:ident,
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

        #[uniffi::export]
        pub fn $new_signed(produced_signatures: Vec<$signature>) -> Result<$struct_name> {
            $internal_struct_name::signed(
                sargon::IndexSet::from_iter(
                    produced_signatures.into_iter().map(|sig| sig.into_internal())
                )
            ).into_result()
        }

        #[uniffi::export]
        pub fn $new_failure(factor_source_id: FactorSourceIDFromHash) -> $struct_name {
            $internal_struct_name::failure(
                factor_source_id.into_internal()
            ).into()
        }

        #[uniffi::export]
        pub fn $new_skipped(factor_source_id: FactorSourceIDFromHash) -> $struct_name {
            $internal_struct_name::skipped(
                factor_source_id.into_internal()
            ).into()
        }
    };
    ($signable_id:ty) => {
        paste! {
            use sargon::[< $signable_id >] as [< Internal $signable_id >];

            type [< InternalFactorOutcomeOf $signable_id >] =
                sargon::FactorOutcome<[< Internal $signable_id >]>;

            decl_factor_outcome!(
                struct_name: [< FactorOutcomeOf $signable_id >],
                internal_struct_name: [< InternalFactorOutcomeOf $signable_id >],
                signature: [< HDSignatureOf $signable_id >],
                new_signed: [< new_signed_factor_outcome_of_ $signable_id:snake >],
                new_failure: [< new_failure_factor_outcome_of_ $signable_id:snake >],
                new_skipped: [< new_skipped_factor_outcome_of_ $signable_id:snake >],
            );
        }
    };
}

decl_factor_outcome!(TransactionIntentHash);
decl_factor_outcome!(SubintentHash);
decl_factor_outcome!(AuthIntentHash);
