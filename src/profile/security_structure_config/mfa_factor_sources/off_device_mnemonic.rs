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
    ///
    /// Has interior mutability since we must be able to update the
    /// last used date.
    pub common: FactorSourceCommon,

    /// Properties describing a OffDeviceMnemonicFactorSource to help user
    /// disambiguate between it and another one.
    pub hint: OffDeviceFactorSourceHint,
}

fn new_off_device_with_mwp(
    mwp: MnemonicWithPassphrase,
    hint: OffDeviceFactorSourceHint,
) -> OffDeviceMnemonicFactorSource {
    let id = FactorSourceIDFromHash::new_for_off_device(&mwp);
    let mut source = OffDeviceMnemonicFactorSource::new(
        id,
        FactorSourceCommon::new_bdfs(false),
        hint,
    );
    source.common.last_used_on = Timestamp::sample();
    source.common.added_on = Timestamp::sample();
    source
}

impl OffDeviceMnemonicFactorSource {
    /// Instantiates a new `OffDeviceMnemonicFactorSource`
    pub fn new(
        id: FactorSourceIDFromHash,
        common: FactorSourceCommon,
        hint: OffDeviceFactorSourceHint,
    ) -> Self {
        Self { id, common, hint }
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
}

/// Properties describing a DeviceFactorSource to help user disambiguate between
/// it and another one.
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
pub struct OffDeviceFactorSourceHint {
    pub display_name: DisplayName,
}

impl OffDeviceFactorSourceHint {
    pub fn new(display_name: DisplayName) -> Self {
        Self { display_name }
    }
}

impl HasSampleValues for OffDeviceFactorSourceHint {
    fn sample() -> Self {
        Self::new(DisplayName::new("Story about a horse").unwrap())
    }

    fn sample_other() -> Self {
        Self::new(DisplayName::new("Thrilled with a shark").unwrap())
    }
}
