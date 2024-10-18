use crate::prelude::*;
use sargon::OffDeviceMnemonicHint as InternalOffDeviceMnemonicHint;

/// Properties describing a DeviceFactorSource to help user disambiguate between
/// it and another one.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct OffDeviceMnemonicHint {
    pub display_name: DisplayName,
}
