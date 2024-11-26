use crate::prelude::*;
use sargon::SubintentHash as InternalSubintentHash;

type InternalHdSignatureForSubintent =
    sargon::HDSignature<InternalSubintentHash>;

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
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
    pub signature: Signature,
}

impl HdSignatureForSubintent {
    pub fn into_internal(&self) -> InternalHdSignatureForSubintent {
        self.clone().into()
    }
}

impl From<InternalHdSignatureForSubintent> for HdSignatureForSubintent {
    fn from(value: InternalHdSignatureForSubintent) -> Self {
        Self {
            input: value.input.into(),
            signature: value.signature.into(),
        }
    }
}

impl From<HdSignatureForSubintent> for InternalHdSignatureForSubintent {
    fn from(value: HdSignatureForSubintent) -> Self {
        Self {
            input: value.input.into_internal(),
            signature: value.signature.into_internal(),
        }
    }
}
