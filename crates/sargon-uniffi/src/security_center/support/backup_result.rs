use crate::prelude::*;
use sargon::BackupResult as InternalBackupResult;
use sargon::IsBackupResultCurrent;
use sargon::IsBackupResultFailed;

/// A struct that represents the result of a given backup.
///
/// Reference for iOS: it is a combination of `BackupStatus` and `BackupResult` (all in one).
#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct BackupResult {
    /// The identifier of the backup.
    pub save_identifier: String,

    /// Whether this backup matches the one on Profile.
    pub is_current: bool,

    /// Whether this backup has failed.
    pub is_failed: bool,
}

impl From<InternalBackupResult> for BackupResult {
    fn from(internal: InternalBackupResult) -> Self {
        Self {
            save_identifier: internal.save_identifier,
            is_current: internal.is_current.0,
            is_failed: internal.is_failed.0,
        }
    }
}

impl From<BackupResult> for InternalBackupResult {
    fn from(backup_result: BackupResult) -> Self {
        InternalBackupResult {
            save_identifier: backup_result.save_identifier,
            is_current: IsBackupResultCurrent(backup_result.is_current),
            is_failed: IsBackupResultFailed(backup_result.is_failed),
        }
    }
}
