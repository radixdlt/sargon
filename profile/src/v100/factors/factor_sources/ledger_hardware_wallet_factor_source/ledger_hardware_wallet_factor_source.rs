use std::cell::RefCell;

use serde::{Deserialize, Serialize};

use crate::v100::factors::{
    factor_source_common::FactorSourceCommon, factor_source_id_from_hash::FactorSourceIDFromHash,
};

use super::ledger_hardware_wallet_hint::LedgerHardwareWalletHint;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LedgerHardwareWalletFactorSource {
    /// Unique and stable identifier of this factor source, stemming from the
    /// hash of a special child key of the HD root of the mnemonic,
    /// that is secured by the Ledger Hardware Wallet device.
    pub id: FactorSourceIDFromHash,

    /// Common properties shared between FactorSources of different kinds,
    /// describing its state, when added, and supported cryptographic parameters.
    ///
    /// Has interior mutability since we must be able to update the
    /// last used date.
    pub common: RefCell<FactorSourceCommon>,

    /// Properties describing a LedgerHardwareWalletFactorSource to help user disambiguate between it and another one.
    pub hint: LedgerHardwareWalletHint,
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
            common: RefCell::new(common),
            hint,
        }
    }
}

impl LedgerHardwareWalletFactorSource {
    pub fn placeholder() -> Self {
        Self::new(
            FactorSourceIDFromHash::placeholder_ledger(),
            FactorSourceCommon::placeholder(),
            LedgerHardwareWalletHint::placeholder(),
        )
    }
}

#[cfg(test)]
mod tests {
    use wallet_kit_common::json::assert_eq_after_json_roundtrip;

    use super::LedgerHardwareWalletFactorSource;

    #[test]
    fn json_roundtrip() {
        let model: LedgerHardwareWalletFactorSource =
            LedgerHardwareWalletFactorSource::placeholder();
        assert_eq_after_json_roundtrip(
            &model,
            r#"            
            {
                "id": {
                    "kind": "ledgerHQHardwareWallet",
                    "body": "3c986ebf9dcd9167a97036d3b2c997433e85e6cc4e4422ad89269dac7bfea240"
                },
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
                    "name": "Orange, scratched",
                    "model": "nanoS+"
                }
            }"#,
        );
    }
}
