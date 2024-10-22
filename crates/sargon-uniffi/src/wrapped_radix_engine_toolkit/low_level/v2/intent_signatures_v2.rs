use crate::prelude::*;
use sargon::IntentSignaturesV2 as InternalIntentSignaturesV2;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct IntentSignaturesV2 {
    pub signatures: Vec<IntentSignature>,
}