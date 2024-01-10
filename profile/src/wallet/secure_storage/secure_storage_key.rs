use crate::{FactorSourceIDFromHash, ProfileID};

#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum SecureStorageKey {
    SnapshotHeadersList,
    DeviceFactorSourceMnemonic {
        factor_source_id: FactorSourceIDFromHash,
    },
    ProfileSnapshot {
        profile_id: ProfileID,
    },
}
impl SecureStorageKey {
    pub fn identifier(&self) -> String {
        format!(
            "secure_storage_key_{}",
            match self {
                SecureStorageKey::SnapshotHeadersList => "headers".to_string(),
                SecureStorageKey::DeviceFactorSourceMnemonic { factor_source_id } =>
                    format!("device_factor_source_{}", factor_source_id.to_string()),
                SecureStorageKey::ProfileSnapshot { profile_id } =>
                    format!("profile_snapshot_{}", profile_id),
            }
        )
    }
}

#[uniffi::export]
pub fn secure_storage_key_identifier(key: &SecureStorageKey) -> String {
    key.identifier()
}
