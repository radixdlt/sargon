use crate::prelude::*;

/// A struct representing the integrity of a device factor source.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceFactorSourceIntegrity {
    /// The factor source that is linked to the entities.
    pub factor_source: DeviceFactorSource,

    /// Whether the mnemonic of the factor source is present in keychain.
    pub is_mnemonic_present_in_keychain: bool,

    /// Whether the mnemonic of the factor source is marked as backed up.
    pub is_mnemonic_marked_as_backed_up: bool,
}

impl DeviceFactorSourceIntegrity {
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

impl HasSampleValues for DeviceFactorSourceIntegrity {
    fn sample() -> Self {
        Self::new(DeviceFactorSource::sample(), true, true)
    }

    fn sample_other() -> Self {
        Self::new(DeviceFactorSource::sample_other(), false, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DeviceFactorSourceIntegrity;

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
