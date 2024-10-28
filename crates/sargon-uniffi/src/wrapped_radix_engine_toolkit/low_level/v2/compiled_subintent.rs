use crate::prelude::*;
use base64::prelude::BASE64_STANDARD_NO_PAD;
use base64::Engine;
use sargon::CompiledSubintent as InternalCompiledSubintent;

#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct CompiledSubintent {
    /// A base-64 encoded version of the compiled subintent
    secret_magic: String,
}

impl From<InternalCompiledSubintent> for CompiledSubintent {
    fn from(value: InternalCompiledSubintent) -> Self {
        Self {
            secret_magic: BASE64_STANDARD_NO_PAD.encode(value.bytes().bytes()),
        }
    }
}

impl CompiledSubintent {
    pub fn into_internal(&self) -> InternalCompiledSubintent {
        self.clone().into()
    }
}

impl From<CompiledSubintent> for InternalCompiledSubintent {
    fn from(val: CompiledSubintent) -> Self {
        let decoded = BASE64_STANDARD_NO_PAD
            .decode(val.secret_magic)
            .expect("Should always be able to decode base-64 encoded bytes");

        InternalCompiledSubintent::new(sargon::BagOfBytes::from(decoded))
            .expect("Should always be able to compile a Subintent")
    }
}

#[uniffi::export]
pub fn new_compiled_subintent_sample() -> CompiledSubintent {
    InternalCompiledSubintent::sample().into()
}

#[uniffi::export]
pub fn new_compiled_subintent_sample_other() -> CompiledSubintent {
    InternalCompiledSubintent::sample_other().into()
}

#[uniffi::export]
pub fn compiled_subintent_bytes(
    compiled_intent: &CompiledSubintent,
) -> BagOfBytes {
    compiled_intent.into_internal().bytes().into()
}

#[uniffi::export]
pub fn compiled_subintent_decompile(
    compiled_intent: &CompiledSubintent,
) -> Subintent {
    compiled_intent.into_internal().decompile().into()
}

decl_conversion_tests_for!(CompiledSubintent);
