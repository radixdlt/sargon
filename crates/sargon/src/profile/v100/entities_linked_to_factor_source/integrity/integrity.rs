use crate::prelude::*;

/// An enum representing the integrity of a factor source.
#[derive(Clone, Debug, PartialEq)]
pub enum FactorSourceIntegrity {
    Device(DeviceFactorSourceIntegrity),

    Ledger(LedgerHardwareWalletFactorSource),

    OffDeviceMnemonic(OffDeviceMnemonicFactorSource),

    ArculusCard(ArculusCardFactorSource),

    Password(PasswordFactorSource),
}

impl HasSampleValues for FactorSourceIntegrity {
    fn sample() -> Self {
        Self::sample_device()
    }

    fn sample_other() -> Self {
        Self::sample_ledger()
    }
}

impl FactorSourceIntegrity {
    pub fn sample_device() -> Self {
        DeviceFactorSourceIntegrity::sample().into()
    }

    pub fn sample_ledger() -> Self {
        LedgerHardwareWalletFactorSource::sample().into()
    }

    pub fn sample_off_device_mnemonic() -> Self {
        OffDeviceMnemonicFactorSource::sample().into()
    }

    pub fn sample_arculus_card() -> Self {
        ArculusCardFactorSource::sample().into()
    }

    pub fn sample_password() -> Self {
        PasswordFactorSource::sample().into()
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

impl From<OffDeviceMnemonicFactorSource> for FactorSourceIntegrity {
    fn from(value: OffDeviceMnemonicFactorSource) -> Self {
        Self::OffDeviceMnemonic(value)
    }
}

impl From<ArculusCardFactorSource> for FactorSourceIntegrity {
    fn from(value: ArculusCardFactorSource) -> Self {
        Self::ArculusCard(value)
    }
}

impl From<PasswordFactorSource> for FactorSourceIntegrity {
    fn from(value: PasswordFactorSource) -> Self {
        Self::Password(value)
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

    #[test]
    fn from_off_device_mnemonic() {
        assert_eq!(
            SUT::sample_off_device_mnemonic(),
            OffDeviceMnemonicFactorSource::sample().into()
        )
    }

    #[test]
    fn from_arculus_card() {
        assert_eq!(
            SUT::sample_arculus_card(),
            ArculusCardFactorSource::sample().into()
        )
    }

    #[test]
    fn from_password() {
        assert_eq!(
            SUT::sample_password(),
            PasswordFactorSource::sample().into()
        )
    }
}
