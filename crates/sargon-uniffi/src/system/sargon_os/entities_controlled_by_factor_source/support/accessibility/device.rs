use crate::prelude::*;
use sargon::DeviceFactorSourceAccessibility as InternalDeviceFactorSourceAccessibility;

/// A struct representing the accessibility of a device factor source.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct DeviceFactorSourceAccessibility {
    /// The factor source that controls the entities.
    pub factor_source: DeviceFactorSource,

    /// Whether the mnemonic of the factor source is present in keychain.
    pub is_mnemonic_present_in_keychain: bool,

    /// Whether the mnemonic of the factor source is marked as backed up.
    pub is_mnemonic_marked_as_backed_up: bool,
}
