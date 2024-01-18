use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct PrivateHierarchicalDeterministicFactorSource {
    pub mnemonic_with_passphrase: MnemonicWithPassphrase,
    pub factor_source: DeviceFactorSource,
}

#[uniffi::export]
pub fn new_private_hd_factor_source(
    entropy: Vec<u8>,
    wallet_client_model: WalletClientModel,
) -> Result<PrivateHierarchicalDeterministicFactorSource> {
    Hex32Bytes::from_vec(entropy).map(|e| {
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
        entropy: Hex32Bytes,
        passphrase: BIP39Passphrase,
        wallet_client_model: WalletClientModel,
    ) -> Self {
        let mnemonic = Mnemonic::from_hex32(entropy);
        let mnemonic_with_passphrase =
            MnemonicWithPassphrase::with_passphrase(mnemonic, passphrase);
        Self::new_with_mnemonic_with_passphrase(mnemonic_with_passphrase, wallet_client_model)
    }

    pub fn generate_new(wallet_client_model: WalletClientModel) -> Self {
        Self::new_with_entropy(
            Hex32Bytes::generate(),
            BIP39Passphrase::default(),
            wallet_client_model,
        )
    }
}

impl PrivateHierarchicalDeterministicFactorSource {
    pub fn derive_account_creation_factor_instance(
        &self,
        network_id: NetworkID,
        index: HDPathValue,
    ) -> HDFactorInstanceAccountCreation {
        let path = AccountPath::new(network_id, CAP26KeyKind::TransactionSigning, index);
        let hd_private_key = self.mnemonic_with_passphrase.derive_private_key(path);
        let hd_factor_instance = HierarchicalDeterministicFactorInstance::new(
            self.factor_source.id.clone(),
            hd_private_key.public_key(),
        );
        HDFactorInstanceAccountCreation::new(hd_factor_instance).unwrap()
    }
}

impl HasPlaceholder for PrivateHierarchicalDeterministicFactorSource {
    fn placeholder() -> Self {
        Self::new_with_mnemonic_with_passphrase(
            MnemonicWithPassphrase::placeholder(),
            WalletClientModel::Iphone,
        )
    }

    fn placeholder_other() -> Self {
        Self::new_with_mnemonic_with_passphrase(
            MnemonicWithPassphrase::placeholder_other(),
            WalletClientModel::Android,
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
        let set =
            (0..n)
                .into_iter()
                .map(|_| {
                    PrivateHierarchicalDeterministicFactorSource::generate_new(
                        WalletClientModel::Unknown,
                    )
                })
                .collect::<HashSet<_>>();
        assert_eq!(set.len(), n);
    }
}
