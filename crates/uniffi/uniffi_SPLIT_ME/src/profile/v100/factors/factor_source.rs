use crate::prelude::*;
use sargon::BaseBaseIsFactorSource;
use sargon::FactorSource as InternalFactorSource;

decl_vec_samples_for!(FactorSources, FactorSource);

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Enum)]
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

    Password {
        value: PasswordFactorSource,
    },
}

delegate_debug_into!(FactorSource, InternalFactorSource);

impl FactorSource {
    pub fn into_internal(&self) -> InternalFactorSource {
        self.clone().into()
    }
}

impl From<InternalFactorSource> for FactorSource {
    fn from(value: InternalFactorSource) -> Self {
        match value {
            InternalFactorSource::Device { value } => FactorSource::Device {
                value: value.into(),
            },
            InternalFactorSource::Ledger { value } => FactorSource::Ledger {
                value: value.into(),
            },
            InternalFactorSource::OffDeviceMnemonic { value } => {
                FactorSource::OffDeviceMnemonic {
                    value: value.into(),
                }
            }
            InternalFactorSource::ArculusCard { value } => {
                FactorSource::ArculusCard {
                    value: value.into(),
                }
            }
            InternalFactorSource::Password { value } => {
                FactorSource::Password {
                    value: value.into(),
                }
            }
            InternalFactorSource::TrustedContact { value: _ } => {
                panic!("TrustedContact not yet supported in the Wallet")
            }
            InternalFactorSource::SecurityQuestions { value: _ } => {
                panic!("SecurityQuestions not yet supported in the Wallet")
            }
        }
    }
}

impl From<FactorSource> for InternalFactorSource {
    fn from(val: FactorSource) -> Self {
        match val {
            FactorSource::Device { value } => InternalFactorSource::Device {
                value: value.into_internal(),
            },
            FactorSource::Ledger { value } => InternalFactorSource::Ledger {
                value: value.into_internal(),
            },
            FactorSource::OffDeviceMnemonic { value } => {
                InternalFactorSource::OffDeviceMnemonic {
                    value: value.into_internal(),
                }
            }
            FactorSource::ArculusCard { value } => {
                InternalFactorSource::ArculusCard {
                    value: value.into_internal(),
                }
            }
            FactorSource::Password { value } => {
                InternalFactorSource::Password {
                    value: value.into_internal(),
                }
            }
        }
    }
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
pub fn factor_source_name(factor_source: &FactorSource) -> String {
    factor_source.into_internal().name()
}

#[uniffi::export]
pub fn new_factor_source_sample() -> FactorSource {
    InternalFactorSource::sample().into()
}

#[uniffi::export]
pub fn new_factor_source_sample_other() -> FactorSource {
    InternalFactorSource::sample_other().into()
}
