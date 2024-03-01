use crate::prelude::*;

use transaction::model::IntentSignaturesV1 as ScryptoIntentSignatures;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct IntentSignatures {
    pub signatures: Vec<IntentSignature>,
}

impl IntentSignatures {
    pub fn new(signatures: Vec<IntentSignature>) -> Self {
        Self { signatures }
    }
}

impl From<ScryptoIntentSignatures> for IntentSignatures {
    fn from(value: ScryptoIntentSignatures) -> Self {
        Self::new(value.signatures.into_iter().map(|s| s.into()).collect_vec())
    }
}
impl From<IntentSignatures> for ScryptoIntentSignatures {
    fn from(value: IntentSignatures) -> Self {
        Self {
            signatures: value
                .signatures
                .into_iter()
                .map(|s| s.into())
                .collect_vec(),
        }
    }
}
