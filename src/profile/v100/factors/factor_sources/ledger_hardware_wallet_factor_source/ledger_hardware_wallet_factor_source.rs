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

fn new_ledger_with_mwp(
    mwp: MnemonicWithPassphrase,
    hint: LedgerHardwareWalletHint,
    common: FactorSourceCommon,
) -> LedgerHardwareWalletFactorSource {
    let id = FactorSourceIDFromHash::new_for_ledger(&mwp);
    let mut source = LedgerHardwareWalletFactorSource::new(id, common, hint);
    source.common.last_used_on = Timestamp::sample();
    source.common.added_on = Timestamp::sample();
    source
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

impl HasSampleValues for LedgerHardwareWalletFactorSource {
    fn sample() -> Self {
        new_ledger_with_mwp(
            MnemonicWithPassphrase::sample_ledger(),
            LedgerHardwareWalletHint::sample(),
            FactorSourceCommon::new_bdfs(false),
        )
    }

    fn sample_other() -> Self {
        new_ledger_with_mwp(
            MnemonicWithPassphrase::sample_ledger_other(),
            LedgerHardwareWalletHint::sample_other(),
            FactorSourceCommon::new_olympia(),
        )
    }
}

impl TryFrom<FactorSource> for LedgerHardwareWalletFactorSource {
    type Error = CommonError;

    fn try_from(value: FactorSource) -> Result<Self> {
        match value {
            FactorSource::Ledger { value: factor } => Ok(factor),
            _ =>  Err(Self::Error::ExpectedLedgerHardwareWalletFactorSourceGotSomethingElse)
        }
    }
}
impl IsFactorSource for LedgerHardwareWalletFactorSource {
    fn kind() -> FactorSourceKind {
        FactorSourceKind::LedgerHQHardwareWallet
    }
}
impl BaseIsFactorSource for LedgerHardwareWalletFactorSource {
    fn common_properties(&self) -> FactorSourceCommon {
        self.common.clone()
    }

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
            LedgerHardwareWalletFactorSource::sample(),
            LedgerHardwareWalletFactorSource::sample()
        );
        assert_eq!(
            LedgerHardwareWalletFactorSource::sample_other(),
            LedgerHardwareWalletFactorSource::sample_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            LedgerHardwareWalletFactorSource::sample(),
            LedgerHardwareWalletFactorSource::sample_other()
        );
    }

    #[test]
    fn json_roundtrip() {
        let model = LedgerHardwareWalletFactorSource::sample();
        assert_eq_after_json_roundtrip(
            &model,
            r#"            
            {
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
            "#,
        );
    }

    #[test]
    fn from_factor_source() {
        let sut = LedgerHardwareWalletFactorSource::sample();
        let factor_source: FactorSource = sut.clone().into();
        assert_eq!(
            LedgerHardwareWalletFactorSource::try_from(factor_source),
            Ok(sut)
        );
    }

    #[test]
    fn from_factor_source_invalid_got_device() {
        let wrong = DeviceFactorSource::sample();
        let factor_source: FactorSource = wrong.clone().into();
        assert_eq!(
            LedgerHardwareWalletFactorSource::try_from(factor_source),
            Err(CommonError::ExpectedLedgerHardwareWalletFactorSourceGotSomethingElse)
        );
    }

    #[test]
    fn factor_source_id() {
        assert_eq!(
            LedgerHardwareWalletFactorSource::sample().factor_source_id(),
            LedgerHardwareWalletFactorSource::sample().id.into()
        );
    }

    #[test]
    fn factor_source_kind() {
        assert_eq!(
            LedgerHardwareWalletFactorSource::sample().factor_source_kind(),
            LedgerHardwareWalletFactorSource::sample().id.kind
        );
    }
}
