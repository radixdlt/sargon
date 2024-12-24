use crate::prelude::*;
use sargon::CheckSecurityProblemsInput as InternalCheckSecurityProblemsInput;
use sargon::IsCloudProfileSyncEnabled;

#[derive(Clone, PartialEq, Eq, uniffi::Record, InternalConversion)]
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
