use crate::prelude::*;
use sargon::UnsafeStorageKey as InternalUnsafeStorageKey;

#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum UnsafeStorageKey {
    FactorSourceUserHasWrittenDown,
}

impl From<InternalUnsafeStorageKey> for UnsafeStorageKey {
    fn from(value: InternalUnsafeStorageKey) -> Self {
        match value {
            InternalUnsafeStorageKey::FactorSourceUserHasWrittenDown =>
                UnsafeStorageKey::FactorSourceUserHasWrittenDown,
        }
    }
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

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn identifier() {
        assert_eq!(
            unsafe_storage_key_identifier(
                &UnsafeStorageKey::FactorSourceUserHasWrittenDown
            ),
            "unsafe_storage_key_factor_source_user_has_written_down"
        );
    }
}
