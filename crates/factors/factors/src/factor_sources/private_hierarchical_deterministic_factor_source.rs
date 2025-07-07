use host_info::prelude::HostInfo;

use crate::prelude::*;

#[derive(Zeroize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrivateHierarchicalDeterministicFactorSource {
    pub mnemonic_with_passphrase: MnemonicWithPassphrase,
    #[zeroize(skip)]
    pub factor_source: DeviceFactorSource,
}

impl PrivateHierarchicalDeterministicFactorSource {
    pub fn new(
        mnemonic_with_passphrase: MnemonicWithPassphrase,
        factor_source: DeviceFactorSource,
    ) -> Self {
        assert_eq!(
            factor_source.factor_source_id(),
            FactorSourceIDFromHash::from_mnemonic_with_passphrase(
                factor_source.factor_source_kind(),
                &mnemonic_with_passphrase
            )
            .into()
        );
        Self {
            mnemonic_with_passphrase,
            factor_source,
        }
    }

    pub fn new_olympia_with_mnemonic_with_passphrase(
        mnemonic_with_passphrase: MnemonicWithPassphrase,
        host_info: &HostInfo,
    ) -> Self {
        let device_factor_source =
            DeviceFactorSource::olympia(&mnemonic_with_passphrase, host_info);
        Self::new(mnemonic_with_passphrase, device_factor_source)
    }

    pub fn new_babylon_with_mnemonic_with_passphrase(
        mnemonic_with_passphrase: MnemonicWithPassphrase,
        host_info: &HostInfo,
    ) -> Self {
        let bdfs =
            DeviceFactorSource::babylon(&mnemonic_with_passphrase, host_info);
        Self::new(mnemonic_with_passphrase, bdfs)
    }

    pub fn new_babylon_with_entropy(
        entropy: BIP39Entropy,
        passphrase: BIP39Passphrase,
        host_info: &HostInfo,
    ) -> Self {
        let mnemonic = Mnemonic::from_entropy(entropy);
        let mnemonic_with_passphrase =
            MnemonicWithPassphrase::with_passphrase(mnemonic, passphrase);
        Self::new_babylon_with_mnemonic_with_passphrase(
            mnemonic_with_passphrase,
            host_info,
        )
    }

    pub fn new_babylon_with_entropy_bytes(
        entropy_bytes: NonEmptyMax32Bytes,
        host_info: &HostInfo,
    ) -> Result<Self> {
        let entropy = BIP39Entropy::try_from(entropy_bytes)?;
        Ok(Self::new_babylon_with_entropy(
            entropy,
            BIP39Passphrase::default(),
            host_info,
        ))
    }

    pub fn generate_new_babylon(host_info: &HostInfo) -> Self {
        let mnemonic = Mnemonic::generate_new();
        let mnemonic_with_passphrase = MnemonicWithPassphrase::new(mnemonic);
        Self::new_babylon_with_mnemonic_with_passphrase(
            mnemonic_with_passphrase,
            host_info,
        )
    }
}

impl PrivateHierarchicalDeterministicFactorSource {
    /// Should only be used for testing and sample values, for production code use
    /// `VirtualEntityCreatingInstanceProvider` powered by the `FactorInstancesProvider`
    pub fn _derive_entity_creation_factor_instance<T>(
        &self,
        network_id: NetworkID,
        index: HDPathComponent,
    ) -> HDFactorInstanceTransactionSigning<T>
    where
        T: IsEntityPath,
    {
        self._derive_entity_creation_factor_instances(network_id, [index])
            .into_iter()
            .last()
            .expect("Should have created one factor instance")
    }

    /// Should only be used for testing and sample values, for production code use
    /// `VirtualEntityCreatingInstanceProvider` powered by the `FactorInstancesProvider`
    pub fn _derive_entity_creation_factor_instances<T>(
        &self,
        network_id: NetworkID,
        indices: impl IntoIterator<Item = HDPathComponent>,
    ) -> Vec<HDFactorInstanceTransactionSigning<T>>
    where
        T: IsEntityPath,
    {
        let paths = indices
            .into_iter()
            .map(|i| Hardened::try_from(i).expect("only supports hardened"))
            .map(|i| T::new(network_id, CAP26KeyKind::TransactionSigning, i))
            .map(|p| p.derivation_path());

        self.mnemonic_with_passphrase
            ._derive_entity_creation_factor_instances(
                self.factor_source.id,
                paths,
            ).into_iter().map(|f| HDFactorInstanceTransactionSigning::<T>::try_from_factor_instance(f).unwrap()).collect()
    }
}

pub trait EntityCreatingFactorInstanceDeriving {
    fn _derive_entity_creation_factor_instances(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        derivation_paths: impl IntoIterator<Item = DerivationPath>,
    ) -> IndexSet<HierarchicalDeterministicFactorInstance>;
}

impl EntityCreatingFactorInstanceDeriving for MnemonicWithPassphrase {
    /// Should only be used for testing and sample values, for production code use
    /// `VirtualEntityCreatingInstanceProvider` powered by the `FactorInstancesProvider`
    fn _derive_entity_creation_factor_instances(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        derivation_paths: impl IntoIterator<Item = DerivationPath>,
    ) -> IndexSet<HierarchicalDeterministicFactorInstance> {
        let paths = derivation_paths.into_iter().collect::<IndexSet<_>>();
        let mut seed = self.to_seed();
        let instances = paths
            .into_iter()
            .map(|p| {
                let hd_private_key = seed.derive_private_key(&p);

                // hd_private_key.zeroize();
                HierarchicalDeterministicFactorInstance::new(
                    factor_source_id,
                    hd_private_key.public_key(),
                )
            })
            .collect::<IndexSet<_>>();

        seed.zeroize();
        instances
    }
}

impl HasSampleValues for PrivateHierarchicalDeterministicFactorSource {
    fn sample() -> Self {
        let mwp = MnemonicWithPassphrase::sample();
        Self::new(
            mwp.clone(),
            DeviceFactorSource::new(
                FactorSourceIDFromHash::new_for_device(&mwp),
                FactorSourceCommon::sample_main_babylon(),
                DeviceFactorSourceHint::sample_other(),
            ),
        )
    }

    fn sample_other() -> Self {
        let mwp = MnemonicWithPassphrase::sample_other();
        Self::new(
            mwp.clone(),
            DeviceFactorSource::new(
                FactorSourceIDFromHash::new_for_device(&mwp),
                FactorSourceCommon::sample_olympia(),
                DeviceFactorSourceHint::sample_other(),
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PrivateHierarchicalDeterministicFactorSource;

    #[test]
    fn hash() {
        let n = 100;
        let set = (0..n)
            .map(|_| SUT::generate_new_babylon(&HostInfo::sample()))
            .collect::<HashSet<_>>();
        assert_eq!(set.len(), n);
    }

    #[test]
    fn zeroize() {
        let mut sut = SUT::sample();
        sut.zeroize();
        assert_ne!(sut, SUT::sample());
    }

    #[test]
    fn new_olympia_with_mnemonic_with_passphrase() {
        let sut = SUT::new_olympia_with_mnemonic_with_passphrase(
            MnemonicWithPassphrase::sample(),
            &HostInfo::sample(),
        );
        assert_eq!(
            sut.factor_source.common.crypto_parameters,
            FactorSourceCryptoParameters::olympia()
        );
    }

    #[test]
    fn new_babylon_with_entropy_bytes() {
        let sut = SUT::new_babylon_with_entropy_bytes(
            NonEmptyMax32Bytes::from_bytes(&[0xff; 32]),
            &HostInfo::sample(),
        )
        .unwrap();
        assert_eq!(sut.mnemonic_with_passphrase.mnemonic.phrase(), "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote");
    }
}
