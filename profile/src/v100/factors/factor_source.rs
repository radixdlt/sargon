use std::sync::Arc;

use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

use enum_as_inner::EnumAsInner;

#[cfg(any(test, feature = "placeholder"))]
use crate::HasPlaceholder;

use super::{
    DeviceFactorSource, FactorSourceID, FactorSourceKind, IsFactorSource,
    LedgerHardwareWalletFactorSource,
};
#[derive(Clone, EnumAsInner, Debug, PartialEq, Eq, uniffi::Enum)]
pub enum FactorSource {
    Device {
        factor: Arc<DeviceFactorSource>,
    },

    Ledger {
        factor: Arc<LedgerHardwareWalletFactorSource>,
    },
}

impl IsFactorSource for FactorSource {
    fn factor_source_kind(&self) -> FactorSourceKind {
        match self {
            FactorSource::Device { factor } => factor.factor_source_kind(),
            FactorSource::Ledger { factor } => factor.factor_source_kind(),
        }
    }

    fn factor_source_id(&self) -> FactorSourceID {
        match self {
            FactorSource::Device { factor } => factor.factor_source_id(),
            FactorSource::Ledger { factor } => factor.factor_source_id(),
        }
    }
}

impl From<DeviceFactorSource> for FactorSource {
    fn from(value: DeviceFactorSource) -> Self {
        FactorSource::Device {
            factor: value.into(),
        }
    }
}

impl From<LedgerHardwareWalletFactorSource> for FactorSource {
    fn from(value: LedgerHardwareWalletFactorSource) -> Self {
        FactorSource::Ledger {
            factor: value.into(),
        }
    }
}

impl<'de> Deserialize<'de> for FactorSource {
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // https://github.com/serde-rs/serde/issues/1343#issuecomment-409698470
        #[derive(Deserialize, Serialize)]
        struct Wrapper {
            #[serde(rename = "discriminator")]
            _ignore: String,
            #[serde(flatten, with = "FactorSource")]
            inner: FactorSource,
        }
        Wrapper::deserialize(deserializer).map(|w| w.inner)
    }
}

impl Serialize for FactorSource {
    #[cfg(not(tarpaulin_include))] // false negative
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use std::ops::Deref;

        let mut state = serializer.serialize_struct("FactorSource", 2)?;
        let discriminator_key = "discriminator";
        match self {
            FactorSource::Device { factor: device } => {
                let discriminant = "device";
                state.serialize_field(discriminator_key, discriminant)?;
                state.serialize_field(discriminant, device.deref())?;
            }
            FactorSource::Ledger { factor: ledger } => {
                let discriminant = "ledgerHQHardwareWallet";
                state.serialize_field(discriminator_key, discriminant)?;
                state.serialize_field(discriminant, ledger.deref())?;
            }
        }
        state.end()
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl HasPlaceholder for FactorSource {
    fn placeholder() -> Self {
        Self::placeholder_device()
    }

    fn placeholder_other() -> Self {
        Self::placeholder_ledger()
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl FactorSource {
    pub fn placeholder_device() -> Self {
        Self::placeholder_device_babylon()
    }

    pub fn placeholder_device_babylon() -> Self {
        Self::Device {
            factor: DeviceFactorSource::placeholder_babylon().into(),
        }
    }

    pub fn placeholder_device_olympia() -> Self {
        Self::Device {
            factor: DeviceFactorSource::placeholder_olympia().into(),
        }
    }

    pub fn placeholder_ledger() -> Self {
        Self::Ledger {
            factor: LedgerHardwareWalletFactorSource::placeholder().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_eq_after_json_roundtrip, HasPlaceholder};

    use crate::v100::{
        DeviceFactorSource, FactorSourceKind, IsFactorSource, LedgerHardwareWalletFactorSource,
    };

    use super::FactorSource;

    #[test]
    fn equality() {
        assert_eq!(FactorSource::placeholder(), FactorSource::placeholder());
        assert_eq!(
            FactorSource::placeholder_other(),
            FactorSource::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            FactorSource::placeholder(),
            FactorSource::placeholder_other()
        );
    }

    #[test]
    fn factor_source_id_device() {
        assert_eq!(
            FactorSource::placeholder_device().factor_source_id(),
            DeviceFactorSource::placeholder().factor_source_id()
        );
    }

    #[test]
    fn factor_source_id_ledger() {
        assert_eq!(
            FactorSource::placeholder_ledger().factor_source_id(),
            LedgerHardwareWalletFactorSource::placeholder().factor_source_id()
        );
    }

    #[test]
    fn factor_source_kind_device() {
        assert_eq!(
            FactorSource::placeholder_device().factor_source_kind(),
            FactorSourceKind::Device
        );
    }

    #[test]
    fn factor_source_kind_ledger() {
        assert_eq!(
            FactorSource::placeholder_ledger().factor_source_kind(),
            FactorSourceKind::LedgerHQHardwareWallet
        );
    }

    #[test]
    fn into_from_device() {
        let factor_source: FactorSource = DeviceFactorSource::placeholder().into();
        assert_eq!(
            factor_source,
            FactorSource::Device {
                factor: DeviceFactorSource::placeholder().into()
            }
        );
    }

    #[test]
    fn into_from_ledger() {
        let factor_source: FactorSource = LedgerHardwareWalletFactorSource::placeholder().into();
        assert_eq!(
            factor_source,
            FactorSource::Ledger {
                factor: LedgerHardwareWalletFactorSource::placeholder().into()
            }
        );
    }

    #[test]
    fn json_roundtrip_device() {
        let model = FactorSource::placeholder_device();
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
        let model = FactorSource::placeholder_ledger();
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
