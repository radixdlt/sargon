use crate::prelude::*;
use sargon::Subintent as InternalSubintent;

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct Subintent {
    pub header: IntentHeaderV2,
    pub manifest: SubintentManifest,
    pub message: MessageV2,
}

#[uniffi::export]
pub fn new_subintent(
    header: IntentHeaderV2,
    manifest: SubintentManifest,
    message: MessageV2,
) -> Result<Subintent> {
    InternalSubintent::new(header.into(), manifest.into(), message.into())
        .into_result()
}

#[uniffi::export]
pub fn new_subintent_sample() -> Subintent {
    InternalSubintent::sample().into()
}

#[uniffi::export]
pub fn new_subintent_sample_other() -> Subintent {
    InternalSubintent::sample_other().into()
}

#[uniffi::export]
pub fn subintent_hash(subintent: &Subintent) -> SubintentHash {
    subintent.into_internal().hash().into()
}

#[uniffi::export]
pub fn subintent_compile(subintent: &Subintent) -> CompiledSubintent {
    subintent.into_internal().compile().into()
}
