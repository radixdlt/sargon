use crate::prelude::*;
use sargon::CheckSecurityProblemsInput as InternalCheckSecurityProblemsInput;
use sargon::IsCloudProfileSyncEnabled;

#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct CheckSecurityProblemsInput {
    /// Whether the cloud profile sync is enabled.
    pub is_cloud_profile_sync_enabled: bool,

    /// Addresses of entities that are unrecoverable. This is, the Factor Source used to create such entities
    /// has not been backed up (e.g. seed phrase was not written down).
    pub unrecoverable_entities: AddressesOfEntitiesInBadState,

    /// Addresses of entities that we don't have control over them. This is, the Factor Source used to create such entities
    /// is missing (e.g. entity was imported but seed phrase never entered).
    pub without_control_entities: AddressesOfEntitiesInBadState,

    /// Information about the latest backup made on the cloud.
    pub last_cloud_backup: Option<BackupResult>,

    /// Information about the latest backup made manually.
    pub last_manual_backup: Option<BackupResult>,
}

impl CheckSecurityProblemsInput {
    pub fn into_internal(&self) -> InternalCheckSecurityProblemsInput {
        self.clone().into()
    }
}

impl From<InternalCheckSecurityProblemsInput> for CheckSecurityProblemsInput {
    fn from(internal: InternalCheckSecurityProblemsInput) -> Self {
        Self {
            is_cloud_profile_sync_enabled: internal
                .is_cloud_profile_sync_enabled
                .0,
            unrecoverable_entities: internal.unrecoverable_entities.into(),
            without_control_entities: internal.without_control_entities.into(),
            last_cloud_backup: internal
                .last_cloud_backup
                .map(BackupResult::from),
            last_manual_backup: internal
                .last_manual_backup
                .map(BackupResult::from),
        }
    }
}

impl From<CheckSecurityProblemsInput> for InternalCheckSecurityProblemsInput {
    fn from(input: CheckSecurityProblemsInput) -> Self {
        InternalCheckSecurityProblemsInput {
            is_cloud_profile_sync_enabled: IsCloudProfileSyncEnabled(
                input.is_cloud_profile_sync_enabled,
            ),
            unrecoverable_entities: input.unrecoverable_entities.into(),
            without_control_entities: input.without_control_entities.into(),
            last_cloud_backup: input.last_cloud_backup.map(BackupResult::into),
            last_manual_backup: input
                .last_manual_backup
                .map(BackupResult::into),
        }
    }
}
