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
        mnemonic_with_passphrase: MnemonicWithPassphrase,
        wallet_client_model: WalletClientModel,
    ) -> Self {
        let id = FactorSourceIDFromHash::from_mnemonic_with_passphrase(
            FactorSourceKind::Device,
            mnemonic_with_passphrase.clone(),
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

    /// Checks if its Main Babylon Device Factor Source (BDFS).
    pub fn is_main_bdfs(&self) -> bool {
        self.common.is_main_bdfs()
    }
}

impl HasPlaceholder for DeviceFactorSource {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::placeholder_babylon()
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
        Self::placeholder_olympia()
    }
}

impl DeviceFactorSource {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_babylon() -> Self {
        Self::new(
            FactorSourceIDFromHash::placeholder(),
            FactorSourceCommon::placeholder_main_babylon(),
            DeviceFactorSourceHint::placeholder(),
        )
    }

    /// A placeholder used to facilitate unit tests.
    pub fn placeholder_olympia() -> Self {
        Self::new(
            FactorSourceIDFromHash::placeholder_other(),
            FactorSourceCommon::placeholder_olympia(),
            DeviceFactorSourceHint::placeholder(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(
            DeviceFactorSource::placeholder(),
            DeviceFactorSource::placeholder()
        );
        assert_eq!(
            DeviceFactorSource::placeholder_other(),
            DeviceFactorSource::placeholder_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            DeviceFactorSource::placeholder(),
            DeviceFactorSource::placeholder_other()
        );
    }

    #[test]
    fn main_babylon() {
        assert!(DeviceFactorSource::babylon(
            true,
            MnemonicWithPassphrase::placeholder(),
            WalletClientModel::placeholder()
        )
        .is_main_bdfs());
    }

    #[test]
    fn json() {
        let model = DeviceFactorSource::placeholder();
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
    fn static_kind() {
        assert_eq!(DeviceFactorSource::kind(), FactorSourceKind::Device);
    }

    #[test]
    fn from_factor_source_invalid_got_ledger() {
        let ledger = LedgerHardwareWalletFactorSource::placeholder();
        let factor_source: FactorSource = ledger.clone().into();
        assert_eq!(
            DeviceFactorSource::try_from(factor_source),
            Err(CommonError::ExpectedDeviceFactorSourceGotSomethingElse)
        );
    }

    #[test]
    fn placeholder_olympia_has_crypto_parameters_olympia() {
        assert_eq!(
            DeviceFactorSource::placeholder_olympia()
                .common
                .crypto_parameters,
            FactorSourceCryptoParameters::olympia()
        );
    }

    #[test]
    fn hint() {
        assert_eq!(
            DeviceFactorSource::placeholder().hint.mnemonic_word_count,
            BIP39WordCount::TwentyFour
        );
    }
}
