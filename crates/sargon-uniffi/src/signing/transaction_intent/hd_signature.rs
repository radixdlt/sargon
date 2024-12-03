use crate::prelude::*;
use sargon::HDSignature as InternalHDSignature;
use sargon::TransactionIntentHash as InternalTransactionIntentHash;

type InternalHdSignatureForTransactionIntent =
    InternalHDSignature<InternalTransactionIntentHash>;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct HdSignatureForTransactionIntent {
    /// The input used to produce this `HdSignatureForTransactionIntent`
    pub input: HDSignatureInputForTransactionIntent,

    /// The ECDSA/EdDSA signature produced by the private key of the
    /// `owned_hd_factor_instance.public_key`,
    /// derived by the HDFactorSource identified by
    /// `owned_hd_factor_
    /// instance.factor_s
    /// ource_id` and which
    /// was derived at `owned_hd_factor_instance.derivation_path`.
    pub signature: SignatureWithPublicKey,
}
