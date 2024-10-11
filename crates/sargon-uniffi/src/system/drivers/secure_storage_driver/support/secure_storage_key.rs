use crate::prelude::*;
use sargon::SecureStorageKey as InternalSecureStorageKey;

#[derive(Clone, Eq, PartialEq, InternalConversion, uniffi::Enum)]
pub enum SecureStorageKey {
    HostID,
    DeviceFactorSourceMnemonic {
        factor_source_id: FactorSourceIDFromHash,
    },
    ProfileSnapshot {
        // Note:
        // `profile_id` is only meant to be used by the iOS Host for backward compatibility.
        // iOS Host stores multiple profiles in the secure storage uniquely identified by `profile_id`,
        // while Android Host stores only one profile in the secure storage.
        profile_id: ProfileID,
    },
}

delegate_display_debug_into!(SecureStorageKey, InternalSecureStorageKey);

impl From<InternalSecureStorageKey> for SecureStorageKey {
    fn from(value: InternalSecureStorageKey) -> Self {
        match value {
            InternalSecureStorageKey::HostID => SecureStorageKey::HostID,
            InternalSecureStorageKey::DeviceFactorSourceMnemonic {
                factor_source_id,
            } => SecureStorageKey::DeviceFactorSourceMnemonic {
                factor_source_id: factor_source_id.into(),
            },
            InternalSecureStorageKey::ProfileSnapshot { profile_id } => {
                SecureStorageKey::ProfileSnapshot {
                    profile_id: profile_id.into(),
                }
            }
        }
    }
}

impl Into<InternalSecureStorageKey> for SecureStorageKey {
    fn into(self) -> InternalSecureStorageKey {
        match self {
            SecureStorageKey::HostID => InternalSecureStorageKey::HostID,
            SecureStorageKey::DeviceFactorSourceMnemonic {
                factor_source_id,
            } => InternalSecureStorageKey::DeviceFactorSourceMnemonic {
                factor_source_id: factor_source_id.into(),
            },
            SecureStorageKey::ProfileSnapshot { profile_id } => {
                InternalSecureStorageKey::ProfileSnapshot {
                    profile_id: profile_id.into(),
                }
            }
        }
    }
}

#[uniffi::export]
pub fn secure_storage_key_identifier(key: &SecureStorageKey) -> String {
    key.into_internal().identifier()
}
