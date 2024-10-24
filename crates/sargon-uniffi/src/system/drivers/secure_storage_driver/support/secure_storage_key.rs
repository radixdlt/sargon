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

#[uniffi::export]
pub fn secure_storage_key_identifier(key: &SecureStorageKey) -> String {
    key.into_internal().identifier()
}
