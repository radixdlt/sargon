use crate::prelude::*;
use sargon::OffDeviceMnemonicHint as InternalOffDeviceMnemonicHint;

/// Properties describing a DeviceFactorSource to help user disambiguate between
/// it and another one.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct OffDeviceMnemonicHint {
    pub display_name: DisplayName,
}

impl From<InternalOffDeviceMnemonicHint> for OffDeviceMnemonicHint {
    fn from(value: InternalOffDeviceMnemonicHint) -> Self {
        Self {
            display_name: value.display_name.into(),
        }
    }
}

impl Into<InternalOffDeviceMnemonicHint> for OffDeviceMnemonicHint {
    fn into(self) -> InternalOffDeviceMnemonicHint {
        InternalOffDeviceMnemonicHint {
            display_name: self.display_name.into(),
        }
    }
}
