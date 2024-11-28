use crate::prelude::*;

/// A struct that represents the result of a given backup.
///
/// Reference for iOS: it is a combination of `BackupStatus` and `BackupResult` (all in one).
#[derive(Debug, Clone, PartialEq)]
pub struct BackupResult {
    /// The identifier of the backup.
    pub save_identifier: String,

    /// Whether this backup matches the one on Profile.
    pub is_current: bool,

    /// Whether this backup has failed.
    pub is_failed: bool,
}

impl BackupResult {
    pub fn new(save_identifier: String, is_current: bool, is_failed: bool) -> Self {
        Self {
            save_identifier,
            is_current,
            is_failed,
        }
    }
}

impl HasSampleValues for BackupResult {
    fn sample() -> Self {
        Self {
            save_identifier: String::sample(),
            is_current: true,
            is_failed: false,
        }
    }

    fn sample_other() -> Self {
        Self {
            save_identifier: String::sample_other(),
            is_current: false,
            is_failed: true,
        }
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
