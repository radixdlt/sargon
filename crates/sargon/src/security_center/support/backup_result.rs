use crate::prelude::*;

/// A struct that represents the result of a given backup.
///
/// Reference for iOS: it is a combination of `BackupStatus` and `BackupResult` (all in one).
#[derive(Debug, Clone, PartialEq)]
pub struct BackupResult {
    /// Whether this backup matches the one on Profile.
    pub is_current: IsBackupResultCurrent,

    /// Whether this backup has failed.
    pub is_failed: IsBackupResultFailed,
}

decl_bool_type!(IsBackupResultCurrent, false);
decl_bool_type!(IsBackupResultFailed, false);

impl BackupResult {
    pub fn new(
        is_current: IsBackupResultCurrent,
        is_failed: IsBackupResultFailed,
    ) -> Self {
        Self {
            is_current,
            is_failed,
        }
    }
}

impl HasSampleValues for BackupResult {
    fn sample() -> Self {
        Self::new(IsBackupResultCurrent(true), IsBackupResultFailed(false))
    }

    fn sample_other() -> Self {
        Self::new(IsBackupResultCurrent(false), IsBackupResultFailed(true))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = BackupResult;

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
