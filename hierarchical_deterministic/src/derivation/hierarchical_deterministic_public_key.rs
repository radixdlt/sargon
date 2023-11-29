use serde::{Deserialize, Serialize};
use wallet_kit_common::types::keys::public_key::PublicKey;

use crate::derivation::derivation_path::DerivationPath;

/// The **source** of a virtual hierarchical deterministic badge, contains a
/// derivation path and public key, from which a private key is derived which
/// produces virtual badges (signatures).
///
/// The `.device` `FactorSource` produces `FactorInstance`s with this kind if badge source.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HierarchicalDeterministicPublicKey {
    /// The expected public key of the private key derived at `derivationPath`
    pub public_key: PublicKey,

    /// The HD derivation path for the key pair which produces virtual badges (signatures).
    pub derivation_path: DerivationPath,
}

impl HierarchicalDeterministicPublicKey {
    pub fn new(public_key: PublicKey, derivation_path: DerivationPath) -> Self {
        Self {
            public_key,
            derivation_path,
        }
    }
}
