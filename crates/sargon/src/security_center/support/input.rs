use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct CheckSecurityProblemsInput {
    pub is_cloud_profile_sync_enabled: IsCloudProfileSyncEnabled,
    pub unrecoverable_entities: AddressesOfEntitiesInBadState,
    pub without_control_entities: AddressesOfEntitiesInBadState,
    pub last_cloud_backup: Option<BackupResult>,
    pub last_manual_backup: Option<BackupResult>,
}

impl CheckSecurityProblemsInput {
    pub fn new(
        is_cloud_profile_sync_enabled: IsCloudProfileSyncEnabled,
        unrecoverable_entities: AddressesOfEntitiesInBadState,
        without_control_entities: AddressesOfEntitiesInBadState,
        last_cloud_backup: impl Into<Option<BackupResult>>,
        last_manual_backup: impl Into<Option<BackupResult>>,
    ) -> Self {
        Self {
            is_cloud_profile_sync_enabled,
            unrecoverable_entities,
            without_control_entities,
            last_cloud_backup: last_cloud_backup.into(),
            last_manual_backup: last_manual_backup.into(),
        }
    }
}

impl HasSampleValues for CheckSecurityProblemsInput {
    fn sample() -> Self {
        Self::new(
            IsCloudProfileSyncEnabled::sample(),
            AddressesOfEntitiesInBadState::sample(),
            AddressesOfEntitiesInBadState::empty(),
            BackupResult::sample(),
            None,
        )
    }

    fn sample_other() -> Self {
        Self::new(
            IsCloudProfileSyncEnabled::sample_other(),
            AddressesOfEntitiesInBadState::empty(),
            AddressesOfEntitiesInBadState::sample_other(),
            None,
            BackupResult::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = CheckSecurityProblemsInput;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
