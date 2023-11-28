use wallet_kit_common::types::keys::public_key::PublicKey;

use crate::derivation::derivation_path::DerivationPath;

/// The **source** of a virtual hierarchical deterministic badge, contains a
/// derivation path and public key, from which a private key is derived which
/// produces virtual badges (signatures).
///
/// The `.device` `FactorSource` produces `FactorInstance`s with this kind if badge source.
pub struct HierarchicalDeterministicPublicKey {
    /// The expected public key of the private key derived at `derivationPath`
    pub publicKey: PublicKey,

    /// The HD derivation path for the key pair which produces virtual badges (signatures).
    pub derivationPath: DerivationPath,
}
