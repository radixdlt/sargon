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
    pub common: FactorSourceCommon,

    /// Properties describing a LedgerHardwareWalletFactorSource to help user disambiguate between it and another one.
    pub hint: LedgerHardwareWalletHint,
}

/// # Safety
/// Rust memory safe, but marked "unsafe" since this ctor is only used for tests (and shouldn't be
/// used by production code). A real Ledger device won't be initialized with a `MnemonicWithPassphrase`,
/// but with `Exactly32Bytes` detailing the device's ID.
unsafe fn new_ledger_with_mwp(
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
        unsafe {
            new_ledger_with_mwp(
                MnemonicWithPassphrase::sample_ledger(),
                LedgerHardwareWalletHint::sample(),
                FactorSourceCommon::new_bdfs(),
            )
        }
    }

    fn sample_other() -> Self {
        unsafe {
            new_ledger_with_mwp(
                MnemonicWithPassphrase::sample_ledger_other(),
                LedgerHardwareWalletHint::sample_other(),
                FactorSourceCommon::new_olympia(),
            )
        }
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
impl BaseBaseIsFactorSource for LedgerHardwareWalletFactorSource {
    fn common_properties(&self) -> FactorSourceCommon {
        self.common.clone()
    }

    fn factor_source_kind(&self) -> FactorSourceKind {
        self.id.kind
    }

    fn factor_source_id(&self) -> FactorSourceID {
        self.clone().id.into()
    }

    fn set_common_properties(&mut self, updated: FactorSourceCommon) {
        self.common = updated
    }

    fn name(&self) -> String {
        self.hint.label.clone()
    }

    fn set_name(&mut self, updated: String) {
        self.hint.label = updated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = LedgerHardwareWalletFactorSource;

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
    fn kind() {
        assert_eq!(SUT::kind(), FactorSourceKind::LedgerHQHardwareWallet);
    }

    #[test]
    fn json_roundtrip() {
        let model = SUT::sample();
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
        let sut = SUT::sample();
        let factor_source: FactorSource = sut.clone().into();
        assert_eq!(SUT::try_from(factor_source), Ok(sut));
    }

    #[test]
    fn from_factor_source_invalid_got_device() {
        let wrong = DeviceFactorSource::sample();
        let factor_source: FactorSource = wrong.clone().into();
        assert_eq!(
            SUT::try_from(factor_source),
            Err(CommonError::ExpectedLedgerHardwareWalletFactorSourceGotSomethingElse)
        );
    }

    #[test]
    fn factor_source_id() {
        assert_eq!(SUT::sample().factor_source_id(), SUT::sample().id.into());
    }

    #[test]
    fn factor_source_kind() {
        assert_eq!(SUT::sample().factor_source_kind(), SUT::sample().id.kind);
    }

    #[test]
    fn name() {
        let mut sut = SUT::sample();
        assert_eq!(sut.name(), "Orange, scratched");
        sut.set_name("Old cracked".to_string());
        assert_eq!(sut.name(), "Old cracked");
    }
}
