mod test_diagnose_instance_duplicates;

pub mod prelude {

    pub use build_info::prelude::*;
    pub use clients::prelude::*;
    pub use factor_instances_provider::prelude::*;
    pub use home_cards::prelude::*;
    pub use interactors::prelude::*;
    pub use key_derivation_traits::prelude::*;
    pub use keys_collector::prelude::*;
    pub use manifests::prelude::*;
    pub use profile_logic::prelude::*;
    pub use radix_connect::prelude::*;
    pub use radix_connect_models::prelude::*;
    pub use sargon_os::prelude::*;
    pub use sargon_os_accounts::prelude::*;
    pub use sargon_os_derive_public_keys::prelude::*;
    pub use sargon_os_factors::prelude::*;
    pub use sargon_os_security_center::prelude::*;
    pub use sargon_os_signing::prelude::*;
    pub use sargon_os_transaction::prelude::*;
    pub use security_center::prelude::*;
    pub use signatures_collector::prelude::*;
    pub use signing_traits::prelude::*;
    pub use sub_systems::prelude::*;
}

pub use prelude::*;

/// Helper implementation for Uniffi
pub fn android_notarize_hash_with_private_key_bytes(
    private_key_bytes: Exactly32Bytes,
    signed_transaction_intent_hash: &SignedTransactionIntentHash,
) -> Result<NotarySignature> {
    let ed25519_private_key =
        Ed25519PrivateKey::try_from(private_key_bytes.as_ref())?;

    let private_key = PrivateKey::from(ed25519_private_key);
    let signature = private_key.notarize_hash(signed_transaction_intent_hash);

    Ok(signature)
}

pub fn android_sign_hash_with_private_key_bytes(
    private_key_bytes: Exactly32Bytes,
    hash: &Hash,
) -> Result<Ed25519Signature> {
    Ed25519PrivateKey::try_from(private_key_bytes.as_ref())
        .map(|pk| pk.sign(hash))
}

#[cfg(test)]
mod helper_tests {
    use super::*;

    #[test]
    fn test_android_notarize_hash_with_private_key_bytes() {
        assert!(android_notarize_hash_with_private_key_bytes(
            Exactly32Bytes::sample(),
            &SignedTransactionIntentHash::sample()
        )
        .is_ok());
    }

    #[test]
    fn test_android_sign_hash_with_private_key_bytes() {
        assert!(android_sign_hash_with_private_key_bytes(
            Exactly32Bytes::sample(),
            &Hash::sample()
        )
        .is_ok());
    }
}
