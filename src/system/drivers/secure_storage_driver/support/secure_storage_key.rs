use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum SecureStorageKey {
    ActiveProfileID,
    HostID,
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
                SecureStorageKey::ActiveProfileID =>
                    "activeProfileID".to_owned(),
                SecureStorageKey::HostID => "host_id".to_owned(),
                SecureStorageKey::DeviceFactorSourceMnemonic {
                    factor_source_id,
                } => format!("device_factor_source_{}", factor_source_id),
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
            SecureStorageKey::DeviceFactorSourceMnemonic {
                factor_source_id: FactorSourceIDFromHash::sample()
            }
            .identifier(),
            "secure_storage_key_device_factor_source_device:f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
        );
        assert_eq!(
            SecureStorageKey::ProfileSnapshot {
                profile_id: ProfileID::sample()
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
            profile_id: ProfileID::sample(),
        };
        assert_eq!(
            key.clone().identifier(),
            secure_storage_key_identifier(&key)
        );
    }
}
