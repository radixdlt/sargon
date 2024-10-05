use crate::prelude::*;
use sargon::PrivateHierarchicalDeterministicFactorSource as InternalPrivateHierarchicalDeterministicFactorSource;

#[derive(Zeroize, Debug, Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct PrivateHierarchicalDeterministicFactorSource {
    pub mnemonic_with_passphrase: MnemonicWithPassphrase,
    #[zeroize(skip)]
    pub factor_source: DeviceFactorSource,
}

impl From<InternalPrivateHierarchicalDeterministicFactorSource> for PrivateHierarchicalDeterministicFactorSource {
    fn from(value: InternalPrivateHierarchicalDeterministicFactorSource) -> Self {
        unimplemented!()
    }
}

impl Into<InternalPrivateHierarchicalDeterministicFactorSource> for PrivateHierarchicalDeterministicFactorSource {
    fn into(self) -> InternalPrivateHierarchicalDeterministicFactorSource {
        unimplemented!()
    }
}

#[uniffi::export]
pub fn new_private_hd_factor_source_babylon(
    is_main: bool,
    entropy: NonEmptyMax32Bytes,
    host_info: &HostInfo,
) -> Result<PrivateHierarchicalDeterministicFactorSource> {
    BIP39Entropy::try_from(entropy).map(|entropy| {
        PrivateHierarchicalDeterministicFactorSource::new_babylon_with_entropy(
            is_main,
            entropy,
            BIP39Passphrase::default(),
            host_info,
        )
    })
}

#[uniffi::export]
pub fn new_private_hd_factor_source_babylon_from_mnemonic_with_passphrase(
    is_main: bool,
    mnemonic_with_passphrase: MnemonicWithPassphrase,
    host_info: &HostInfo,
) -> PrivateHierarchicalDeterministicFactorSource {
    PrivateHierarchicalDeterministicFactorSource::new_babylon_with_mnemonic_with_passphrase(is_main, mnemonic_with_passphrase, host_info)
}

#[uniffi::export]
pub fn new_private_hd_factor_source_olympia_from_mnemonic_with_passphrase(
    mnemonic_with_passphrase: MnemonicWithPassphrase,
    host_info: &HostInfo,
) -> PrivateHierarchicalDeterministicFactorSource {
    PrivateHierarchicalDeterministicFactorSource::new_olympia_with_mnemonic_with_passphrase(mnemonic_with_passphrase, host_info)
}

#[uniffi::export]
pub fn new_private_hd_factor_source_sample(
) -> PrivateHierarchicalDeterministicFactorSource {
    PrivateHierarchicalDeterministicFactorSource::sample()
}

#[uniffi::export]
pub fn new_private_hd_factor_source_sample_other(
) -> PrivateHierarchicalDeterministicFactorSource {
    PrivateHierarchicalDeterministicFactorSource::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_private_hd_factor_source_sample(),
                new_private_hd_factor_source_sample_other(),
                // duplicates should get removed
                new_private_hd_factor_source_sample(),
                new_private_hd_factor_source_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PrivateHierarchicalDeterministicFactorSource;

    #[test]
    fn new_uses_empty_bip39_passphrase() {
        let private: SUT = new_private_hd_factor_source_babylon(
            true,
            Entropy32Bytes::new([0xff; 32]).into(),
            &HostInfo::sample(),
        )
        .unwrap();
        assert_eq!(private.mnemonic_with_passphrase.passphrase.0, "");
    }

    #[test]
    fn test_new_private_hd_factor_source_babylon_from_mnemonic_with_passphrase()
    {
        let sut =
            new_private_hd_factor_source_babylon_from_mnemonic_with_passphrase(
                true,
                MnemonicWithPassphrase::sample(),
                &HostInfo::sample(),
            );
        assert!(&sut.factor_source.supports_babylon());
        assert!(!&sut.factor_source.supports_olympia());
    }

    #[test]
    fn test_new_private_hd_factor_source_babylon_from_mnemonic_with_passphrase_is_main_true(
    ) {
        let sut =
            new_private_hd_factor_source_babylon_from_mnemonic_with_passphrase(
                true,
                MnemonicWithPassphrase::sample(),
                &HostInfo::sample(),
            );
        assert!(sut.factor_source.is_main_bdfs());
    }

    #[test]
    fn test_new_private_hd_factor_source_olympia_from_mnemonic_with_passphrase()
    {
        let sut =
            new_private_hd_factor_source_olympia_from_mnemonic_with_passphrase(
                MnemonicWithPassphrase::sample(),
                &HostInfo::sample(),
            );
        assert!(&sut.factor_source.supports_olympia());
        assert!(!&sut.factor_source.supports_babylon());
    }
}
