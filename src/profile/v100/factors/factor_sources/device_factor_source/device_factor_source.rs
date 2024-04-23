use crate::prelude::*;

/// A factor source representing the device that the Radix Wallet is running on
/// typically an iPhone or Android device. This is the initial factor source of
/// all new Accounts and Personas an users authenticate signing by authorizing
/// the client (Wallet App) to access a mnemonic stored in secure storage on
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
    ///
    /// Has interior mutability since we must be able to update the
    /// last used date.
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
        wallet_client_model: WalletClientModel,
    ) -> Self {
        let id = FactorSourceIDFromHash::from_mnemonic_with_passphrase(
            FactorSourceKind::Device,
            mnemonic_with_passphrase,
        );

        Self::new(
            id,
            FactorSourceCommon::new_bdfs(is_main),
            DeviceFactorSourceHint::unknown_model_of_client(
                mnemonic_with_passphrase.mnemonic.word_count,
                wallet_client_model,
            ),
        )
    }

    pub fn olympia(
        mnemonic_with_passphrase: &MnemonicWithPassphrase,
        wallet_client_model: WalletClientModel,
    ) -> Self {
        let id = FactorSourceIDFromHash::from_mnemonic_with_passphrase(
            FactorSourceKind::Device,
            mnemonic_with_passphrase,
        );

        Self::new(
            id,
            FactorSourceCommon::new_olympia(),
            DeviceFactorSourceHint::unknown_model_of_client(
                mnemonic_with_passphrase.mnemonic.word_count,
                wallet_client_model,
            ),
        )
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
        Self::new(
            FactorSourceIDFromHash::sample(),
            FactorSourceCommon::sample_main_babylon(),
            DeviceFactorSourceHint::sample(),
        )
    }

    /// A sample used to facilitate unit tests.
    pub fn sample_olympia() -> Self {
        Self::new(
            FactorSourceIDFromHash::sample_other(),
            FactorSourceCommon::sample_olympia(),
            DeviceFactorSourceHint::sample(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(DeviceFactorSource::sample(), DeviceFactorSource::sample());
        assert_eq!(
            DeviceFactorSource::sample_other(),
            DeviceFactorSource::sample_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            DeviceFactorSource::sample(),
            DeviceFactorSource::sample_other()
        );
    }

    #[test]
    fn main_babylon() {
        assert!(DeviceFactorSource::babylon(
            true,
            &MnemonicWithPassphrase::sample(),
            WalletClientModel::sample()
        )
        .is_main_bdfs());
    }

    #[test]
    fn json() {
        let model = DeviceFactorSource::sample();
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
        let sut = DeviceFactorSource::sample();
        let factor_source_id: FactorSourceID = sut.clone().id.into();
        assert_eq!(factor_source_id, sut.factor_source_id());
    }

    #[test]
    fn from_factor_source() {
        let sut = DeviceFactorSource::sample();
        let factor_source: FactorSource = sut.clone().into();
        assert_eq!(DeviceFactorSource::try_from(factor_source), Ok(sut));
    }

    #[test]
    fn static_kind() {
        assert_eq!(DeviceFactorSource::kind(), FactorSourceKind::Device);
    }

    #[test]
    fn from_factor_source_invalid_got_ledger() {
        let ledger = LedgerHardwareWalletFactorSource::sample();
        let factor_source: FactorSource = ledger.clone().into();
        assert_eq!(
            DeviceFactorSource::try_from(factor_source),
            Err(CommonError::ExpectedDeviceFactorSourceGotSomethingElse)
        );
    }

    #[test]
    fn sample_olympia_has_crypto_parameters_olympia() {
        assert_eq!(
            DeviceFactorSource::sample_olympia()
                .common
                .crypto_parameters,
            FactorSourceCryptoParameters::olympia()
        );
    }

    #[test]
    fn hint() {
        assert_eq!(
            DeviceFactorSource::sample().hint.mnemonic_word_count,
            BIP39WordCount::TwentyFour
        );
    }
}
