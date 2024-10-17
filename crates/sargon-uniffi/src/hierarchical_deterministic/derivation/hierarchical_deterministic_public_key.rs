use crate::prelude::*;
use sargon::HierarchicalDeterministicPublicKey as InternalHierarchicalDeterministicPublicKey;

/// The **source** of a virtual hierarchical deterministic badge, contains a
/// derivation path and public key, from which a private key is derived which
/// produces virtual badges (signatures).
///
/// The `.device` `FactorSource` produces `FactorInstance`s with this kind if badge source.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct HierarchicalDeterministicPublicKey {
    /// The expected public key of the private key derived at `derivationPath`
    pub public_key: PublicKey,

    /// The HD derivation path for the key pair which produces virtual badges (signatures).
    pub derivation_path: DerivationPath,
}

#[uniffi::export]
pub fn new_hierarchical_deterministic_public_key_sample(
) -> HierarchicalDeterministicPublicKey {
    InternalHierarchicalDeterministicPublicKey::sample().into()
}

#[uniffi::export]
pub fn new_hierarchical_deterministic_public_key_sample_other(
) -> HierarchicalDeterministicPublicKey {
    InternalHierarchicalDeterministicPublicKey::sample_other().into()
}

#[uniffi::export]
pub fn hierarchical_deterministic_public_key_is_valid_signature_for_hash(
    key: &HierarchicalDeterministicPublicKey,
    signature: Signature,
    hash: &Hash,
) -> bool {
    key.into_internal().is_valid_signature_for_hash(
        signature.into_internal(),
        &hash.into_internal(),
    )
}
