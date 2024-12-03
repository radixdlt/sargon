use crate::prelude::*;
use sargon::SubintentHash as InternalSubintentHash;

type InternalHdSignatureForSubintent =
    sargon::HDSignature<InternalSubintentHash>;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct HdSignatureForSubintent {
    /// The input used to produce this `HdSignatureForSubintent`
    pub input: HDSignatureInputForSubintent,

    /// The ECDSA/EdDSA signature produced by the private key of the
    /// `owned_hd_factor_instance.public_key`,
    /// derived by the HDFactorSource identified by
    /// `owned_hd_factor_
    /// instance.factor_s
    /// ource_id` and which
    /// was derived at `owned_hd_factor_instance.derivation_path`.
    pub signature: SignatureWithPublicKey,
}
