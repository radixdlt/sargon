use crate::prelude::*;
use sargon::UnsafeStorageKey as InternalUnsafeStorageKey;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum UnsafeStorageKey {
    FactorSourceUserHasWrittenDown,
}

impl UnsafeStorageKey {
    pub fn identifier(&self) -> String {
        format!(
            "unsafe_storage_key_{}",
            match self {
                UnsafeStorageKey::FactorSourceUserHasWrittenDown =>
                    "factor_source_user_has_written_down".to_owned(),
            }
        )
    }
}

#[uniffi::export]
pub fn unsafe_storage_key_identifier(key: &UnsafeStorageKey) -> String {
    key.identifier()
}
