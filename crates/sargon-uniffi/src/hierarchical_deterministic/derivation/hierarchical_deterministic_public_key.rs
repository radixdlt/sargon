use crate::prelude::*;

/// The **source** of a virtual hierarchical deterministic badge, contains a
/// derivation path and public key, from which a private key is derived which
/// produces virtual badges (signatures).
///
/// The `.device` `FactorSource` produces `FactorInstance`s with this kind if badge source.
#[derive(
    Clone, Debug, PartialEq, Eq, Hash, uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
pub struct HierarchicalDeterministicPublicKey {
    /// The expected public key of the private key derived at `derivationPath`
    pub public_key: PublicKey,

    /// The HD derivation path for the key pair which produces virtual badges (signatures).
    pub derivation_path: DerivationPath,
}

#[uniffi::export]
pub fn new_hierarchical_deterministic_public_key_sample(
) -> HierarchicalDeterministicPublicKey {
    HierarchicalDeterministicPublicKey::sample()
}

#[uniffi::export]
pub fn new_hierarchical_deterministic_public_key_sample_other(
) -> HierarchicalDeterministicPublicKey {
    HierarchicalDeterministicPublicKey::sample_other()
}
#[uniffi::export]
pub fn hierarchical_deterministic_public_key_is_valid_signature_for_hash(
    key: &HierarchicalDeterministicPublicKey,
    signature: Signature,
    hash: &Hash,
) -> bool {
    key.is_valid_signature_for_hash(signature, hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = HierarchicalDeterministicPublicKey;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_hierarchical_deterministic_public_key_sample(),
                new_hierarchical_deterministic_public_key_sample_other(),
                // duplicates should get removed
                new_hierarchical_deterministic_public_key_sample(),
                new_hierarchical_deterministic_public_key_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn is_valid() {
        let private_key = HierarchicalDeterministicPrivateKey::sample();
        let msg = Hash::sample();
        let sut = private_key.sign(&msg);
        let public_key = private_key.public_key();
        assert!(
            hierarchical_deterministic_public_key_is_valid_signature_for_hash(
                &public_key,
                sut.signature(),
                &msg
            )
        );
    }
}
