use crate::prelude::*;

/// An enum representing the integrity of a factor source.
#[derive(Clone, Debug, PartialEq)]
pub enum FactorSourceIntegrity {
    Device(DeviceFactorSourceIntegrity),

    Ledger(LedgerHardwareWalletFactorSource),
}

impl HasSampleValues for FactorSourceIntegrity {
    fn sample() -> Self {
        Self::Device(DeviceFactorSourceIntegrity::sample())
    }

    fn sample_other() -> Self {
        Self::Ledger(LedgerHardwareWalletFactorSource::sample())
    }
}

impl From<DeviceFactorSourceIntegrity> for FactorSourceIntegrity {
    fn from(value: DeviceFactorSourceIntegrity) -> Self {
        Self::Device(value)
    }
}

impl From<LedgerHardwareWalletFactorSource> for FactorSourceIntegrity {
    fn from(value: LedgerHardwareWalletFactorSource) -> Self {
        Self::Ledger(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorSourceIntegrity;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn from_device() {
        assert_eq!(SUT::sample(), DeviceFactorSourceIntegrity::sample().into())
    }

    #[test]
    fn from_ledger() {
        assert_eq!(
            SUT::sample_other(),
            LedgerHardwareWalletFactorSource::sample().into()
        )
    }
}
