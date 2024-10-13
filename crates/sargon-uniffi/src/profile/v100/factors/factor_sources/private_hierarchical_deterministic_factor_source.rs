use crate::prelude::*;
use sargon::BIP39Entropy as InternalBIP39Entropy;
use sargon::PrivateHierarchicalDeterministicFactorSource as InternalPrivateHierarchicalDeterministicFactorSource;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Record)]
pub struct PrivateHierarchicalDeterministicFactorSource {
    pub mnemonic_with_passphrase: MnemonicWithPassphrase,
    pub factor_source: DeviceFactorSource,
}

#[uniffi::export]
pub fn new_private_hd_factor_source_babylon(
    is_main: bool,
    entropy: NonEmptyMax32Bytes,
    host_info: &HostInfo,
) -> Result<PrivateHierarchicalDeterministicFactorSource> {
    InternalPrivateHierarchicalDeterministicFactorSource::new_babylon_with_entropy_bytes(
        is_main,
        entropy.into_internal(),
        &host_info.into_internal(),
    ).map_result()
}

#[uniffi::export]
pub fn new_private_hd_factor_source_babylon_from_mnemonic_with_passphrase(
    is_main: bool,
    mnemonic_with_passphrase: MnemonicWithPassphrase,
    host_info: &HostInfo,
) -> PrivateHierarchicalDeterministicFactorSource {
    InternalPrivateHierarchicalDeterministicFactorSource::new_babylon_with_mnemonic_with_passphrase(is_main, mnemonic_with_passphrase.into_internal(), &host_info.into_internal()).into()
}

#[uniffi::export]
pub fn new_private_hd_factor_source_olympia_from_mnemonic_with_passphrase(
    mnemonic_with_passphrase: MnemonicWithPassphrase,
    host_info: &HostInfo,
) -> PrivateHierarchicalDeterministicFactorSource {
    InternalPrivateHierarchicalDeterministicFactorSource::new_olympia_with_mnemonic_with_passphrase(mnemonic_with_passphrase.into_internal(), &host_info.into_internal()).into()
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
