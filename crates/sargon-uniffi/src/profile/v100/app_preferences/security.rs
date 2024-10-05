use crate::prelude::*;

/// Controls e.g. if Profile Snapshot gets synced to iCloud or not, and whether
/// developer mode is enabled or not. In future (MFA) we will also save a list of
/// MFA security structure configurations.
#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Hash,
    derive_more::Display,
    uniffi::Record,
)]
#[display(
    "cloud? {}, dev? {}, advanced lock? {}",
    is_cloud_profile_sync_enabled,
    is_developer_mode_enabled,
    is_advanced_lock_enabled
)]
pub struct Security {
    pub is_cloud_profile_sync_enabled: IsCloudProfileSyncEnabled,
    pub is_developer_mode_enabled: IsDeveloperModeEnabled,

    pub is_advanced_lock_enabled: IsAdvancedLockEnabled,

    pub security_structures_of_factor_source_ids:
        SecurityStructuresOfFactorSourceIDs,
}

decl_bool_type!(IsCloudProfileSyncEnabled, true);
decl_bool_type!(IsDeveloperModeEnabled, false);
decl_bool_type!(IsAdvancedLockEnabled, false);

