use crate::prelude::*;

#[derive(Zeroize, Debug, Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct PrivateHierarchicalDeterministicFactorSource {
    pub mnemonic_with_passphrase: MnemonicWithPassphrase,
    #[zeroize(skip)]
    pub factor_source: DeviceFactorSource,
}

#[uniffi::export]
pub fn new_private_hd_factor_source(
    entropy: NonEmptyMax32Bytes,
    wallet_client_model: WalletClientModel,
) -> Result<PrivateHierarchicalDeterministicFactorSource> {
    BIP39Entropy::try_from(entropy).map(|entropy| {
        PrivateHierarchicalDeterministicFactorSource::new_with_entropy(
            entropy,
            BIP39Passphrase::default(),
            wallet_client_model,
        )
    })
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

    fn new_with_mnemonic_with_passphrase(
        mnemonic_with_passphrase: MnemonicWithPassphrase,
        wallet_client_model: WalletClientModel,
    ) -> Self {
        let bdfs = DeviceFactorSource::babylon(
            true,
            &mnemonic_with_passphrase,
            wallet_client_model,
        );
        Self::new(mnemonic_with_passphrase, bdfs)
    }

    pub fn new_with_entropy(
        entropy: BIP39Entropy,
        passphrase: BIP39Passphrase,
        wallet_client_model: WalletClientModel,
    ) -> Self {
        let mnemonic = Mnemonic::from_entropy(entropy);
        let mnemonic_with_passphrase =
            MnemonicWithPassphrase::with_passphrase(mnemonic, passphrase);
        Self::new_with_mnemonic_with_passphrase(
            mnemonic_with_passphrase,
            wallet_client_model,
        )
    }

    pub fn generate_new(wallet_client_model: WalletClientModel) -> Self {
        let mnemonic = Mnemonic::generate_new();
        let mnemonic_with_passphrase = MnemonicWithPassphrase::new(mnemonic);
        Self::new_with_mnemonic_with_passphrase(
            mnemonic_with_passphrase,
            wallet_client_model,
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
        let path = T::new(network_id, CAP26KeyKind::TransactionSigning, index);
        let mut seed = self.mnemonic_with_passphrase.to_seed();
        let hd_private_key = seed.derive_private_key(&path);
        let hd_factor_instance = HierarchicalDeterministicFactorInstance::new(
            self.factor_source.id,
            hd_private_key.public_key(),
        );
        seed.zeroize();
        // TODO: zeroize `hd_private_key` when `HierarchicalDeterministicPrivateKey` implement Zeroize...
        HDFactorInstanceTransactionSigning::new(hd_factor_instance).unwrap()
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
            .map(|_| SUT::generate_new(WalletClientModel::Unknown))
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

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = PrivateHierarchicalDeterministicFactorSource;

    #[test]
    fn new_uses_empty_bip39_passphrase() {
        let private: SUT = new_private_hd_factor_source(
            Entropy32Bytes::new([0xff; 32]).into(),
            WalletClientModel::Unknown,
        )
        .unwrap();
        assert_eq!(private.mnemonic_with_passphrase.passphrase.0, "");
    }
}
