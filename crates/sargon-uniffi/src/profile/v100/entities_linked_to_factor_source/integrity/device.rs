use crate::prelude::*;
use sargon::DeviceFactorSourceIntegrity as InternalDeviceFactorSourceIntegrity;

/// A struct representing the integrity of a device factor source.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct DeviceFactorSourceIntegrity {
    /// The factor source that is linked to the entities.
    pub factor_source: DeviceFactorSource,

    /// Whether the mnemonic of the factor source is present in keychain.
    pub is_mnemonic_present_in_keychain: bool,

    /// Whether the mnemonic of the factor source is marked as backed up.
    pub is_mnemonic_marked_as_backed_up: bool,
}
