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

mod home_cards;
mod needs_a_new_home_dumping_ground;
mod radix_connect;
mod security_center;
mod signing;
mod system;
mod types;

pub mod prelude {
    pub use gateway_client_and_api::prelude::*;
    pub use sargon_clients::prelude::*;
    pub use sargon_factor_instances_provider::prelude::*;
    pub use sargon_keys_collector::prelude::*;
    pub use sargon_manifests::prelude::*;
    pub use sargon_profile_logic::prelude::*;

    pub use crate::home_cards::*;
    pub use crate::radix_connect::*;
    pub use crate::security_center::*;
    pub use crate::signing::*;
    pub use crate::system::*;
    pub use crate::types::*;

    pub(crate) use radix_engine_interface::prelude::MetadataValue as ScryptoMetadataValue;

    #[cfg(test)]
    pub(crate) use radix_common::math::Decimal as ScryptoDecimal192;

    pub use radix_engine_toolkit::{
        functions::{
            transaction_v1::{
                instructions::extract_addresses as RET_ins_extract_addresses,
                intent::{
                    from_payload_bytes as RET_intent_from_payload_bytes,
                    hash as ret_hash_intent,
                    to_payload_bytes as RET_intent_to_payload_bytes,
                },
                manifest::{
                    from_payload_bytes as RET_from_payload_bytes_manifest_v1,
                    statically_analyze_and_validate as RET_statically_analyze_and_validate,
                    to_payload_bytes as RET_to_payload_bytes_manifest_v1,
                },
                notarized_transaction::{
                    from_payload_bytes as RET_decompile_notarize_tx,
                    to_payload_bytes as RET_compile_notarized_tx,
                },
                signed_intent::hash as RET_signed_intent_hash,
            },
            transaction_v2::{
                instructions::extract_addresses as RET_ins_extract_addresses_v2,
                notarized_transaction::{
                    from_payload_bytes as RET_decompile_notarize_tx_v2,
                    to_payload_bytes as RET_compile_notarized_tx_v2,
                },
                signed_partial_transaction::{
                    from_payload_bytes as RET_decompile_signed_partial_tx,
                    to_payload_bytes as RET_compile_signed_partial_tx,
                },
                signed_transaction_intent::hash as RET_signed_intent_hash_v2,
                subintent::{
                    from_payload_bytes as RET_subintent_from_payload_bytes,
                    hash as ret_hash_subintent,
                    to_payload_bytes as RET_subintent_to_payload_bytes,
                },
                subintent_manifest::{
                    as_enclosed as RET_subintent_manifest_as_enclosed,
                    from_payload_bytes as RET_from_payload_bytes_subintent_manifest,
                    statically_analyze_and_validate as RET_statically_analyze_and_validate_subintent_manifest,
                    to_payload_bytes as RET_to_payload_bytes_subintent_manifest,
                },
                transaction_intent::{
                    hash as ret_hash_transaction_intent_v2,
                    to_payload_bytes as RET_transaction_intent_to_payload_bytes_v2,
                },
                transaction_manifest::{
                    dynamically_analyze as RET_dynamically_analyze_v2,
                    from_payload_bytes as RET_from_payload_bytes_manifest_v2,
                    statically_analyze_and_validate as RET_statically_analyze_and_validate_v2,
                    to_payload_bytes as RET_to_payload_bytes_manifest_v2,
                },
            },
        },
        models::{
            canonical_address_types::{
                CanonicalAccessControllerAddress as RetAccessControllerAddress,
                CanonicalAccountAddress as RetAccountAddress,
                CanonicalAddress as RetIsAddressTrait,
                CanonicalComponentAddress as RetComponentAddress,
                CanonicalIdentityAddress as RetIdentityAddress,
                CanonicalLockerAddress as RetLockerAddress,
                CanonicalPackageAddress as RetPackageAddress,
                CanonicalPoolAddress as RetPoolAddress,
                CanonicalResourceAddress as RetResourceAddress,
                CanonicalValidatorAddress as RetValidatorAddress,
                CanonicalVaultAddress as RetVaultAddress,
            },
            node_id::TypedNodeId as RetTypedNodeId,
        },
        transaction_types::{
            DetailedManifestClass as RetDetailedManifestClass,
            DynamicAnalysis as RetDynamicAnalysis, FeeSummary as RetFeeSummary,
            FungibleResourceIndicator as RetFungibleResourceIndicator,
            ManifestClass as RetManifestClass, NewEntities as RetNewEntities,
            NonFungibleResourceIndicator as RetNonFungibleResourceIndicator,
            Operation as RetOperation, Predicted as RetPredicted,
            ReservedInstruction as RetReservedInstruction,
            ResourceIndicator as RetResourceIndicator,
            StaticAnalysisWithResourceMovements as RetStaticAnalysisWithResourceMovements,
            TrackedPoolContribution as RetTrackedPoolContribution,
            TrackedPoolRedemption as RetTrackedPoolRedemption,
            TrackedValidatorClaim as RetTrackedValidatorClaim,
            TrackedValidatorStake as RetTrackedValidatorStake,
            TransactionTypesError as RetTransactionTypesError,
            Update as RetUpdate,
        },
    };
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
