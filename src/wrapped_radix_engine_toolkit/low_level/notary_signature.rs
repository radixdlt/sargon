use crate::prelude::*;

use transaction::model::NotarySignatureV1 as ScryptoNotarySignature;

#[derive(Debug, Clone, Eq, PartialEq, uniffi::Record)]
pub struct NotarySignature {
    pub(crate) secret_magic: Signature,
}

impl From<ScryptoNotarySignature> for NotarySignature {
    fn from(value: ScryptoNotarySignature) -> Self {
        Self {
            secret_magic: value.0.into(),
        }
    }
}
impl From<NotarySignature> for ScryptoNotarySignature {
    fn from(value: NotarySignature) -> Self {
        ScryptoNotarySignature(value.secret_magic.into())
    }
}
impl From<Signature> for NotarySignature {
    fn from(value: Signature) -> Self {
        Self {
            secret_magic: value,
        }
    }
}

impl From<Secp256k1Signature> for NotarySignature {
    fn from(value: Secp256k1Signature) -> Self {
        Signature::from(value).into()
    }
}

impl From<Ed25519Signature> for NotarySignature {
    fn from(value: Ed25519Signature) -> Self {
        value.into()
    }
}
