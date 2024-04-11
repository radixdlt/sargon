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
}

impl BaseIsFactorSource for FactorSource {
    fn factor_source_kind(&self) -> FactorSourceKind {
        match self {
            FactorSource::Device { value } => value.factor_source_kind(),
            FactorSource::Ledger { value } => value.factor_source_kind(),
        }
    }

    fn factor_source_id(&self) -> FactorSourceID {
        match self {
            FactorSource::Device { value } => value.factor_source_id(),
            FactorSource::Ledger { value } => value.factor_source_id(),
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
            FactorSource::Device { value: device } => {
                let discriminant = "device";
                state.serialize_field(discriminator_key, discriminant)?;
                state.serialize_field(discriminant, device)?;
            }
            FactorSource::Ledger { value: ledger } => {
                let discriminant = "ledgerHQHardwareWallet";
                state.serialize_field(discriminator_key, discriminant)?;
                state.serialize_field(discriminant, ledger)?;
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
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(FactorSource::sample(), FactorSource::sample());
        assert_eq!(FactorSource::sample_other(), FactorSource::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(FactorSource::sample(), FactorSource::sample_other());
    }

    #[test]
    fn factor_source_id_device() {
        assert_eq!(
            FactorSource::sample_device().factor_source_id(),
            DeviceFactorSource::sample().factor_source_id()
        );
    }

    #[test]
    fn factor_source_id_ledger() {
        assert_eq!(
            FactorSource::sample_ledger().factor_source_id(),
            LedgerHardwareWalletFactorSource::sample().factor_source_id()
        );
    }

    #[test]
    fn factor_source_kind_device() {
        assert_eq!(
            FactorSource::sample_device().factor_source_kind(),
            FactorSourceKind::Device
        );
    }

    #[test]
    fn factor_source_kind_ledger() {
        assert_eq!(
            FactorSource::sample_ledger().factor_source_kind(),
            FactorSourceKind::LedgerHQHardwareWallet
        );
    }

    #[test]
    fn into_from_device() {
        let factor_source: FactorSource = DeviceFactorSource::sample().into();
        assert_eq!(
            factor_source,
            FactorSource::Device {
                value: DeviceFactorSource::sample()
            }
        );
    }

    #[test]
    fn into_from_ledger() {
        let factor_source: FactorSource =
            LedgerHardwareWalletFactorSource::sample().into();
        assert_eq!(
            factor_source,
            FactorSource::Ledger {
                value: LedgerHardwareWalletFactorSource::sample()
            }
        );
    }

    #[test]
    fn json_roundtrip_device() {
        let model = FactorSource::sample_device();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "discriminator": "device",
                "device": {
                    "id": {
                        "kind": "device",
                        "body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
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
                        "name": "Unknown Name",
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
        let model = FactorSource::sample_ledger();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "discriminator": "ledgerHQHardwareWallet",
                "ledgerHQHardwareWallet": {
                    "id": {
                        "kind": "ledgerHQHardwareWallet",
                        "body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
                    },
                    "common": {
                        "addedOn": "2023-09-11T16:05:56.000Z",
                        "cryptoParameters": {
                            "supportedCurves": ["curve25519"],
                            "supportedDerivationPathSchemes": ["cap26"]
                        },
                        "flags": ["main"],
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
