use crate::prelude::*;
use sargon::IntentSignatures as InternalIntentSignatures;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct IntentSignatures {
    pub signatures: Vec<IntentSignature>,
}
