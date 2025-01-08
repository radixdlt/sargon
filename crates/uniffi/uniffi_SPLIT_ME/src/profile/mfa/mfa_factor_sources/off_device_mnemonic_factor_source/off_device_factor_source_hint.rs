use crate::prelude::*;
use sargon::OffDeviceMnemonicHint as InternalOffDeviceMnemonicHint;

/// Properties describing a DeviceFactorSource to help user disambiguate between
/// it and another one.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct OffDeviceMnemonicHint {
    /// A user-assigned name for the passphrase, intended to help users
    /// differentiate between multiple passphrases.
    pub label: DisplayName,
}
