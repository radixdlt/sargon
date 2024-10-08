use crate::prelude::*;

/// Properties describing a DeviceFactorSource to help user disambiguate between
/// it and another one.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    
    uniffi::Record,
)]
pub struct OffDeviceMnemonicHint {
    pub display_name: DisplayName,
}