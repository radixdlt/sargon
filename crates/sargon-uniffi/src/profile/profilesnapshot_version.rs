use crate::prelude::*;
use sargon::ProfileSnapshotVersion as InternalProfileSnapshotVersion;

/// The version of the Profile Snapshot data format (JSON).
#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
#[repr(u16)]
pub enum ProfileSnapshotVersion {
    /// The version we went live with on Babylon mainnet 2023-09-28,
    /// shipped with iOS 1.0.0 (7) and Android v 1.0.0.
    V100 = 100,
}
