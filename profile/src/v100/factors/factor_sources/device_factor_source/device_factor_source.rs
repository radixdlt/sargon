use std::cell::RefCell;

use hierarchical_deterministic::derivation::mnemonic_with_passphrase::MnemonicWithPassphrase;
use serde::{Deserialize, Serialize};

use crate::v100::factors::{
    factor_source::FactorSource, factor_source_common::FactorSourceCommon,
    factor_source_id::FactorSourceID, factor_source_id_from_hash::FactorSourceIDFromHash,
    factor_source_kind::FactorSourceKind, is_factor_source::IsFactorSource,
};

use super::device_factor_source_hint::DeviceFactorSourceHint;

/// A factor source representing the device that the Radix Wallet is running on
/// typically an iPhone or Android device. This is the initial factor source of
/// all new Accounts and Personas an users authenticate signing by authorizing
/// the client (Wallet App) to access a mnemonic stored in secure storage on
/// the device.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeviceFactorSource {
    /// Unique and stable identifier of this factor source, stemming from the
    /// hash of a special child key of the HD root of the mnemonic.
    pub id: FactorSourceIDFromHash,

    /// Common properties shared between FactorSources of different kinds,
    /// describing its state, when added, and supported cryptographic parameters.
    ///
    /// Has interior mutability since we must be able to update the
    /// last used date.
    pub common: RefCell<FactorSourceCommon>,

    /// Properties describing a DeviceFactorSource to help user disambiguate between it and another one.
    pub hint: DeviceFactorSourceHint,
}

impl TryFrom<FactorSource> for DeviceFactorSource {
    type Error = wallet_kit_common::error::common_error::CommonError;

    fn try_from(value: FactorSource) -> Result<Self, Self::Error> {
        value
            .into_device()
            .map_err(|_| Self::Error::ExpectedDeviceFactorSourceGotSomethingElse)
    }
}

impl IsFactorSource for DeviceFactorSource {
    fn factor_source_kind(&self) -> FactorSourceKind {
        self.id.kind
    }

    fn factor_source_id(&self) -> FactorSourceID {
        self.clone().id.into()
    }
}

impl DeviceFactorSource {
    /// Instantiates a new `DeviceFactorSource`
    pub fn new(
        id: FactorSourceIDFromHash,
        common: FactorSourceCommon,
        hint: DeviceFactorSourceHint,
    ) -> Self {
        Self {
            id,
            common: RefCell::new(common),
            hint,
        }
    }

    pub fn babylon(
        is_main: bool,
        mnemonic_with_passphrase: MnemonicWithPassphrase,
        device_model: &str,
    ) -> Self {
        let id = FactorSourceIDFromHash::from_mnemonic_with_passphrase(
            FactorSourceKind::Device,
            mnemonic_with_passphrase.clone(),
        );

        Self::new(
            id,
            FactorSourceCommon::new_bdfs(is_main),
            DeviceFactorSourceHint::unknown_model_and_name_with_word_count(
                mnemonic_with_passphrase.mnemonic.word_count,
                device_model,
            ),
        )
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl DeviceFactorSource {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder() -> Self {
        Self::new(
            FactorSourceIDFromHash::placeholder(),
            FactorSourceCommon::placeholder(),
            DeviceFactorSourceHint::placeholder(),
        )
    }
}

#[cfg(test)]
mod tests {
    use wallet_kit_common::json::assert_eq_after_json_roundtrip;

    use crate::v100::factors::{
        factor_source_id::FactorSourceID, is_factor_source::IsFactorSource, factor_source::FactorSource, factor_sources::ledger_hardware_wallet_factor_source::ledger_hardware_wallet_factor_source::LedgerHardwareWalletFactorSource,
    };
use wallet_kit_common::error::common_error::CommonError as Error;
    use super::DeviceFactorSource;

    #[test]
    fn json() {
        let model = DeviceFactorSource::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "common": {
                    "addedOn": "2023-09-11T16:05:56",
                    "cryptoParameters": {
                        "supportedCurves": ["curve25519"],
                        "supportedDerivationPathSchemes": ["cap26"]
                    },
                    "flags": ["main"],
                    "lastUsedOn": "2023-09-11T16:05:56"
                },
                "hint": {
                    "mnemonicWordCount": 24,
                    "model": "iPhone",
                    "name": "Unknown Name"
                },
                "id": {
                    "body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240",
                    "kind": "device"
                }
            }
            "#,
        );
    }

    #[test]
    fn factor_source_id() {
        let sut = DeviceFactorSource::placeholder();
        let factor_source_id: FactorSourceID = sut.clone().id.into();
        assert_eq!(factor_source_id, sut.factor_source_id());
    }

    #[test]
    fn from_factor_source() {
        let sut = DeviceFactorSource::placeholder();
        let factor_source: FactorSource = sut.clone().into();
        assert_eq!(DeviceFactorSource::try_from(factor_source), Ok(sut));
    }

    #[test]
    fn from_factor_source_invalid_got_ledger() {
        let ledger = LedgerHardwareWalletFactorSource::placeholder();
        let factor_source: FactorSource = ledger.clone().into();
        assert_eq!(
            DeviceFactorSource::try_from(factor_source),
            Err(Error::ExpectedDeviceFactorSourceGotSomethingElse)
        );
    }
}
