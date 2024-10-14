use crate::prelude::*;
use sargon::IntentSignatures as InternalIntentSignatures;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Record)]
pub struct IntentSignatures {
    pub signatures: Vec<IntentSignature>,
}
