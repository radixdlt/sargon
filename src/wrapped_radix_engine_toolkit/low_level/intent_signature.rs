use crate::prelude::*;

use transaction::model::IntentSignatureV1 as ScryptoIntentSignature;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct IntentSignature {
    pub(crate) secret_magic: SignatureWithPublicKey,
}
impl From<IntentSignature> for ScryptoIntentSignature {
    fn from(value: IntentSignature) -> Self {
        ScryptoIntentSignature(value.secret_magic.into())
    }
}
impl From<ScryptoIntentSignature> for IntentSignature {
    fn from(value: ScryptoIntentSignature) -> Self {
        Self {
            secret_magic: value.0.into(),
        }
    }
}
