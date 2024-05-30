use crate::prelude::*;

#[derive(
    Serialize,
    Deserialize,
    Clone,
    EnumAsInner,
    Debug,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    uniffi::Enum,
)]
#[serde(untagged, remote = "Self")]
pub enum FactorSource {
    Device {
        #[serde(rename = "device")]
        #[display("DeviceFS({value})")]
        value: DeviceFactorSource,
    },

    Ledger {
        #[serde(rename = "ledgerHQHardwareWallet")]
        #[display("LedgerHWFS({value})")]
        value: LedgerHardwareWalletFactorSource,
    },

    OffDeviceMnemonic {
        #[serde(rename = "offDeviceMnemonic")]
        #[display("OffDevice({value})")]
        value: OffDeviceMnemonicFactorSource,
    },

    ArculusCard {
        #[serde(rename = "arculusCard")]
        #[display("ArculusCard({value})")]
        value: ArculusCardFactorSource,
    },

    SecurityQuestions {
        #[serde(rename = "securityQuestions")]
        #[display("SecurityQuestions({value})")]
        value: SecurityQuestions_NOT_PRODUCTION_READY_FactorSource,
    },
}

impl BaseIsFactorSource for FactorSource {
    fn common_properties(&self) -> FactorSourceCommon {
        match self {
            FactorSource::Device { value } => value.common_properties(),
            FactorSource::Ledger { value } => value.common_properties(),
            FactorSource::SecurityQuestions { value } => {
                value.common_properties()
            }
            FactorSource::ArculusCard { value } => value.common_properties(),
            FactorSource::OffDeviceMnemonic { value } => {
                value.common_properties()
            }
        }
    }

    fn factor_source_kind(&self) -> FactorSourceKind {
        match self {
            FactorSource::Device { value } => value.factor_source_kind(),
            FactorSource::Ledger { value } => value.factor_source_kind(),
            FactorSource::SecurityQuestions { value } => {
                value.factor_source_kind()
            }
            FactorSource::ArculusCard { value } => value.factor_source_kind(),
            FactorSource::OffDeviceMnemonic { value } => {
                value.factor_source_kind()
            }
        }
    }

    fn factor_source_id(&self) -> FactorSourceID {
        match self {
            FactorSource::Device { value } => value.factor_source_id(),
            FactorSource::Ledger { value } => value.factor_source_id(),
            FactorSource::SecurityQuestions { value } => {
                value.factor_source_id()
            }
            FactorSource::ArculusCard { value } => value.factor_source_id(),
            FactorSource::OffDeviceMnemonic { value } => {
                value.factor_source_id()
            }
        }
    }
}

impl Identifiable for FactorSource {
    type ID = FactorSourceID;

    fn id(&self) -> Self::ID {
        self.factor_source_id()
    }
}

impl From<DeviceFactorSource> for FactorSource {
    fn from(value: DeviceFactorSource) -> Self {
        FactorSource::Device { value }
    }
}

impl From<LedgerHardwareWalletFactorSource> for FactorSource {
    fn from(value: LedgerHardwareWalletFactorSource) -> Self {
        FactorSource::Ledger { value }
    }
}

impl<'de> Deserialize<'de> for FactorSource {
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        // https://github.com/serde-rs/serde/issues/1343#issuecomment-409698470
        #[derive(Deserialize, Serialize)]
        struct Wrapper {
            discriminator: String,
            #[serde(flatten, with = "FactorSource")]
            factor: FactorSource,
        }
        Wrapper::deserialize(deserializer).map(|w| w.factor)
    }
}

impl Serialize for FactorSource {
    #[cfg(not(tarpaulin_include))] // false negative
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("FactorSource", 2)?;
        let discriminator_key = "discriminator";
        match self {
            FactorSource::Device { value } => {
                let discriminant = "device";
                state.serialize_field(discriminator_key, discriminant)?;
                state.serialize_field(discriminant, value)?;
            }
            FactorSource::Ledger { value } => {
                let discriminant = "ledgerHQHardwareWallet";
                state.serialize_field(discriminator_key, discriminant)?;
                state.serialize_field(discriminant, value)?;
            }
            FactorSource::ArculusCard { value } => {
                let discriminant = "arculusCard";
                state.serialize_field(discriminator_key, discriminant)?;
                state.serialize_field(discriminant, value)?;
            }
            FactorSource::OffDeviceMnemonic { value } => {
                let discriminant = "offDeviceMnemonic";
                state.serialize_field(discriminator_key, discriminant)?;
                state.serialize_field(discriminant, value)?;
            }
            FactorSource::SecurityQuestions { value } => {
                let discriminant = "securityQuestions";
                state.serialize_field(discriminator_key, discriminant)?;
                state.serialize_field(discriminant, value)?;
            }
        }
        state.end()
    }
}

impl HasSampleValues for FactorSource {
    fn sample() -> Self {
        Self::sample_device()
    }

    fn sample_other() -> Self {
        Self::sample_ledger()
    }
}

impl FactorSource {
    pub fn sample_device() -> Self {
        Self::sample_device_babylon()
    }

    pub fn sample_device_babylon() -> Self {
        Self::Device {
            value: DeviceFactorSource::sample_babylon(),
        }
    }

    pub fn sample_device_olympia() -> Self {
        Self::Device {
            value: DeviceFactorSource::sample_olympia(),
        }
    }

    pub fn sample_ledger() -> Self {
        Self::Ledger {
            value: LedgerHardwareWalletFactorSource::sample(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorSource;

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
    fn device_common_properties() {
        assert_eq!(
            SUT::sample().common_properties(),
            DeviceFactorSource::sample_babylon().common
        );

        assert_eq!(
            SUT::sample_other().common_properties(),
            LedgerHardwareWalletFactorSource::sample().common
        )
    }

    #[test]
    fn factor_source_id_device() {
        assert_eq!(
            SUT::sample_device().factor_source_id(),
            DeviceFactorSource::sample().factor_source_id()
        );
    }

    #[test]
    fn factor_source_id_ledger() {
        assert_eq!(
            SUT::sample_ledger().factor_source_id(),
            LedgerHardwareWalletFactorSource::sample().factor_source_id()
        );
    }

    #[test]
    fn factor_source_kind_device() {
        assert_eq!(
            SUT::sample_device().factor_source_kind(),
            FactorSourceKind::Device
        );
    }

    #[test]
    fn factor_source_kind_ledger() {
        assert_eq!(
            SUT::sample_ledger().factor_source_kind(),
            FactorSourceKind::LedgerHQHardwareWallet
        );
    }

    #[test]
    fn into_from_device() {
        let factor_source: SUT = DeviceFactorSource::sample().into();
        assert_eq!(
            factor_source,
            SUT::Device {
                value: DeviceFactorSource::sample()
            }
        );
    }

    #[test]
    fn into_from_ledger() {
        let factor_source: SUT =
            LedgerHardwareWalletFactorSource::sample().into();
        assert_eq!(
            factor_source,
            SUT::Ledger {
                value: LedgerHardwareWalletFactorSource::sample()
            }
        );
    }

    #[test]
    fn json_roundtrip_device() {
        let model = SUT::sample_device();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "discriminator": "device",
                "device": {
                    "id": {
                        "kind": "device",
                        "body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
                    },
                    "common": {
                        "flags": ["main"],
                        "addedOn": "2023-09-11T16:05:56.000Z",
                        "cryptoParameters": {
                            "supportedCurves": ["curve25519"],
                            "supportedDerivationPathSchemes": ["cap26"]
                        },
                        "lastUsedOn": "2023-09-11T16:05:56.000Z"
                    },
                    "hint": {
                        "name": "iPhone",
                        "model": "iPhone",
                        "mnemonicWordCount": 24
                    }
                }
            }
            "#,
        )
    }

    #[test]
    fn json_roundtrip_ledger() {
        let model = SUT::sample_ledger();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "discriminator": "ledgerHQHardwareWallet",
                "ledgerHQHardwareWallet": {
                    "id": {
                        "kind": "ledgerHQHardwareWallet",
                        "body": "ab59987eedd181fe98e512c1ba0f5ff059f11b5c7c56f15614dcc9fe03fec58b"
                    },
                    "common": {
                        "addedOn": "2023-09-11T16:05:56.000Z",
                        "cryptoParameters": {
                            "supportedCurves": ["curve25519"],
                            "supportedDerivationPathSchemes": ["cap26"]
                        },
                        "flags": [],
                        "lastUsedOn": "2023-09-11T16:05:56.000Z"
                    },
                    "hint": {
                        "name": "Orange, scratched",
                        "model": "nanoS+"
                    }
                }
            }
            "#,
        )
    }
}
