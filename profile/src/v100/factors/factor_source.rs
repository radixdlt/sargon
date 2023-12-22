use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

use enum_as_inner::EnumAsInner;

#[cfg(any(test, feature = "placeholder"))]
use crate::HasPlaceholder;

use super::{
    DeviceFactorSource, FactorSourceID, FactorSourceKind, IsFactorSource,
    LedgerHardwareWalletFactorSource,
};
#[derive(Serialize, Deserialize, Clone, EnumAsInner, Debug, PartialEq, Eq)]
#[serde(remote = "Self")]
pub enum FactorSource {
    #[serde(rename = "device")]
    Device(DeviceFactorSource),

    #[serde(rename = "ledgerHQHardwareWallet")]
    Ledger(LedgerHardwareWalletFactorSource),
}

impl IsFactorSource for FactorSource {
    fn factor_source_kind(&self) -> FactorSourceKind {
        match self {
            FactorSource::Device(fs) => fs.factor_source_kind(),
            FactorSource::Ledger(fs) => fs.factor_source_kind(),
        }
    }

    fn factor_source_id(&self) -> FactorSourceID {
        match self {
            FactorSource::Device(fs) => fs.factor_source_id(),
            FactorSource::Ledger(fs) => fs.factor_source_id(),
        }
    }
}

impl From<DeviceFactorSource> for FactorSource {
    fn from(value: DeviceFactorSource) -> Self {
        FactorSource::Device(value)
    }
}

impl From<LedgerHardwareWalletFactorSource> for FactorSource {
    fn from(value: LedgerHardwareWalletFactorSource) -> Self {
        FactorSource::Ledger(value)
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
        let mut state = serializer.serialize_struct("FactorSource", 2)?;
        let discriminator_key = "discriminator";
        match self {
            FactorSource::Device(device) => {
                let discriminant = "device";
                state.serialize_field(discriminator_key, discriminant)?;
                state.serialize_field(discriminant, device)?;
            }
            FactorSource::Ledger(ledger) => {
                let discriminant = "ledgerHQHardwareWallet";
                state.serialize_field(discriminator_key, discriminant)?;
                state.serialize_field(discriminant, ledger)?;
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
        Self::Device(DeviceFactorSource::placeholder_babylon())
    }

    pub fn placeholder_device_olympia() -> Self {
        Self::Device(DeviceFactorSource::placeholder_olympia())
    }

    pub fn placeholder_ledger() -> Self {
        Self::Ledger(LedgerHardwareWalletFactorSource::placeholder())
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
            FactorSource::Device(DeviceFactorSource::placeholder())
        );
    }

    #[test]
    fn into_from_ledger() {
        let factor_source: FactorSource = LedgerHardwareWalletFactorSource::placeholder().into();
        assert_eq!(
            factor_source,
            FactorSource::Ledger(LedgerHardwareWalletFactorSource::placeholder())
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
