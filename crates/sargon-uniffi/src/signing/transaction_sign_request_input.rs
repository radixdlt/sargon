use crate::prelude::*;
use paste::paste;

macro_rules! decl_transaction_sign_request_input {
    (signable: $signable:ty, payload: $payload:ty) => {
        paste! {
            use sargon::[< $signable >] as [< Internal $signable >];
            use sargon::[< $payload >] as [< Internal $payload >];

            type [< InternalTransactionSignRequestInputOf $signable >] =
                sargon::TransactionSignRequestInput<[< Internal $signable >]>;

            /// A batch of keys (derivation paths) all being factor instances of a HDFactorSource
            /// with id `factor_source_id` to sign a single transaction with, which hash
            /// is `intent_hash`.
            #[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
            pub struct [< TransactionSignRequestInputOf $signable >] {
                /// Payload to sign
                pub payload: [< $payload >],

                /// ID of factor to use to sign
                pub factor_source_id: FactorSourceIDFromHash,

                /// The derivation paths to use to derive the private keys to sign with. The
                /// `factor_source_id` of each item must match `factor_source_id`.
                pub owned_factor_instances: Vec<OwnedFactorInstance>,
            }
        }
    };
}

decl_transaction_sign_request_input!(
    signable: TransactionIntent,
    payload: CompiledTransactionIntent
);
decl_transaction_sign_request_input!(
    signable: Subintent,
    payload: CompiledSubintent
);
