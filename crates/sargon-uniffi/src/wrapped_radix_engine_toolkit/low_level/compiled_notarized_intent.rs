use crate::prelude::*;
use sargon::CompiledNotarizedIntent as InternalCompiledNotarizedIntent;

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct CompiledNotarizedIntent {
    secret_magic: BagOfBytes,
}

impl CompiledNotarizedIntent {
    pub fn into_internal(&self) -> InternalCompiledNotarizedIntent {
        self.clone().into()
    }
}

impl From<InternalCompiledNotarizedIntent> for CompiledNotarizedIntent {
    fn from(internal: InternalCompiledNotarizedIntent) -> Self {
        Self {
            secret_magic: internal.0.into(),
        }
    }
}

impl Into<InternalCompiledNotarizedIntent> for CompiledNotarizedIntent {
    fn into(self) -> InternalCompiledNotarizedIntent {
        InternalCompiledNotarizedIntent::new(self.secret_magic.into())
    }
}

#[uniffi::export]
pub fn new_compiled_notarized_intent_sample() -> CompiledNotarizedIntent {
    InternalCompiledNotarizedIntent::sample().into()
}

#[uniffi::export]
pub fn new_compiled_notarized_intent_sample_other() -> CompiledNotarizedIntent {
    InternalCompiledNotarizedIntent::sample_other().into()
}

#[uniffi::export]
pub fn compiled_notarized_intent_get_bytes(
    compiled_notarized_intent: &CompiledNotarizedIntent,
) -> BagOfBytes {
    compiled_notarized_intent.into_internal().bytes().into()
}
