use crate::prelude::*;

#[derive(
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Debug,
    derive_more::Display,
    uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
#[display("{hint} : {id}")]
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
    pub common: FactorSourceCommon,

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
        Self { id, common, hint }
    }
}

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
            FactorSourceIDFromHash::placeholder_ledger_other(),
            FactorSourceCommon::placeholder_other(),
            LedgerHardwareWalletHint::placeholder_other(),
        )
    }
}

impl TryFrom<FactorSource> for LedgerHardwareWalletFactorSource {
    type Error = CommonError;

    fn try_from(value: FactorSource) -> Result<Self> {
        match value {
            FactorSource::Ledger { value: factor } => Ok(factor),
            FactorSource::Device { value: _ } => {
                Err(Self::Error::ExpectedLedgerHardwareWalletFactorSourceGotSomethingElse)
            }
        }
    }
}
impl IsFactorSource for LedgerHardwareWalletFactorSource {
    fn kind() -> FactorSourceKind {
        FactorSourceKind::LedgerHQHardwareWallet
    }
}
impl BaseIsFactorSource for LedgerHardwareWalletFactorSource {
    fn factor_source_kind(&self) -> FactorSourceKind {
        self.id.kind
    }

    fn factor_source_id(&self) -> FactorSourceID {
        self.clone().id.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

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
            Err(CommonError::ExpectedLedgerHardwareWalletFactorSourceGotSomethingElse)
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
            LedgerHardwareWalletFactorSource::placeholder()
                .factor_source_kind(),
            LedgerHardwareWalletFactorSource::placeholder().id.kind
        );
    }
}
