use crate::prelude::*;
use sargon::IsAdvancedLockEnabled;
use sargon::IsCloudProfileSyncEnabled;
use sargon::IsDeveloperModeEnabled;
use sargon::Security as InternalSecurity;

/// Controls e.g. if Profile Snapshot gets synced to iCloud or not, and whether
/// developer mode is enabled or not. In future (MFA) we will also save a list of
/// MFA security structure configurations.
#[derive(PartialEq, Eq, Clone, Hash, uniffi::Record)]
pub struct Security {
    pub is_cloud_profile_sync_enabled: bool,
    pub is_developer_mode_enabled: bool,

    pub is_advanced_lock_enabled: bool,

    pub security_structures_of_factor_source_ids:
        Vec<SecurityStructureOfFactorSourceIDs>,
}

impl Security {
    pub fn into_internal(&self) -> InternalSecurity {
        self.clone().into()
    }
}

impl From<InternalSecurity> for Security {
    fn from(value: InternalSecurity) -> Self {
        Self {
            is_cloud_profile_sync_enabled: value
                .is_cloud_profile_sync_enabled
                .0,
            is_developer_mode_enabled: value.is_developer_mode_enabled.0,
            is_advanced_lock_enabled: value.is_advanced_lock_enabled.0,
            security_structures_of_factor_source_ids: value
                .security_structures_of_factor_source_ids
                .into_type(),
        }
    }
}

impl From<Security> for InternalSecurity {
    fn from(val: Security) -> Self {
        InternalSecurity {
            is_cloud_profile_sync_enabled: IsCloudProfileSyncEnabled(
                val.is_cloud_profile_sync_enabled,
            ),
            is_developer_mode_enabled: IsDeveloperModeEnabled(
                val.is_developer_mode_enabled,
            ),
            is_advanced_lock_enabled: IsAdvancedLockEnabled(
                val.is_advanced_lock_enabled,
            ),
            security_structures_of_factor_source_ids: val
                .security_structures_of_factor_source_ids
                .into_internal(),
        }
    }
}

decl_conversion_tests_for!(Security);
