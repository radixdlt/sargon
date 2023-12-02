use wallet_kit_common::types::keys::private_key::PrivateKey;

use super::{
    derivation_path::DerivationPath,
    hierarchical_deterministic_public_key::HierarchicalDeterministicPublicKey,
};

pub struct HierarchicalDeterministicPrivateKey {
    pub private_key: PrivateKey,
    pub derivation_path: DerivationPath,
}

impl HierarchicalDeterministicPrivateKey {
    pub fn new(private_key: PrivateKey, derivation_path: DerivationPath) -> Self {
        Self {
            private_key,
            derivation_path,
        }
    }
}

impl HierarchicalDeterministicPrivateKey {
    pub fn public_key(&self) -> HierarchicalDeterministicPublicKey {
        HierarchicalDeterministicPublicKey::new(
            self.private_key.public_key(),
            self.derivation_path.clone(),
        )
    }

    pub fn to_hex(&self) -> String {
        self.private_key.to_hex()
    }
}
