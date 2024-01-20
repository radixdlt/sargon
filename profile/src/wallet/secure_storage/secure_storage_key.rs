use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum SecureStorageKey {
    SnapshotHeadersList,
    ActiveProfileID,
    DeviceFactorSourceMnemonic {
        factor_source_id: FactorSourceIDFromHash,
    },
    ProfileSnapshot {
        profile_id: ProfileID,
    },
}
impl SecureStorageKey {
    #[cfg(not(tarpaulin_include))] // false negative
    pub fn identifier(&self) -> String {
        format!(
            "secure_storage_key_{}",
            match self {
                SecureStorageKey::ActiveProfileID => "activeProfileID".to_string(),
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

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn identifier() {
        assert_eq!(
            SecureStorageKey::ActiveProfileID.identifier(),
            "secure_storage_key_activeProfileID"
        );
        assert_eq!(
            SecureStorageKey::SnapshotHeadersList.identifier(),
            "secure_storage_key_headers"
        );
        assert_eq!(
            SecureStorageKey::DeviceFactorSourceMnemonic {
                factor_source_id: FactorSourceIDFromHash::placeholder()
            }
            .identifier(),
            "secure_storage_key_device_factor_source_device:3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
        );
        assert_eq!(
            SecureStorageKey::ProfileSnapshot {
                profile_id: ProfileID::placeholder()
            }
            .identifier(),
            "secure_storage_key_profile_snapshot_ffffffff-ffff-ffff-ffff-ffffffffffff"
        );
    }
}

#[cfg(test)]
mod uniffi_tests {
    use crate::prelude::*;

    #[test]
    fn identifier() {
        let key = SecureStorageKey::ProfileSnapshot {
            profile_id: ProfileID::placeholder(),
        };
        assert_eq!(
            key.clone().identifier(),
            secure_storage_key_identifier(&key)
        );
    }
}
