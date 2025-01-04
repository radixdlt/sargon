#![allow(async_fn_in_trait)]
#![allow(internal_features)]
#![allow(incomplete_features)]
#![feature(async_closure)]
#![feature(let_chains)]
#![feature(core_intrinsics)]
#![feature(iter_repeat_n)]
#![feature(future_join)]
#![feature(generic_const_exprs)]
#![feature(trait_upcasting)]
#![feature(trivial_bounds)]
#![allow(trivial_bounds)]

mod security_center;
mod signing;
mod system;
mod types;

pub mod prelude {
    pub use crate::security_center::*;
    pub use crate::signing::*;
    pub use crate::system::*;
    pub use crate::types::*;

    pub use clients::prelude::*;
    pub use factor_instances_provider::prelude::*;
    pub use home_cards::prelude::*;
    pub use keys_collector::prelude::*;
    pub use manifests::prelude::*;
    pub use profile_logic::prelude::*;
    pub use radix_connect::prelude::*;

    pub(crate) use radix_engine_interface::prelude::MetadataValue as ScryptoMetadataValue;

    // pub(crate) use serde::{
    //     de, ser::SerializeStruct, Deserializer, Serializer,
    // };

    // pub(crate) use serde_with::{serde_as, DisplayFromStr};
    pub(crate) use std::collections::HashSet;

    #[cfg(test)]
    pub(crate) use radix_common::math::Decimal as ScryptoDecimal192;
    // #[cfg(test)]
    // pub(crate) use std::collections::BTreeSet;

    // #[cfg(test)]
    // pub(crate) use serde_json::json;
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
