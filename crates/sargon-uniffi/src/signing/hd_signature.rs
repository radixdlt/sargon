use crate::prelude::*;
use paste::paste;

macro_rules! decl_hd_signature {
    (
        struct_name: $struct_name:ident,
        inp: $input:ident
    ) => {
        #[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
        pub struct $struct_name {
            /// The input used to produce this `HDSignature`
            pub input: $input,

            /// The ECDSA/EdDSA signature produced by the private key of the
            /// `owned_hd_factor_instance.public_key`,
            /// derived by the HDFactorSource identified by
            /// `owned_hd_factor_
            /// instance.factor_s
            /// ource_id` and which
            /// was derived at `owned_hd_factor_instance.derivation_path`.
            pub signature: SignatureWithPublicKey,
        }
    };
    ($type:ty) => {
        paste! {
            use sargon::[< $type >] as [< Internal $type >];

            type [< InternalHDSignatureOf $type >] = sargon::HDSignature<[< Internal $type >]>;

            decl_hd_signature!(
                struct_name: [< HDSignatureOf $type >],
                inp: [< HDSignatureInputOf $type >]
            );
        }
    };
}

decl_hd_signature!(TransactionIntentHash);
decl_hd_signature!(SubintentHash);
