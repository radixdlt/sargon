use crate::prelude::*;
use sargon::PrivateHierarchicalDeterministicFactorSource as InternalPrivateHierarchicalDeterministicFactorSource;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct PrivateHierarchicalDeterministicFactorSource {
    pub mnemonic_with_passphrase: MnemonicWithPassphrase,
    pub factor_source: DeviceFactorSource,
}

#[uniffi::export]
pub fn new_private_hd_factor_source_babylon(
    entropy: NonEmptyMax32Bytes,
    host_info: &HostInfo,
) -> Result<PrivateHierarchicalDeterministicFactorSource> {
    InternalPrivateHierarchicalDeterministicFactorSource::new_babylon_with_entropy_bytes(
        entropy.into_internal(),
        &host_info.into_internal(),
    ).into_result()
}

#[uniffi::export]
pub fn new_private_hd_factor_source_babylon_from_mnemonic_with_passphrase(
    mnemonic_with_passphrase: MnemonicWithPassphrase,
    host_info: &HostInfo,
) -> PrivateHierarchicalDeterministicFactorSource {
    InternalPrivateHierarchicalDeterministicFactorSource::new_babylon_with_mnemonic_with_passphrase(mnemonic_with_passphrase.into_internal(), &host_info.into_internal()).into()
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
