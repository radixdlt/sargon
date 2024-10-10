
use sargon::ProfileSnapshotVersion as InternalProfileSnapshotVersion;

/// The version of the Profile Snapshot data format (JSON).
#[derive(
    Clone,
    
    
    PartialEq,
    Eq,
    Hash,
    uniffi::Enum,
)]
pub enum ProfileSnapshotVersion {
    /// The version we went live with on Babylon mainnet 2023-09-28,
    /// shipped with iOS 1.0.0 (7) and Android v 1.0.0.
    V100 = 100,
}

impl From<InternalProfileSnapshotVersion> for ProfileSnapshotVersion {
    fn from(value: InternalProfileSnapshotVersion) -> Self {
        match value {
            InternalProfileSnapshotVersion::V100 => ProfileSnapshotVersion::V100,
        }
    }
}

impl Into<InternalProfileSnapshotVersion> for ProfileSnapshotVersion {
    fn into(self) -> InternalProfileSnapshotVersion {
        match self {
            ProfileSnapshotVersion::V100 => InternalProfileSnapshotVersion::V100,
        }
    }
}