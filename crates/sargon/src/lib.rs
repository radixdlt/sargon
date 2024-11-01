#![feature(async_closure)]
#![feature(let_chains)]
#![feature(core_intrinsics)]
#![allow(unused_imports)]
#![allow(internal_features)]
#![feature(iter_repeat_n)]

mod core;
mod factor_instances_provider;
mod gateway_api;
mod hierarchical_deterministic;
mod home_cards;
mod profile;
mod radix_connect;
mod signing;
mod system;
mod types;
mod wrapped_radix_engine_toolkit;

pub mod prelude {

    pub use crate::core::*;
    pub use crate::factor_instances_provider::*;
    pub use crate::gateway_api::*;
    pub use crate::hierarchical_deterministic::*;
    pub use crate::home_cards::*;
    pub use crate::profile::*;
    pub use crate::radix_connect::*;
    pub use crate::signing::*;
    pub use crate::system::*;
    pub use crate::types::*;
    pub use crate::wrapped_radix_engine_toolkit::*;

    pub use radix_rust::prelude::{
        BTreeSet, HashMap, HashSet, IndexMap, IndexSet,
    };

    pub(crate) use ::hex::decode as hex_decode;
    pub(crate) use ::hex::encode as hex_encode;
    pub(crate) use iso8601_timestamp::Timestamp;
    pub(crate) use itertools::Itertools;
    pub(crate) use log::{debug, error, info, trace, warn};
    pub(crate) use once_cell::sync::Lazy;
    pub(crate) use serde::{
        de, ser::SerializeStruct, Deserialize, Deserializer, Serialize,
        Serializer,
    };
    pub(crate) use serde_json::json;
    pub(crate) use serde_repr::{Deserialize_repr, Serialize_repr};
    pub(crate) use serde_with::*;
    pub(crate) use zeroize::{Zeroize, ZeroizeOnDrop};

    pub(crate) use derive_more::derive::{
        AsRef, Debug as MoreDebug, Deref, Display,
    };
    pub use radix_common::math::traits::CheckedMul as ScryptoCheckedMul;
    pub(crate) use std::cell::RefCell;
    pub(crate) use std::cmp::Ordering;
    pub(crate) use std::collections::BTreeMap;
    pub(crate) use std::fmt::{Debug, Display, Formatter};
    pub(crate) use std::fs;
    pub(crate) use std::hash::Hash as StdHash;
    pub use std::ops::{Add, AddAssign, Deref, Div, Mul, Neg, Sub};
    pub(crate) use std::str::FromStr;
    pub(crate) use std::sync::{Arc, RwLock};

    pub(crate) use strum::FromRepr;
    pub(crate) use strum::IntoEnumIterator;
    pub(crate) use url::Url;
    pub(crate) use uuid::Uuid;

    pub(crate) use enum_as_inner::EnumAsInner;
    pub(crate) use paste::*;
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
        address::{
            AddressBech32Decoder as ScryptoAddressBech32Decoder,
            AddressBech32Encoder as ScryptoAddressBech32Encoder,
        },
        crypto::{
            blake2b_256_hash, verify_ed25519 as scrypto_verify_ed25519,
            verify_secp256k1 as scrypto_verify_secp256k1,
            Ed25519PrivateKey as ScryptoEd25519PrivateKey,
            Ed25519PublicKey as ScryptoEd25519PublicKey,
            Ed25519PublicKeyHash as ScryptoEd25519PublicKeyHash,
            Ed25519Signature as ScryptoEd25519Signature, Hash as ScryptoHash,
            IsHash as ScryptoIsHash, PublicKey as ScryptoPublicKey,
            PublicKeyHash as ScryptoPublicKeyHash,
            Secp256k1PrivateKey as ScryptoSecp256k1PrivateKey,
            Secp256k1PublicKey as ScryptoSecp256k1PublicKey,
            Secp256k1PublicKeyHash as ScryptoSecp256k1PublicKeyHash,
            Secp256k1Signature as ScryptoSecp256k1Signature,
        },
        data::scrypto::{
            model::{
                BytesNonFungibleLocalId as ScryptoBytesNonFungibleLocalId,
                IntegerNonFungibleLocalId as ScryptoIntegerNonFungibleLocalId,
                NonFungibleLocalId as ScryptoNonFungibleLocalId,
                RUIDNonFungibleLocalId as ScryptoRUIDNonFungibleLocalId,
                StringNonFungibleLocalId as ScryptoStringNonFungibleLocalId,
            },
            scrypto_decode as Scrypto_scrypto_decode,
            scrypto_encode as Scrypto_scrypto_encode,
        },
        math::{
            Decimal as ScryptoDecimal192, RoundingMode as ScryptoRoundingMode,
        },
        network::NetworkDefinition as ScryptoNetworkDefinition,
        prelude::{
            recover_secp256k1 as Scrypto_recover_secp256k1,
            AllowedIds as ScryptoAllowedIds,
            FromPublicKey as ScryptoFromPublicKey, Instant as ScryptoInstant,
            LowerBound as ScryptoLowerBound,
            ManifestAddress as ScryptoManifestAddress,
            ManifestAddressReservation as ScryptoManifestAddressReservation,
            ManifestBucket as ScryptoManifestBucket,
            ManifestCustomValue as ScryptoManifestCustomValue,
            ManifestCustomValueKind as ScryptoManifestCustomValueKind,
            ManifestEncode as ScryptoManifestEncode,
            ManifestNamedAddress as ScryptoManifestNamedAddress,
            ManifestProof as ScryptoManifestProof,
            ManifestValue as ScryptoManifestValue,
            NonFungibleData as ScryptoNonFungibleData,
            NonFungibleGlobalId as ScryptoNonFungibleGlobalId,
            NonFungibleIdType as ScryptoNonFungibleIdType,
            UpperBound as ScryptoUpperBound, XRD,
        },
        types::{
            ComponentAddress as ScryptoComponentAddress,
            EntityType as ScryptoEntityType,
            GlobalAddress as ScryptoGlobalAddress, NodeId as ScryptoNodeId,
            ResourceAddress as ScryptoResourceAddress,
        },
        ManifestSbor as ScryptoManifestSbor, ScryptoSbor,
    };
    pub(crate) use radix_engine_interface::blueprints::{
        account::{
            DefaultDepositRule as ScryptoDefaultDepositRule,
            ResourcePreference as ScryptoResourcePreference,
        },
        resource::ResourceOrNonFungible as ScryptoResourceOrNonFungible,
    };
    pub(crate) use radix_engine_interface::prelude::{
        AccessRule as ScryptoAccessRule,
        BasicRequirement as ScryptoBasicRequirement,
        CompositeRequirement as ScryptoCompositeRequirement,
        Epoch as ScryptoEpoch,
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
            ExistingManifestBucket as ScryptoExistingManifestBucket,
            ManifestNameRegistrar as ScryptoManifestNameRegistrar,
            NewManifestBucket as ScryptoNewManifestBucket,
            PartialTransactionV2Builder as ScryptoPartialTransactionV2Builder,
            ResolvableArguments as ScryptoResolvableArguments,
            ResolvableComponentAddress as ScryptoResolvableComponentAddress,
        },
        manifest::{
            compile as scrypto_compile,
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
            KnownManifestObjectNames as ScryptoKnownManifestObjectNames,
            ManifestObjectNames as ScryptoManifestObjectNames,
            MockBlobProvider as ScryptoMockBlobProvider,
        },
        model::{
            BlobV1 as ScryptoBlob, BlobsV1 as ScryptoBlobs,
            ChildIntentsV2 as ScryptoChildIntents,
            ChildSubintent as ScryptoChildSubintent,
            DynamicComponentAddress as ScryptoDynamicComponentAddress,
            DynamicGlobalAddress as ScryptoDynamicGlobalAddress,
            DynamicResourceAddress as ScryptoDynamicResourceAddress,
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
            SignatureV1 as ScryptoSignature,
            SignatureWithPublicKeyV1 as ScryptoSignatureWithPublicKey,
            SignedIntentV1 as ScryptoSignedIntent,
            SignedPartialTransactionV2 as ScryptoSignedPartialTransaction,
            SignedTransactionIntentHash as ScryptoSignedTransactionIntentHash,
            SubintentHash as ScryptoSubintentHash,
            SubintentV2 as ScryptoSubintent,
            TransactionHashBech32Decoder as ScryptoTransactionHashBech32Decoder,
            TransactionHashBech32Encoder as ScryptoTransactionHashBech32Encoder,
            TransactionHeaderV1 as ScryptoTransactionHeader,
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
                    statically_analyze as RET_statically_analyze,
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
                    statically_analyze as RET_statically_analyze_subintent_manifest,
                    to_payload_bytes as RET_to_payload_bytes_subintent_manifest,
                },
                transaction_intent::{
                    hash as ret_hash_transaction_intent_v2,
                    to_payload_bytes as RET_transaction_intent_to_payload_bytes_v2,
                },
                transaction_manifest::{
                    dynamically_analyze as RET_dynamically_analyze_v2,
                    from_payload_bytes as RET_from_payload_bytes_manifest_v2,
                    statically_analyze as RET_statically_analyze_v2,
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
            NewEntities as RetNewEntities,
            NonFungibleResourceIndicator as RetNonFungibleResourceIndicator,
            Operation as RetOperation, Predicted as RetPredicted,
            ReservedInstruction as RetReservedInstruction,
            ResourceIndicator as RetResourceIndicator,
            StaticAnalysis as RetStaticAnalysis,
            TrackedPoolContribution as RetTrackedPoolContribution,
            TrackedPoolRedemption as RetTrackedPoolRedemption,
            TrackedValidatorClaim as RetTrackedValidatorClaim,
            TrackedValidatorStake as RetTrackedValidatorStake,
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
