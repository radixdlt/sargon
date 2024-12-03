use crate::prelude::*;
use paste::paste;

macro_rules! decl_hd_signature_input {
    ($type:ty) => {
        paste! {
            use sargon::[< $type >] as [< Internal $type >];

            type [< InternalHDSignatureInputOf $type >] = sargon::HDSignatureInput<[< Internal $type >]>;

            #[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
            pub struct [< HDSignatureInputOf $type >] {
                /// Hash which was signed.
                pub payload_id: [< $type >],

                /// The account or identity address of the entity which signed the hash,
                /// with expected public key and with derivation path to derive PrivateKey
                /// with.
                pub owned_factor_instance: OwnedFactorInstance,
            }
        }
    };
}

decl_hd_signature_input!(TransactionIntentHash);
decl_hd_signature_input!(SubintentHash);
