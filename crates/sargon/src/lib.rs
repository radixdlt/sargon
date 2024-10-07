#![feature(async_closure)]
#![feature(let_chains)]
#![feature(core_intrinsics)]
#![allow(unused_imports)]
#![allow(internal_features)]

mod core;
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
    pub use crate::gateway_api::*;
    pub use crate::hierarchical_deterministic::*;
    pub use crate::home_cards::*;
    pub use crate::profile::*;
    pub use crate::radix_connect::*;
    pub use crate::signing::*;
    pub use crate::system::*;
    pub use crate::types::*;
    pub use crate::wrapped_radix_engine_toolkit::*;

    pub(crate) use radix_rust::prelude::{
        BTreeSet, HashMap, HashSet, IndexMap, IndexSet,
    };

    pub(crate) use ::hex::decode as hex_decode;
    pub(crate) use ::hex::encode as hex_encode;
    pub(crate) use iso8601_timestamp::Timestamp;
    pub(crate) use itertools::Itertools;
    pub(crate) use log::{debug, error, info, trace, warn};
    pub(crate) use serde::{
        de, ser::SerializeStruct, Deserialize, Deserializer, Serialize,
        Serializer,
    };
    pub(crate) use serde_json::json;
    pub(crate) use serde_repr::{Deserialize_repr, Serialize_repr};
    pub(crate) use serde_with::*;
    pub(crate) use zeroize::{Zeroize, ZeroizeOnDrop};
    pub(crate) use once_cell::sync::Lazy;

    pub use radix_common::math::traits::CheckedMul as ScryptoCheckedMul;
    pub(crate) use std::cmp::Ordering;
    pub(crate) use std::collections::BTreeMap;
    pub(crate) use std::fmt::{Debug, Display, Formatter};
    pub(crate) use std::fs;
    pub(crate) use std::cell::RefCell;
    pub(crate) use std::hash::Hash as StdHash;
    pub use std::ops::{Add, AddAssign, Deref, Div, Mul, Neg, Sub};
    pub(crate) use std::str::FromStr;
    pub(crate) use std::sync::{Arc, RwLock};

    pub(crate) use strum::FromRepr;
    pub(crate) use strum::IntoEnumIterator;
    pub(crate) use url::Url;
    pub(crate) use uuid::Uuid;

    pub(crate) use enum_as_inner::EnumAsInner;
    pub(crate) use native_radix_engine_toolkit::receipt::RuntimeToolkitTransactionReceipt as ScryptoRuntimeToolkitTransactionReceipt;
    pub(crate) use native_radix_engine_toolkit::receipt::SerializableToolkitTransactionReceipt as ScryptoSerializableToolkitTransactionReceipt;
    pub(crate) use paste::*;
    pub(crate) use radix_engine::{
        blueprints::consensus_manager::UnstakeData as ScryptoUnstakeData,
        system::system_modules::execution_trace::ResourceSpecifier as ScryptoResourceSpecifier,
        transaction::{
            FeeLocks as ScryptoFeeLocks,
            TransactionReceiptV1 as ScryptoTransactionReceipt,
            VersionedTransactionReceipt as ScryptoVersionedTransactionReceipt,
        },
    };
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
            FromPublicKey as ScryptoFromPublicKey,
            ManifestAddress as ScryptoManifestAddress,
            ManifestBucket as ScryptoManifestBucket,
            ManifestCustomValue as ScryptoManifestCustomValue,
            ManifestCustomValueKind as ScryptoManifestCustomValueKind,
            ManifestEncode as ScryptoManifestEncode,
            ManifestValue as ScryptoManifestValue,
            NonFungibleData as ScryptoNonFungibleData,
            NonFungibleGlobalId as ScryptoNonFungibleGlobalId,
            NonFungibleIdType as ScryptoNonFungibleIdType, XRD,
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
        AccessRuleNode as ScryptoAccessRuleNode, Epoch as ScryptoEpoch,
        FungibleResourceRoles as ScryptoFungibleResourceRoles,
        MetadataInit as ScryptoMetadataInit,
        MetadataValue as ScryptoMetadataValue,
        ModuleConfig as ScryptoModuleConfig,
        NonFungibleResourceRoles as ScryptoNonFungibleResourceRoles,
        OwnerRole as ScryptoOwnerRole, ProofRule as ScryptoProofRule,
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
            ResolvableArguments as ScryptoResolvableArguments,
            ResolvableComponentAddress as ScryptoResolvableComponentAddress,
        },
        manifest::{
            compile as scrypto_compile, decompile as scrypto_decompile,
            generator::{GeneratorError, GeneratorErrorKind},
            lexer::{LexerError, LexerErrorKind},
            token::{Position, Span},
            CompileError as ScryptoCompileError,
            MockBlobProvider as ScryptoMockBlobProvider,
        },
        model::{
            BlobV1 as ScryptoBlob, BlobsV1 as ScryptoBlobs,
            DynamicComponentAddress as ScryptoDynamicComponentAddress,
            DynamicGlobalAddress as ScryptoDynamicGlobalAddress,
            DynamicResourceAddress as ScryptoDynamicResourceAddress,
            HashHasHrp as ScryptoHashHasHrp,
            InstructionV1 as ScryptoInstruction,
            InstructionsV1 as ScryptoInstructions,
            IntentHash as ScryptoIntentHash,
            IntentSignatureV1 as ScryptoIntentSignature,
            IntentSignaturesV1 as ScryptoIntentSignatures,
            IntentV1 as ScryptoIntent,
            MessageContentsV1 as ScryptoMessageContents,
            MessageV1 as ScryptoMessage,
            NotarizedTransactionV1 as ScryptoNotarizedTransaction,
            NotarySignatureV1 as ScryptoNotarySignature,
            PlaintextMessageV1 as ScryptoPlaintextMessage,
            SignatureV1 as ScryptoSignature,
            SignatureWithPublicKeyV1 as ScryptoSignatureWithPublicKey,
            SignedIntentHash as ScryptoSignedIntentHash,
            SignedIntentV1 as ScryptoSignedIntent,
            TransactionHashBech32Decoder as ScryptoTransactionHashBech32Decoder,
            TransactionHashBech32Encoder as ScryptoTransactionHashBech32Encoder,
            TransactionHeaderV1 as ScryptoTransactionHeader,
        },
        prelude::{
            ManifestBuilder as ScryptoManifestBuilder,
            TransactionManifestV1 as ScryptoTransactionManifest,
        },
    };

    pub(crate) use radix_engine_toolkit_json::models::{
        common::SerializableNonFungibleLocalId as RetNonFungibleLocalId,
        scrypto::non_fungible_global_id::{
            SerializableNonFungibleGlobalId as RetNonFungibleGlobalId,
            SerializableNonFungibleGlobalIdInternal as RetNonFungibleGlobalIdInternal,
        },
    };

    pub use radix_engine_toolkit::{
        functions::{
            instructions::{
                compile as RET_compile_instructions,
                decompile as RET_decompile_instructions,
                extract_addresses as RET_ins_extract_addresses,
            },
            intent::{compile as RET_intent_compile, hash as ret_hash_intent},
            manifest::summary as RET_summary,
            notarized_transaction::{
                compile as RET_compile_notarized_tx,
                decompile as RET_decompile_notarize_tx,
            },
            signed_intent::hash as RET_signed_intent_hash,
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
            ExecutionSummary as RetExecutionSummary,
            FeeSummary as RetFeeSummary,
            FungibleResourceIndicator as RetFungibleResourceIndicator,
            ManifestSummary as RetManifestSummary,
            NewEntities as RetNewEntities,
            NonFungibleResourceIndicator as RetNonFungibleResourceIndicator,
            Operation as RetOperation, Predicted as RetPredicted,
            ReservedInstruction as RetReservedInstruction,
            ResourceIndicator as RetResourceIndicator,
            TrackedPoolContribution as RetTrackedPoolContribution,
            TrackedPoolRedemption as RetTrackedPoolRedemption,
            TrackedValidatorClaim as RetTrackedValidatorClaim,
            TrackedValidatorStake as RetTrackedValidatorStake,
            Update as RetUpdate,
        },
    };
}

pub use prelude::*;

// Use `Url` as a custom type, with `String` as the Builtin
uniffi::custom_type!(Url, String);

// Use `url::Url` as a custom type, with `String` as the Builtin
#[cfg(not(tarpaulin_include))] // Tested in binding tests (e.g. test*.swift files)
impl UniffiCustomTypeConverter for Url {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(Url::parse(&val)?)
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.into()
    }
}

// Use `Timestamp` as a custom type, with `String` as the Builtin
uniffi::custom_type!(Timestamp, String);

#[cfg(not(tarpaulin_include))] // Tested in binding tests (e.g. test*.swift files)
impl UniffiCustomTypeConverter for Timestamp {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Timestamp::parse(val.as_str())
            .ok_or(CommonError::InvalidISO8601String { bad_value: val })
            .map_err(|e| e.into())
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.to_string()
    }
}

// Use `Uuid` as a custom type, with `String` as the Builtin
uniffi::custom_type!(Uuid, String);

#[cfg(not(tarpaulin_include))] // Tested in binding tests (e.g. test*.swift files)
impl UniffiCustomTypeConverter for Uuid {
    type Builtin = String;
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Uuid::try_parse(val.as_str()).map_err(|e| e.into())
    }
    fn from_custom(obj: Self) -> Self::Builtin {
        obj.to_string()
    }
}

uniffi::include_scaffolding!("sargon");
