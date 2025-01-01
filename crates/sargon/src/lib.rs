#![allow(unused_imports)]
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
    pub use identified_vec_of::prelude::*;
    pub use sargon_addresses::prelude::*;
    pub use sargon_clients::prelude::*;
    pub use sargon_core::prelude::*;
    pub use sargon_factor_instances_provider::prelude::*;
    pub use sargon_factors::prelude::*;
    pub use sargon_hierarchical_deterministic::prelude::*;
    pub use sargon_keys_collector::prelude::*;
    pub use sargon_manifests::prelude::*;
    pub use sargon_profile::prelude::*;
    pub use sargon_transaction_models::prelude::*;

    pub use crate::home_cards::*;
    pub use crate::needs_a_new_home_dumping_ground::*;
    pub use crate::radix_connect::*;
    pub use crate::security_center::*;
    pub use crate::signing::*;
    pub use crate::system::*;
    pub use crate::types::*;

    pub(crate) use once_cell::sync::Lazy;

    pub(crate) use futures::future::join_all;

    pub(crate) use radix_engine::{
        blueprints::consensus_manager::UnstakeData as ScryptoUnstakeData,
        system::system_modules::execution_trace::ResourceSpecifier as ScryptoResourceSpecifier,
        transaction::{
            FeeLocks as ScryptoFeeLocks,
            TransactionReceiptV1 as ScryptoTransactionReceipt,
        },
    };
    pub(crate) use radix_engine_toolkit_common::receipt::RuntimeToolkitTransactionReceipt as ScryptoRuntimeToolkitTransactionReceipt;
    pub(crate) use radix_engine_toolkit_common::receipt::SerializableToolkitTransactionReceipt as ScryptoSerializableToolkitTransactionReceipt;
    pub(crate) use sbor::Versioned;

    pub(crate) use radix_common::{
        crypto::{
            blake2b_256_hash, verify_ed25519 as scrypto_verify_ed25519,
            verify_secp256k1 as scrypto_verify_secp256k1,
            Ed25519PrivateKey as ScryptoEd25519PrivateKey,
            Ed25519Signature as ScryptoEd25519Signature,
            Secp256k1PrivateKey as ScryptoSecp256k1PrivateKey,
            Secp256k1PublicKey as ScryptoSecp256k1PublicKey,
            Secp256k1Signature as ScryptoSecp256k1Signature,
        },
        math::{
            Decimal as ScryptoDecimal192, RoundingMode as ScryptoRoundingMode,
        },
        network::NetworkDefinition as ScryptoNetworkDefinition,
        prelude::{
            UpperBound as ScryptoUpperBound,
            ACCOUNT_OWNER_BADGE as SCRYPTO_ACCOUNT_OWNER_BADGE,
            IDENTITY_OWNER_BADGE as SCRYPTO_IDENTITY_OWNER_BADGE,
        },
        ManifestSbor as ScryptoManifestSbor, ScryptoSbor,
    };

    pub(crate) use radix_engine_interface::blueprints::{
        access_controller::{
            RecoveryProposal as ScryptoRecoveryProposal,
            RuleSet as ScryptoRuleSet,
        },
        account::{
            DefaultDepositRule as ScryptoDefaultDepositRule,
            ResourcePreference as ScryptoResourcePreference,
            ACCOUNT_SECURIFY_IDENT as SCRYPTO_ACCOUNT_SECURIFY_IDENT,
        },
        identity::IDENTITY_SECURIFY_IDENT as SCRYPTO_IDENTITY_SECURIFY_IDENT,
        resource::ResourceOrNonFungible as ScryptoResourceOrNonFungible,
    };

    pub(crate) use radix_engine_interface::prelude::{
        AccessRule as ScryptoAccessRule,
        BasicRequirement as ScryptoBasicRequirement,
        CompositeRequirement as ScryptoCompositeRequirement,
        FungibleResourceRoles as ScryptoFungibleResourceRoles,
        MetadataInit as ScryptoMetadataInit,
        MetadataValue as ScryptoMetadataValue,
        ModuleConfig as ScryptoModuleConfig,
        NonFungibleResourceRoles as ScryptoNonFungibleResourceRoles,
        OwnerRole as ScryptoOwnerRole,
        RoleAssignmentInit as ScryptoRoleAssignmentInit,
        ToMetadataEntry as ScryptoToMetadataEntry,
        UncheckedUrl as ScryptoUncheckedUrl,
    };

    pub(crate) use enum_iterator::all;

    pub(crate) use radix_transactions::{
        builder::{
            ManifestNameRegistrar as ScryptoManifestNameRegistrar,
            NewManifestBucket as ScryptoNewManifestBucket,
            PartialTransactionV2Builder as ScryptoPartialTransactionV2Builder,
            ResolvableArguments as ScryptoResolvableArguments,
            TransactionV2Builder as ScryptoTransactionV2Builder,
        },
        manifest::{
            compile as scrypto_compile,
            compile_error_diagnostics as scrypto_compile_error_diagnostics,
            compile_manifest as scrypto_compile_manifest,
            decompile as scrypto_decompile,
            generator::{GeneratorError, GeneratorErrorKind},
            lexer::{LexerError, LexerErrorKind},
            static_resource_movements::{
                AccountDeposit as ScryptoAccountDeposit,
                AccountWithdraw as ScryptoAccountWithdraw,
                ChangeSource as ScryptoChangeSource,
                SimpleFungibleResourceBounds as ScryptoSimpleFungibleResourceBounds,
                SimpleNonFungibleResourceBounds as ScryptoSimpleNonFungibleResourceBounds,
                SimpleResourceBounds as ScryptoSimpleResourceBounds,
                UnspecifiedResources as ScryptoUnspecifiedResources,
            },
            token::{Position, Span},
            CompileError as ScryptoCompileError,
            CompileErrorDiagnosticsStyle as ScryptoCompileErrorDiagnosticsStyle,
            KnownManifestObjectNames as ScryptoKnownManifestObjectNames,
            ManifestObjectNames as ScryptoManifestObjectNames,
            MockBlobProvider as ScryptoMockBlobProvider,
        },
        model::{
            BlobV1 as ScryptoBlob, BlobsV1 as ScryptoBlobs,
            ChildSubintentSpecifier as ScryptoChildSubintentSpecifier,
            ChildSubintentSpecifiersV2 as ScryptoChildSubintentSpecifiers,
            InstructionV1 as ScryptoInstruction,
            InstructionV2 as ScryptoInstructionV2,
            InstructionsV1 as ScryptoInstructions,
            InstructionsV2 as ScryptoInstructionsV2,
            IntentCoreV2 as ScryptoIntentCoreV2,
            IntentHash as ScryptoIntentHash,
            IntentHeaderV2 as ScryptoIntentHeaderV2,
            IntentSignatureV1 as ScryptoIntentSignature,
            IntentSignaturesV1 as ScryptoIntentSignatures,
            IntentSignaturesV2 as ScryptoIntentSignaturesV2,
            IntentV1 as ScryptoIntent,
            IsTransactionHashWithStaticHrp as ScryptoIsTransactionHashWithStaticHrp,
            MessageContentsV1 as ScryptoMessageContents,
            MessageV1 as ScryptoMessage, MessageV2 as ScryptoMessageV2,
            NonRootSubintentSignaturesV2 as ScryptoNonRootSubintentSignatures,
            NonRootSubintentsV2 as ScryptoNonRootSubintents,
            NotarizedTransactionV1 as ScryptoNotarizedTransaction,
            NotarySignatureV1 as ScryptoNotarySignature,
            PartialTransactionV2 as ScryptoPartialTransaction,
            PlaintextMessageV1 as ScryptoPlaintextMessage,
            SignedIntentV1 as ScryptoSignedIntent,
            SignedPartialTransactionV2 as ScryptoSignedPartialTransaction,
            SignedTransactionIntentHash as ScryptoSignedTransactionIntentHash,
            SubintentHash as ScryptoSubintentHash,
            SubintentV2 as ScryptoSubintent,
            TransactionHashBech32Decoder as ScryptoTransactionHashBech32Decoder,
            TransactionHashBech32Encoder as ScryptoTransactionHashBech32Encoder,
            TransactionHeaderV1 as ScryptoTransactionHeader,
            TransactionHeaderV2 as ScryptoTransactionHeaderV2,
            TransactionIntentHash as ScryptoTransactionIntentHash,
        },
        prelude::{
            SubintentManifestV2 as ScryptoSubintentManifestV2,
            SubintentManifestV2Builder as ScryptoSubintentManifestV2Builder,
            TransactionManifestV1 as ScryptoTransactionManifest,
            TransactionManifestV1Builder as ScryptoTransactionManifestBuilder,
            TransactionManifestV2 as ScryptoTransactionManifestV2,
            TransactionManifestV2Builder as ScryptoTransactionManifestV2Builder,
        },
    };

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
