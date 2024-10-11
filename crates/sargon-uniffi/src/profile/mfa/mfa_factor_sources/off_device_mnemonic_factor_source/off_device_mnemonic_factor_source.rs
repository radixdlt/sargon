use crate::prelude::*;
use sargon::FactorSourceIDFromHash as InternalFactorSourceIDFromHash;
use sargon::OffDeviceMnemonicFactorSource as InternalOffDeviceMnemonicFactorSource;

/// A factor source representing a Mnemonic the user has to input every time
/// the use the factor source, since it is not saved on the device, it is said
/// to be "off device".
#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct OffDeviceMnemonicFactorSource {
    /// Unique and stable identifier of this factor source, stemming from the
    /// hash of a special child key of the HD root of the mnemonic.
    pub id: FactorSourceIDFromHash,

    /// Common properties shared between FactorSources of different kinds,
    /// describing its state, when added, and supported cryptographic parameters.
    pub common: FactorSourceCommon,

    /// Properties describing a OffDeviceMnemonicFactorSource to help user
    /// disambiguate between it and another one.
    pub hint: OffDeviceMnemonicHint,
}

impl From<InternalOffDeviceMnemonicFactorSource>
    for OffDeviceMnemonicFactorSource
{
    fn from(factor_source: InternalOffDeviceMnemonicFactorSource) -> Self {
        Self {
            id: factor_source.id.into(),
            common: factor_source.common.into(),
            hint: factor_source.hint.into(),
        }
    }
}

impl Into<InternalOffDeviceMnemonicFactorSource>
    for OffDeviceMnemonicFactorSource
{
    fn into(self) -> InternalOffDeviceMnemonicFactorSource {
        InternalOffDeviceMnemonicFactorSource {
            id: self.id.into(),
            common: self.common.into(),
            hint: self.hint.into(),
        }
    }
}

#[uniffi::export]
pub fn new_off_device_mnemonic_factor_source_sample(
) -> OffDeviceMnemonicFactorSource {
    InternalOffDeviceMnemonicFactorSource::sample().into()
}

#[uniffi::export]
pub fn new_off_device_mnemonic_factor_source_sample_other(
) -> OffDeviceMnemonicFactorSource {
    InternalOffDeviceMnemonicFactorSource::sample_other().into()
}

#[uniffi::export]
fn new_off_device_mnemonic_factor_source_from_mnemonic_with_passphrase(
    mwp: MnemonicWithPassphrase,
    hint: OffDeviceMnemonicHint,
) -> OffDeviceMnemonicFactorSource {
    let id = InternalFactorSourceIDFromHash::new_for_off_device(&mwp.into());
    InternalOffDeviceMnemonicFactorSource::new(id, hint.into()).into()
}
