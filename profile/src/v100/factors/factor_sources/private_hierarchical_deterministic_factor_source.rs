use crate::{
    AccountPath, CAP26KeyKind, CAP26Repr, HDFactorInstanceTransactionSigning, HDPathValue,
    IsEntityPath, MnemonicWithPassphrase, WalletClientModel,
};

use crate::{
    v100::{
        FactorSourceIDFromHash, HDFactorInstanceAccountCreation,
        HierarchicalDeterministicFactorInstance, IsFactorSource,
    },
    NetworkID,
};

use super::DeviceFactorSource;

#[derive(Debug, Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct PrivateHierarchicalDeterministicFactorSource {
    pub mnemonic_with_passphrase: MnemonicWithPassphrase,
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
                mnemonic_with_passphrase.clone()
            )
            .into()
        );
        Self {
            mnemonic_with_passphrase,
            factor_source,
        }
    }

    pub fn generate_new(wallet_client_model: WalletClientModel) -> Self {
        let mnemonic_with_passphrase = MnemonicWithPassphrase::generate_new();
        let bdfs = DeviceFactorSource::babylon(
            true,
            mnemonic_with_passphrase.clone(),
            wallet_client_model,
        );
        Self::new(mnemonic_with_passphrase, bdfs)
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
        let hd_private_key = self
            .mnemonic_with_passphrase
            .derive_private_key(path);
        let hd_factor_instance = HierarchicalDeterministicFactorInstance::new(
            self.factor_source.id.clone(),
            hd_private_key.public_key(),
        );
        HDFactorInstanceTransactionSigning::new(hd_factor_instance).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use radix_engine_interface::sbor::rust::collections::HashSet;

    use crate::WalletClientModel;

    use super::PrivateHierarchicalDeterministicFactorSource;

    #[test]
    fn hash() {
        let n = 100;
        let set = (0..n)
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
