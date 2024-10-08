use crate::prelude::*;
use sargon::Security as InternalSecurity;

/// Controls e.g. if Profile Snapshot gets synced to iCloud or not, and whether
/// developer mode is enabled or not. In future (MFA) we will also save a list of
/// MFA security structure configurations.
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Hash,
    uniffi::Record,
)]
pub struct Security {
    pub is_cloud_profile_sync_enabled: bool,
    pub is_developer_mode_enabled: bool,

    pub is_advanced_lock_enabled: bool,

    pub security_structures_of_factor_source_ids:
        SecurityStructuresOfFactorSourceIDs,
}

impl From<InternalSecurity> for Security {
    fn from(value: InternalSecurity) -> Self {
        Self {
            is_cloud_profile_sync_enabled: value.is_cloud_profile_sync_enabled,
            is_developer_mode_enabled: value.is_developer_mode_enabled,
            is_advanced_lock_enabled: value.is_advanced_lock_enabled,
            security_structures_of_factor_source_ids: value
                .security_structures_of_factor_source_ids
                .into(),
        }
    }
}

impl Into<InternalSecurity> for Security {
    fn into(self) -> InternalSecurity {
        InternalSecurity {
            is_cloud_profile_sync_enabled: self.is_cloud_profile_sync_enabled,
            is_developer_mode_enabled: self.is_developer_mode_enabled,
            is_advanced_lock_enabled: self.is_advanced_lock_enabled,
            security_structures_of_factor_source_ids: self
                .security_structures_of_factor_source_ids
                .into(),
        }
    }
}
