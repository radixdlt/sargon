use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct PrivateHierarchicalDeterministicFactorSource {
    pub mnemonic_with_passphrase: MnemonicWithPassphrase,
    pub factor_source: DeviceFactorSource,
}

#[uniffi::export]
pub fn new_private_hd_factor_source(
    entropy: BagOfBytes,
    wallet_client_model: WalletClientModel,
) -> Result<PrivateHierarchicalDeterministicFactorSource> {
    entropy.try_into().map(|e| {
        PrivateHierarchicalDeterministicFactorSource::new_with_entropy(
            e,
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
                mnemonic_with_passphrase.clone()
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
            mnemonic_with_passphrase.clone(),
            wallet_client_model,
        );
        Self::new(mnemonic_with_passphrase, bdfs)
    }

    pub fn new_with_entropy(
        entropy: Exactly32Bytes,
        passphrase: BIP39Passphrase,
        wallet_client_model: WalletClientModel,
    ) -> Self {
        let mnemonic = Mnemonic::from_exactly32(entropy);
        let mnemonic_with_passphrase =
            MnemonicWithPassphrase::with_passphrase(mnemonic, passphrase);
        Self::new_with_mnemonic_with_passphrase(
            mnemonic_with_passphrase,
            wallet_client_model,
        )
    }

    pub fn generate_new(wallet_client_model: WalletClientModel) -> Self {
        Self::new_with_entropy(
            Exactly32Bytes::generate(),
            BIP39Passphrase::default(),
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
        let hd_private_key =
            self.mnemonic_with_passphrase.derive_private_key(path);
        let hd_factor_instance = HierarchicalDeterministicFactorInstance::new(
            self.factor_source.id,
            hd_private_key.public_key(),
        );
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
                FactorSourceIDFromHash::new_for_device(mwp),
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
    use crate::prelude::*;

    #[test]
    fn hash() {
        let n = 100;
        let set = (0..n)
            .map(|_| {
                PrivateHierarchicalDeterministicFactorSource::generate_new(
                    WalletClientModel::Unknown,
                )
            })
            .collect::<HashSet<_>>();
        assert_eq!(set.len(), n);
    }
}

#[cfg(test)]
mod uniffi_tests {
    use crate::prelude::*;

    #[test]
    fn new_uses_empty_bip39_passphrase() {
        let private = new_private_hd_factor_source(
            BagOfBytes::from(Vec::from_iter([0xff; 32])),
            WalletClientModel::Unknown,
        )
        .unwrap();
        assert_eq!(private.mnemonic_with_passphrase.passphrase.0, "");
    }
}
