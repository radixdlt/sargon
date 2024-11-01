use crate::prelude::*;
use sargon::SignedSubintent as InternalSignedSubintent;

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct SignedSubintent {
    pub subintent: Subintent,
    pub subintent_signatures: IntentSignatures,
}