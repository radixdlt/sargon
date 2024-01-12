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
    Hex32Bytes::from_vec(entropy).map(
        |e| PrivateHierarchicalDeterministicFactorSource::new_with_entropy(e, wallet_client_model)
    )
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

    fn new_with_mnemonic(mnemonic: Mnemonic, wallet_client_model: WalletClientModel) -> Self {
        let mnemonic_with_passphrase = MnemonicWithPassphrase::new(mnemonic);
        let bdfs = DeviceFactorSource::babylon(
            true,
            mnemonic_with_passphrase.clone(),
            wallet_client_model,
        );
        Self::new(mnemonic_with_passphrase, bdfs)
    }

    pub fn new_with_entropy(entropy: Hex32Bytes, wallet_client_model: WalletClientModel) -> Self {
        let mnemonic = Mnemonic::from_hex32(entropy);
        Self::new_with_mnemonic(mnemonic, wallet_client_model)
    }

    pub fn generate_new(wallet_client_model: WalletClientModel) -> Self {
        Self::new_with_entropy(Hex32Bytes::generate(), wallet_client_model)
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
