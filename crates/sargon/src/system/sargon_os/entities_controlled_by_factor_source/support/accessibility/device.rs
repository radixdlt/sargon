use crate::prelude::*;

/// A struct representing the accessibility of a device factor source.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceFactorSourceAccessibility {
    /// The factor source that controls the entities.
    pub factor_source: DeviceFactorSource,

    /// Whether the mnemonic of the factor source is present in keychain.
    pub is_mnemonic_present_in_keychain: bool,

    /// Whether the mnemonic of the factor source is marked as backed up.
    pub is_mnemonic_marked_as_backed_up: bool,
}

impl DeviceFactorSourceAccessibility {
    pub fn new(
        factor_source: DeviceFactorSource,
        is_mnemonic_present_in_keychain: bool,
        is_mnemonic_marked_as_backed_up: bool,
    ) -> Self {
        Self {
            factor_source,
            is_mnemonic_present_in_keychain,
            is_mnemonic_marked_as_backed_up,
        }
    }
}

impl HasSampleValues for DeviceFactorSourceAccessibility {
    fn sample() -> Self {
        Self::new(DeviceFactorSource::sample(), true, true)
    }

    fn sample_other() -> Self {
        Self::new(DeviceFactorSource::sample_other(), false, false)
    }
}
