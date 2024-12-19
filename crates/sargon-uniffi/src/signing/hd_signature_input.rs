use crate::prelude::*;
use paste::paste;

macro_rules! decl_hd_signature_input {
    (
        struct_name: $struct_name:ident,
        payload: $payload_id:ident
    ) => {
        #[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
        pub struct $struct_name {
            /// Hash which was signed.
            pub payload_id: $payload_id,

            /// The account or identity address of the entity which signed the hash,
            /// with expected public key and with derivation path to derive PrivateKey
            /// with.
            pub owned_factor_instance: OwnedFactorInstance,
        }
    };
    ($type:ty) => {
        paste! {
            use sargon::[< $type >] as [< Internal $type >];

            type [< InternalHDSignatureInputOf $type >] = sargon::HDSignatureInput<[< Internal $type >]>;

            decl_hd_signature_input!(
                struct_name: [< HDSignatureInputOf $type >],
                payload: [< $type >]
            );
        }
    };
}

decl_hd_signature_input!(TransactionIntentHash);
decl_hd_signature_input!(SubintentHash);
decl_hd_signature_input!(AuthIntentHash);
