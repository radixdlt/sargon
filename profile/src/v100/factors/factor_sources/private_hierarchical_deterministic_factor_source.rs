use crate::{AccountPath, CAP26KeyKind, CAP26Repr, HDPathValue, MnemonicWithPassphrase};
use derive_getters::Getters;

use crate::{
    v100::{
        FactorSourceIDFromHash, HDFactorInstanceAccountCreation,
        HierarchicalDeterministicFactorInstance, IsFactorSource,
    },
    NetworkID,
};

use super::DeviceFactorSource;

#[derive(Getters)]
pub struct PrivateHierarchicalDeterministicFactorSource {
    mnemonic_with_passphrase: MnemonicWithPassphrase,
    factor_source: DeviceFactorSource,
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
            self.factor_source.id().clone(),
            hd_private_key.public_key(),
        );
        HDFactorInstanceAccountCreation::new(hd_factor_instance).unwrap()
    }
}
