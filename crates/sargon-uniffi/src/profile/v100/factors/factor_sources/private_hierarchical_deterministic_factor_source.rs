use crate::prelude::*;
use sargon::PrivateHierarchicalDeterministicFactorSource as InternalPrivateHierarchicalDeterministicFactorSource;
use sargon::BIP39Entropy as InternalBIP39Entropy;

#[derive(Zeroize,  Clone, PartialEq, Eq, Hash,  uniffi::Record)]
pub struct PrivateHierarchicalDeterministicFactorSource {
    pub mnemonic_with_passphrase: MnemonicWithPassphrase,
    #[zeroize(skip)]
    pub factor_source: DeviceFactorSource,
}

impl From<InternalPrivateHierarchicalDeterministicFactorSource> for PrivateHierarchicalDeterministicFactorSource {
    fn from(value: InternalPrivateHierarchicalDeterministicFactorSource) -> Self {
        Self {
            mnemonic_with_passphrase: value.mnemonic_with_passphrase.into(),
            factor_source: value.factor_source.into(),
        }
    }
}

impl Into<InternalPrivateHierarchicalDeterministicFactorSource> for PrivateHierarchicalDeterministicFactorSource {
    fn into(self) -> InternalPrivateHierarchicalDeterministicFactorSource {
        InternalPrivateHierarchicalDeterministicFactorSource {
            mnemonic_with_passphrase: self.mnemonic_with_passphrase.into(),
            factor_source: self.factor_source.into(),
        }
    }
}

#[uniffi::export]
pub fn new_private_hd_factor_source_babylon(
    is_main: bool,
    entropy: NonEmptyMax32Bytes,
    host_info: &HostInfo,
) -> Result<PrivateHierarchicalDeterministicFactorSource> {
    InternalPrivateHierarchicalDeterministicFactorSource::new_babylon_with_entropy_bytes(
        is_main, 
        entropy.into(), 
        host_info.into()
    ).map_result()
}

#[uniffi::export]
pub fn new_private_hd_factor_source_babylon_from_mnemonic_with_passphrase(
    is_main: bool,
    mnemonic_with_passphrase: MnemonicWithPassphrase,
    host_info: &HostInfo,
) -> PrivateHierarchicalDeterministicFactorSource {
    InternalPrivateHierarchicalDeterministicFactorSource::new_babylon_with_mnemonic_with_passphrase(is_main, mnemonic_with_passphrase.into(), host_info.into()).into()
}

#[uniffi::export]
pub fn new_private_hd_factor_source_olympia_from_mnemonic_with_passphrase(
    mnemonic_with_passphrase: MnemonicWithPassphrase,
    host_info: &HostInfo,
) -> PrivateHierarchicalDeterministicFactorSource {
    InternalPrivateHierarchicalDeterministicFactorSource::new_olympia_with_mnemonic_with_passphrase(mnemonic_with_passphrase.into(), &host_info.into()).into()
}

#[uniffi::export]
pub fn new_private_hd_factor_source_sample(
) -> PrivateHierarchicalDeterministicFactorSource {
    InternalPrivateHierarchicalDeterministicFactorSource::sample().into()
}

#[uniffi::export]
pub fn new_private_hd_factor_source_sample_other(
) -> PrivateHierarchicalDeterministicFactorSource {
    InternalPrivateHierarchicalDeterministicFactorSource::sample_other().into()
}

