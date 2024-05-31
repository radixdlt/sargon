use crate::prelude::*;

/// A factor source representing the host device which SargonOS runs on, typically
/// an iPhone or Android device.
///
/// This is the initial factor source of
/// all new Accounts and Personas. Users authenticate signing by authorizing
/// the host to access a mnemonic stored in secure storage on
/// the device.
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
#[display("{hint} {id}")]
pub struct DeviceFactorSource {
    /// Unique and stable identifier of this factor source, stemming from the
    /// hash of a special child key of the HD root of the mnemonic.
    pub id: FactorSourceIDFromHash,

    /// Common properties shared between FactorSources of different kinds,
    /// describing its state, when added, and supported cryptographic parameters.
    pub common: FactorSourceCommon,

    /// Properties describing a DeviceFactorSource to help user disambiguate between it and another one.
    pub hint: DeviceFactorSourceHint,
}

impl TryFrom<FactorSource> for DeviceFactorSource {
    type Error = CommonError;

    fn try_from(value: FactorSource) -> Result<Self> {
        value.into_device().map_err(|_| {
            Self::Error::ExpectedDeviceFactorSourceGotSomethingElse
        })
    }
}
impl IsFactorSource for DeviceFactorSource {
    fn kind() -> FactorSourceKind {
        FactorSourceKind::Device
    }
}
impl BaseIsFactorSource for DeviceFactorSource {
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
}

impl DeviceFactorSource {
    /// Instantiates a new `DeviceFactorSource`
    pub fn new(
        id: FactorSourceIDFromHash,
        common: FactorSourceCommon,
        hint: DeviceFactorSourceHint,
    ) -> Self {
        Self { id, common, hint }
    }

    pub fn babylon(
        is_main: bool,
        mnemonic_with_passphrase: &MnemonicWithPassphrase,
        device_info: &DeviceInfo,
    ) -> Self {
        let id = FactorSourceIDFromHash::from_mnemonic_with_passphrase(
            FactorSourceKind::Device,
            mnemonic_with_passphrase,
        );
        let hint = DeviceFactorSourceHint::with_info(
            device_info,
            mnemonic_with_passphrase.mnemonic.word_count,
        );
        Self::new(id, FactorSourceCommon::new_bdfs(is_main), hint)
    }

    pub fn olympia(
        mnemonic_with_passphrase: &MnemonicWithPassphrase,
        device_info: &DeviceInfo,
    ) -> Self {
        let id = FactorSourceIDFromHash::from_mnemonic_with_passphrase(
            FactorSourceKind::Device,
            mnemonic_with_passphrase,
        );
        let hint = DeviceFactorSourceHint::with_info(
            device_info,
            mnemonic_with_passphrase.mnemonic.word_count,
        );
        Self::new(id, FactorSourceCommon::new_olympia(), hint)
    }

    /// Checks if its Main Babylon Device Factor Source (BDFS).
    pub fn is_main_bdfs(&self) -> bool {
        self.common.is_main_bdfs()
    }
}

impl HasSampleValues for DeviceFactorSource {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::sample_babylon()
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::sample_olympia()
    }
}

impl DeviceFactorSource {
    /// A sample used to facilitate unit tests.
    pub fn sample_babylon() -> Self {
        let mut source = Self::babylon(
            true,
            &MnemonicWithPassphrase::sample_device(),
            &DeviceInfo::sample(),
        );
        source.common.last_used_on = Timestamp::sample();
        source.common.added_on = Timestamp::sample();
        source
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_babylon_other() -> Self {
        let mut source = Self::babylon(
            true,
            &MnemonicWithPassphrase::sample_device_other(),
            &DeviceInfo::sample_other(),
        );
        source.common.last_used_on = Timestamp::sample_other();
        source.common.added_on = Timestamp::sample_other();
        source
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_olympia() -> Self {
        let mut source = Self::olympia(
            &MnemonicWithPassphrase::sample_device_12_words(),
            &DeviceInfo::sample_other(),
        );
        source.common.last_used_on = Timestamp::sample();
        source.common.added_on = Timestamp::sample();
        source
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_olympia_other() -> Self {
        let mut source = Self::olympia(
            &MnemonicWithPassphrase::sample_device_12_words_other(),
            &DeviceInfo::sample(),
        );
        source.common.last_used_on = Timestamp::sample_other();
        source.common.added_on = Timestamp::sample_other();
        source
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DeviceFactorSource;

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
    fn main_babylon() {
        assert!(SUT::babylon(
            true,
            &MnemonicWithPassphrase::sample(),
            &DeviceInfo::sample()
        )
        .is_main_bdfs());
    }

    #[test]
    fn json() {
        let model = SUT::sample();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
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
                    "mnemonicWordCount": 24,
                    "model": "iPhone",
                    "name": "iPhone"
                },
                "id": {
                    "body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a",
                    "kind": "device"
                }
            }
            "#,
        );
    }

    #[test]
    fn factor_source_id() {
        let sut = SUT::sample();
        let factor_source_id: FactorSourceID = sut.clone().id.into();
        assert_eq!(factor_source_id, sut.factor_source_id());
    }

    #[test]
    fn from_factor_source() {
        let sut = SUT::sample();
        let factor_source: FactorSource = sut.clone().into();
        assert_eq!(SUT::try_from(factor_source), Ok(sut));
    }

    #[test]
    fn static_kind() {
        assert_eq!(SUT::kind(), FactorSourceKind::Device);
    }

    #[test]
    fn from_factor_source_invalid_got_ledger() {
        let ledger = LedgerHardwareWalletFactorSource::sample();
        let factor_source: FactorSource = ledger.clone().into();
        assert_eq!(
            SUT::try_from(factor_source),
            Err(CommonError::ExpectedDeviceFactorSourceGotSomethingElse)
        );
    }

    #[test]
    fn sample_olympia_has_crypto_parameters_olympia() {
        assert_eq!(
            SUT::sample_olympia().common.crypto_parameters,
            FactorSourceCryptoParameters::olympia()
        );
    }

    #[test]
    fn hint() {
        assert_eq!(
            SUT::sample().hint.mnemonic_word_count,
            BIP39WordCount::TwentyFour
        );
    }
}
