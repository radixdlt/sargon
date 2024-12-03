use crate::prelude::*;
use paste::paste;

macro_rules! decl_hd_signature {
    ($type:ty) => {
        paste! {
            use sargon::[< $type >] as [< Internal $type >];

            type [< InternalHDSignatureOf $type >] = sargon::HDSignature<[< Internal $type >]>;

            #[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
            pub struct [< HDSignatureOf $type >] {
                /// The input used to produce this `HDSignature`
                pub input: [< HDSignatureInputOf $type >],

                /// The ECDSA/EdDSA signature produced by the private key of the
                /// `owned_hd_factor_instance.public_key`,
                /// derived by the HDFactorSource identified by
                /// `owned_hd_factor_
                /// instance.factor_s
                /// ource_id` and which
                /// was derived at `owned_hd_factor_instance.derivation_path`.
                pub signature: SignatureWithPublicKey,
            }
        }
    };
}

decl_hd_signature!(TransactionIntentHash);
decl_hd_signature!(SubintentHash);