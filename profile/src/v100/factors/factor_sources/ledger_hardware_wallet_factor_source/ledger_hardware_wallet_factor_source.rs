use std::{
    cell::RefCell,
    ops::Deref,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};

use crate::{
    v100::factors::{
        factor_source::FactorSource, factor_source_common::FactorSourceCommon,
        factor_source_id::FactorSourceID, factor_source_id_from_hash::FactorSourceIDFromHash,
        factor_source_kind::FactorSourceKind, is_factor_source::IsFactorSource,
    },
    CommonError,
};

use super::ledger_hardware_wallet_hint::LedgerHardwareWalletHint;

#[cfg(any(test, feature = "placeholder"))]
use crate::HasPlaceholder;

#[derive(Serialize, Deserialize, Debug, uniffi::Object)]
#[serde(rename_all = "camelCase")]
pub struct LedgerHardwareWalletFactorSource {
    /// Unique and stable identifier of this factor source, stemming from the
    /// hash of a special child key of the HD root of the mnemonic,
    /// that is secured by the Ledger Hardware Wallet device.
    id: FactorSourceIDFromHash,

    /// Common properties shared between FactorSources of different kinds,
    /// describing its state, when added, and supported cryptographic parameters.
    ///
    /// Has interior mutability since we must be able to update the
    /// last used date.
    common: Mutex<FactorSourceCommon>,

    /// Properties describing a LedgerHardwareWalletFactorSource to help user disambiguate between it and another one.
    hint: LedgerHardwareWalletHint,
}

impl Eq for LedgerHardwareWalletFactorSource {}
impl PartialEq for LedgerHardwareWalletFactorSource {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id() && self.common() == other.common() && self.hint() == other.hint()
    }
}

impl Clone for LedgerHardwareWalletFactorSource {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            common: Mutex::new(self.common()),
            hint: self.hint.clone(),
        }
    }
}

#[uniffi::export]
impl LedgerHardwareWalletFactorSource {
    pub fn get_id(&self) -> Arc<FactorSourceIDFromHash> {
        self.id().into()
    }

    pub fn get_common(&self) -> Arc<FactorSourceCommon> {
        self.common().into()
    }

    pub fn get_hint(&self) -> Arc<LedgerHardwareWalletHint> {
        self.hint().into()
    }
}

impl LedgerHardwareWalletFactorSource {
    pub fn id(&self) -> FactorSourceIDFromHash {
        self.id.clone()
    }

    pub fn common(&self) -> FactorSourceCommon {
        self.common
            .lock()
            .expect("`self.common` to not have been locked.")
            .clone()
    }

    pub fn hint(&self) -> LedgerHardwareWalletHint {
        self.hint.clone()
    }
}

impl LedgerHardwareWalletFactorSource {
    pub fn set_common(&self, new: FactorSourceCommon) {
        *self
            .common
            .lock()
            .expect("`self.common` to not have been locked.") = new
    }
}

impl LedgerHardwareWalletFactorSource {
    /// Instantiates a new `LedgerHardwareWalletFactorSource`
    pub fn new(
        id: FactorSourceIDFromHash,
        common: FactorSourceCommon,
        hint: LedgerHardwareWalletHint,
    ) -> Self {
        Self {
            id,
            common: Mutex::new(common),
            hint,
        }
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl HasPlaceholder for LedgerHardwareWalletFactorSource {
    fn placeholder() -> Self {
        Self::new(
            FactorSourceIDFromHash::placeholder_ledger(),
            FactorSourceCommon::placeholder(),
            LedgerHardwareWalletHint::placeholder(),
        )
    }

    fn placeholder_other() -> Self {
        Self::new(
            FactorSourceIDFromHash::placeholder_ledger(),
            FactorSourceCommon::placeholder_other(),
            LedgerHardwareWalletHint::placeholder_other(),
        )
    }
}

impl TryFrom<FactorSource> for LedgerHardwareWalletFactorSource {
    type Error = CommonError;

    fn try_from(value: FactorSource) -> Result<Self, Self::Error> {
        match value {
            FactorSource::Ledger { factor } => Ok(*factor),
            FactorSource::Device { factor } => {
                Err(Self::Error::ExpectedLedgerHardwareWalletFactorSourceGotSomethingElse)
            }
        }
    }
}

impl IsFactorSource for LedgerHardwareWalletFactorSource {
    fn factor_source_kind(&self) -> FactorSourceKind {
        self.id().kind().clone()
    }

    fn factor_source_id(&self) -> FactorSourceID {
        self.clone().id.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_eq_after_json_roundtrip, CommonError as Error, HasPlaceholder};

    use crate::v100::{DeviceFactorSource, FactorSource, FactorSourceCommon, IsFactorSource};

    use super::LedgerHardwareWalletFactorSource;

    #[test]
    fn equality() {
        assert_eq!(
            LedgerHardwareWalletFactorSource::placeholder(),
            LedgerHardwareWalletFactorSource::placeholder()
        );
        assert_eq!(
            LedgerHardwareWalletFactorSource::placeholder_other(),
            LedgerHardwareWalletFactorSource::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            LedgerHardwareWalletFactorSource::placeholder(),
            LedgerHardwareWalletFactorSource::placeholder_other()
        );
    }

    #[test]
    fn set_common() {
        let sut = LedgerHardwareWalletFactorSource::placeholder();
        assert_eq!(sut.common(), FactorSourceCommon::placeholder());
        sut.set_common(FactorSourceCommon::placeholder_other());
        assert_eq!(sut.common(), FactorSourceCommon::placeholder_other());
    }

    #[test]
    fn json_roundtrip() {
        let model = LedgerHardwareWalletFactorSource::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
            r#"            
            {
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
            "#,
        );
    }

    #[test]
    fn from_factor_source() {
        let sut = LedgerHardwareWalletFactorSource::placeholder();
        let factor_source: FactorSource = sut.clone().into();
        assert_eq!(
            LedgerHardwareWalletFactorSource::try_from(factor_source),
            Ok(sut)
        );
    }

    #[test]
    fn from_factor_source_invalid_got_device() {
        let wrong = DeviceFactorSource::placeholder();
        let factor_source: FactorSource = wrong.clone().into();
        assert_eq!(
            LedgerHardwareWalletFactorSource::try_from(factor_source),
            Err(Error::ExpectedLedgerHardwareWalletFactorSourceGotSomethingElse)
        );
    }

    #[test]
    fn factor_source_id() {
        assert_eq!(
            LedgerHardwareWalletFactorSource::placeholder().factor_source_id(),
            LedgerHardwareWalletFactorSource::placeholder().id.into()
        );
    }

    #[test]
    fn factor_source_kind() {
        assert_eq!(
            LedgerHardwareWalletFactorSource::placeholder().factor_source_kind(),
            LedgerHardwareWalletFactorSource::placeholder().id().kind()
        );
    }
}
