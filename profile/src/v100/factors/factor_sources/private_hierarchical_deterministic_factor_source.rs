use hierarchical_deterministic::{
    bip32::hd_path_component::HDPathValue,
    cap26::{
        cap26_key_kind::CAP26KeyKind, cap26_path::paths::account_path::AccountPath,
        cap26_repr::CAP26Repr,
    },
    derivation::mnemonic_with_passphrase::MnemonicWithPassphrase,
};
use wallet_kit_common::network_id::NetworkID;

use crate::v100::factors::{
    factor_source_id_from_hash::FactorSourceIDFromHash,
    hd_transaction_signing_factor_instance::HDFactorInstanceAccountCreation,
    hierarchical_deterministic_factor_instance::HierarchicalDeterministicFactorInstance,
    is_factor_source::IsFactorSource,
};

use super::device_factor_source::device_factor_source::DeviceFactorSource;

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
