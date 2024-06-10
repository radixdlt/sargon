use crate::prelude::*;

/// A factor source representing a Mnemonic the user has to input every time
/// the use the factor source, since it is not saved on the device, it is said
/// to be "off device".
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
pub struct OffDeviceMnemonicFactorSource {
    /// Unique and stable identifier of this factor source, stemming from the
    /// hash of a special child key of the HD root of the mnemonic.
    pub id: FactorSourceIDFromHash,

    /// Common properties shared between FactorSources of different kinds,
    /// describing its state, when added, and supported cryptographic parameters.
    pub common: FactorSourceCommon,

    /// Properties describing a OffDeviceMnemonicFactorSource to help user
    /// disambiguate between it and another one.
    pub hint: OffDeviceFactorSourceHint,
}

impl From<OffDeviceMnemonicFactorSource> for FactorSource {
    fn from(value: OffDeviceMnemonicFactorSource) -> Self {
        FactorSource::OffDeviceMnemonic { value }
    }
}

fn new_off_device_with_mwp(
    mwp: MnemonicWithPassphrase,
    hint: OffDeviceFactorSourceHint,
) -> OffDeviceMnemonicFactorSource {
    let id = FactorSourceIDFromHash::new_for_off_device(&mwp);
    let mut source = OffDeviceMnemonicFactorSource::new(id, hint);
    source.common.last_used_on = Timestamp::sample();
    source.common.added_on = Timestamp::sample();
    source
}

impl OffDeviceMnemonicFactorSource {
    /// Instantiates a new `OffDeviceMnemonicFactorSource`
    pub fn new(
        id: FactorSourceIDFromHash,
        hint: OffDeviceFactorSourceHint,
    ) -> Self {
        Self {
            id,
            common: FactorSourceCommon::new_bdfs(false),
            hint,
        }
    }
}

impl HasSampleValues for OffDeviceMnemonicFactorSource {
    fn sample() -> Self {
        new_off_device_with_mwp(
            MnemonicWithPassphrase::sample_off_device(),
            OffDeviceFactorSourceHint::sample(),
        )
    }

    fn sample_other() -> Self {
        new_off_device_with_mwp(
            MnemonicWithPassphrase::sample_off_device_other(),
            OffDeviceFactorSourceHint::sample_other(),
        )
    }
}

impl TryFrom<FactorSource> for OffDeviceMnemonicFactorSource {
    type Error = CommonError;

    fn try_from(value: FactorSource) -> Result<Self> {
        value.clone().into_off_device_mnemonic().map_err(|_| {
            CommonError::InvalidFactorSourceKind {
                bad_value: value.factor_source_kind().to_string(),
            }
        })
    }
}
impl IsFactorSource for OffDeviceMnemonicFactorSource {
    fn kind() -> FactorSourceKind {
        FactorSourceKind::LedgerHQHardwareWallet
    }
}
impl BaseIsFactorSource for OffDeviceMnemonicFactorSource {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = OffDeviceMnemonicFactorSource;

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
            Err(CommonError::InvalidFactorSourceKind {
                bad_value: "device".to_owned()
            })
        );
    }
}
