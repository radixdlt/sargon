use crate::prelude::*;
use sargon::BaseIsFactorSource;
use sargon::FactorSource as InternalFactorSource;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
pub enum FactorSource {
    Device {
        value: DeviceFactorSource,
    },

    Ledger {
        value: LedgerHardwareWalletFactorSource,
    },

    OffDeviceMnemonic {
        value: OffDeviceMnemonicFactorSource,
    },

    ArculusCard {
        value: ArculusCardFactorSource,
    },

    SecurityQuestions {
        value: SecurityQuestions_NOT_PRODUCTION_READY_FactorSource,
    },

    TrustedContact {
        value: TrustedContactFactorSource,
    },

    Passphrase {
        value: PassphraseFactorSource,
    },
}

#[uniffi::export]
pub fn factor_source_to_string(factor_source: &FactorSource) -> String {
    factor_source.into_internal().to_string()
}

#[uniffi::export]
pub fn factor_source_supports_olympia(factor_source: &FactorSource) -> bool {
    factor_source.into_internal().supports_olympia()
}

#[uniffi::export]
pub fn factor_source_supports_babylon(factor_source: &FactorSource) -> bool {
    factor_source.into_internal().supports_babylon()
}

#[uniffi::export]
pub fn new_factor_source_sample() -> FactorSource {
    InternalFactorSource::sample().into()
}

#[uniffi::export]
pub fn new_factor_source_sample_other() -> FactorSource {
    InternalFactorSource::sample_other().into()
}
