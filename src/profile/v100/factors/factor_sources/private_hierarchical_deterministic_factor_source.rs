use crate::prelude::*;

#[derive(Zeroize, Debug, Clone, PartialEq, Eq, Hash, uniffi::Record)]
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
        device_info: &DeviceInfo,
    ) -> Self {
        let device_factor_source =
            DeviceFactorSource::olympia(&mnemonic_with_passphrase, device_info);
        Self::new(mnemonic_with_passphrase, device_factor_source)
    }

    pub fn new_babylon_with_mnemonic_with_passphrase(
        is_main: bool,
        mnemonic_with_passphrase: MnemonicWithPassphrase,
        device_info: &DeviceInfo,
    ) -> Self {
        let bdfs = DeviceFactorSource::babylon(
            is_main,
            &mnemonic_with_passphrase,
            device_info,
        );
        Self::new(mnemonic_with_passphrase, bdfs)
    }

    pub fn new_babylon_with_entropy(
        is_main: bool,
        entropy: BIP39Entropy,
        passphrase: BIP39Passphrase,
        device_info: &DeviceInfo,
    ) -> Self {
        let mnemonic = Mnemonic::from_entropy(entropy);
        let mnemonic_with_passphrase =
            MnemonicWithPassphrase::with_passphrase(mnemonic, passphrase);
        Self::new_babylon_with_mnemonic_with_passphrase(
            is_main,
            mnemonic_with_passphrase,
            device_info,
        )
    }

    pub fn generate_new_babylon(
        is_main: bool,
        device_info: &DeviceInfo,
    ) -> Self {
        let mnemonic = Mnemonic::generate_new();
        let mnemonic_with_passphrase = MnemonicWithPassphrase::new(mnemonic);
        Self::new_babylon_with_mnemonic_with_passphrase(
            is_main,
            mnemonic_with_passphrase,
            device_info,
        )
    }
}

impl PrivateHierarchicalDeterministicFactorSource {
    pub fn derive_entity_creation_factor_instance<T>(
        &self,
        network_id: NetworkID,
        index: HDPathValue,
    ) -> HDFactorInstanceTransactionSigning<T>
    where
        T: IsEntityPath + Clone,
    {
        self.derive_entity_creation_factor_instances(network_id, [index])
            .into_iter()
            .last()
            .expect("Should have created one factor instance")
    }

    pub fn derive_entity_creation_factor_instances<T>(
        &self,
        network_id: NetworkID,
        indices: impl IntoIterator<Item = HDPathValue>,
    ) -> Vec<HDFactorInstanceTransactionSigning<T>>
    where
        T: IsEntityPath + Clone,
    {
        let paths = indices
            .into_iter()
            .map(|i| T::new(network_id, CAP26KeyKind::TransactionSigning, i));

        let mut seed = self.mnemonic_with_passphrase.to_seed();
        let instances = paths
            .map(|p| {
                let hd_private_key = seed.derive_private_key(&p);
                let hd_factor_instance =
                    HierarchicalDeterministicFactorInstance::new(
                        self.factor_source.id,
                        hd_private_key.public_key(),
                    );
                // TODO: zeroize `hd_private_key` when `HierarchicalDeterministicPrivateKey` implement Zeroize...
                HDFactorInstanceTransactionSigning::new(hd_factor_instance)
                    .unwrap()
            })
            .collect_vec();

        seed.zeroize();
        instances
    }
}

impl HasSampleValues for PrivateHierarchicalDeterministicFactorSource {
    fn sample() -> Self {
        Self::new(
            MnemonicWithPassphrase::sample(),
            DeviceFactorSource::sample_babylon(),
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

impl SafeToLog for PrivateHierarchicalDeterministicFactorSource {
    /// Logs the word count and FactorSourceID.
    fn non_sensitive(&self) -> impl std::fmt::Debug {
        format!(
            "{} {}",
            self.factor_source.hint.mnemonic_word_count, self.factor_source.id
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
            .map(|_| SUT::generate_new_babylon(true, &DeviceInfo::sample()))
            .collect::<HashSet<_>>();
        assert_eq!(set.len(), n);
    }

    #[test]
    fn zeroize() {
        let mut sut = SUT::sample();
        sut.zeroize();
        assert_ne!(sut, SUT::sample());
    }
}
