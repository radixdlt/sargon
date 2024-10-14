use crate::prelude::*;
use sargon::PlaintextMessage as InternalPlaintextMessage;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Record)]
pub struct PlaintextMessage {
    pub mime_type: String,
    pub message: MessageContents,
}
