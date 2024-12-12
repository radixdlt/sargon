use crate::prelude::*;

/// An enum representing the accessibility of a factor source.
#[derive(Clone, Debug, PartialEq)]
pub enum FactorSourceAccessibility {
    Device(DeviceFactorSourceAccessibility),

    Ledger(LedgerHardwareWalletFactorSource),
}

impl HasSampleValues for FactorSourceAccessibility {
    fn sample() -> Self {
        Self::Device(DeviceFactorSourceAccessibility::sample())
    }

    fn sample_other() -> Self {
        Self::Ledger(LedgerHardwareWalletFactorSource::sample())
    }
}

impl From<DeviceFactorSourceAccessibility> for FactorSourceAccessibility {
    fn from(value: DeviceFactorSourceAccessibility) -> Self {
        Self::Device(value)
    }
}

impl From<LedgerHardwareWalletFactorSource> for FactorSourceAccessibility {
    fn from(value: LedgerHardwareWalletFactorSource) -> Self {
        Self::Ledger(value)
    }
}
