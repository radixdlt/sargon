use crate::prelude::*;
use sargon::IntentSignature as InternalIntentSignatures;

#[derive(Clone, Debug, PartialEq, Eq, Default, Hash,  uniffi::Record)]
pub struct IntentSignatures {
    pub signatures: Vec<IntentSignature>,
}

impl From<InternalIntentSignatures> for IntentSignatures {
    fn from(value: InternalIntentSignatures) -> Self {
        Self {
            signatures: value.signatures.into_vec(),
        }
    }
}

impl Into<InternalIntentSignatures> for IntentSignatures {
    fn into(self) -> InternalIntentSignatures {
        InternalIntentSignatures {
            signatures: self.signatures.into_internal_vec()
        }
    }
}