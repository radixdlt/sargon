use crate::prelude::*;
use sargon::FactorSource as InternalFactorSource;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    uniffi::Enum,
)]
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
            InternalFactorSource::OffDeviceMnemonic { value } => FactorSource::OffDeviceMnemonic {
                value: value.into(),
            },
            InternalFactorSource::ArculusCard { value } => FactorSource::ArculusCard {
                value: value.into(),
            },
            InternalFactorSource::SecurityQuestions { value } => FactorSource::SecurityQuestions {
                value: value.into(),
            },
            InternalFactorSource::TrustedContact { value } => FactorSource::TrustedContact {
                value: value.into(),
            },
        }
    }
}

impl Into<InternalFactorSource> for FactorSource {
    fn into(self) -> InternalFactorSource {
        match self {
            FactorSource::Device { value } => InternalFactorSource::Device {
                value: value.into(),
            },
            FactorSource::Ledger { value } => InternalFactorSource::Ledger {
                value: value.into(),
            },
            FactorSource::OffDeviceMnemonic { value } => InternalFactorSource::OffDeviceMnemonic {
                value: value.into(),
            },
            FactorSource::ArculusCard { value } => InternalFactorSource::ArculusCard {
                value: value.into(),
            },
            FactorSource::SecurityQuestions { value } => InternalFactorSource::SecurityQuestions {
                value: value.into(),
            },
            FactorSource::TrustedContact { value } => InternalFactorSource::TrustedContact {
                value: value.into(),
            },
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
pub fn new_factor_source_sample() -> FactorSource {
    InternalFactorSource::sample().into()
}

#[uniffi::export]
pub fn new_factor_source_sample_other() -> FactorSource {
    InternalFactorSource::sample_other().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorSource;

    #[test]
    fn to_string() {
        assert_eq!(factor_source_to_string(&SUT::sample()), "My precious iPhone SE 2nd gen device:f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a")
    }

    #[test]
    fn hash_sample_values() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_factor_source_sample(),
                new_factor_source_sample_other(),
                // duplicates should be removed
                new_factor_source_sample(),
                new_factor_source_sample_other(),
            ])
            .len(),
            2
        )
    }
}