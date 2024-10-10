use crate::prelude::*;
use sargon::PlaintextMessage as InternalPlaintextMessage;

#[derive( Clone, PartialEq, Eq, Hash,  uniffi::Record)]
pub struct PlaintextMessage {
    pub mime_type: String,
    pub message: MessageContents,
}

impl From<InternalPlaintextMessage> for PlaintextMessage {
    fn from(value: InternalPlaintextMessage) -> Self {
        Self {
            mime_type: value.mime_type,
            message: value.message.into(),
        }
    }
}

impl Into<InternalPlaintextMessage> for PlaintextMessage {
    fn into(self) -> InternalPlaintextMessage {
        InternalPlaintextMessage {
            mime_type: self.mime_type,
            message: self.message.into(),
        }
    }
}