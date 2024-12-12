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
