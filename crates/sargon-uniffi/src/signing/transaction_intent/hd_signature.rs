use crate::prelude::*;
use sargon::HDSignature as InternalHDSignature;
use sargon::TransactionIntentHash as InternalTransactionIntentHash;

type InternalHdSignatureForTransactionIntent =
    InternalHDSignature<InternalTransactionIntentHash>;

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
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

impl HdSignatureForTransactionIntent {
    pub fn into_internal(&self) -> InternalHdSignatureForTransactionIntent {
        self.clone().into()
    }
}

impl From<InternalHdSignatureForTransactionIntent>
    for HdSignatureForTransactionIntent
{
    fn from(value: InternalHdSignatureForTransactionIntent) -> Self {
        Self {
            input: value.input.into(),
            signature: value.signature.into(),
        }
    }
}

impl From<HdSignatureForTransactionIntent>
    for InternalHdSignatureForTransactionIntent
{
    fn from(value: HdSignatureForTransactionIntent) -> Self {
        Self {
            input: value.input.into_internal(),
            signature: value.signature.into_internal(),
        }
    }
}

decl_conversion_tests_for!(HdSignatureForTransactionIntent);
