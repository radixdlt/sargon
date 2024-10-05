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
    pub hint: OffDeviceMnemonicHint,
}

#[uniffi::export]
pub fn new_off_device_mnemonic_factor_source_sample(
) -> OffDeviceMnemonicFactorSource {
    OffDeviceMnemonicFactorSource::sample()
}

#[uniffi::export]
pub fn new_off_device_mnemonic_factor_source_sample_other(
) -> OffDeviceMnemonicFactorSource {
    OffDeviceMnemonicFactorSource::sample_other()
}

#[uniffi::export]
fn new_off_device_mnemonic_factor_source_from_mnemonic_with_passphrase(
    mwp: MnemonicWithPassphrase,
    hint: OffDeviceMnemonicHint,
) -> OffDeviceMnemonicFactorSource {
    let id = FactorSourceIDFromHash::new_for_off_device(&mwp);
    OffDeviceMnemonicFactorSource::new(id, hint)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = OffDeviceMnemonicFactorSource;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_off_device_mnemonic_factor_source_sample(),
                new_off_device_mnemonic_factor_source_sample_other(),
                // duplicates should get removed
                new_off_device_mnemonic_factor_source_sample(),
                new_off_device_mnemonic_factor_source_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_new_off_device_mnemonic_factor_source_from_mnemonic_with_passphrase(
    ) {
        assert_eq!(
            new_off_device_mnemonic_factor_source_from_mnemonic_with_passphrase(
                MnemonicWithPassphrase::sample_off_device(),
                OffDeviceMnemonicHint::sample()
            )
            .factor_source_id(),
            SUT::sample().factor_source_id()
        );
    }
}
