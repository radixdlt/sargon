mod address;
mod address_conversion;
mod resource;

pub mod prelude {

    pub(crate) use identified_vec_of::prelude::*;
    pub(crate) use sargon_bytes::prelude::*;
    pub(crate) use sargon_core_network::prelude::*;
    pub(crate) use sargon_ecc::prelude::*;
    pub(crate) use sargon_factors::prelude::*;
    pub(crate) use sargon_hierarchical_deterministic::prelude::*;

    pub use crate::address::*;
    pub use crate::resource::*;

    pub(crate) use radix_engine_interface::blueprints::resource::ResourceOrNonFungible as ScryptoResourceOrNonFungible;

    pub use radix_common::{
        address::{
            AddressBech32Decoder as ScryptoAddressBech32Decoder,
            AddressBech32Encoder as ScryptoAddressBech32Encoder,
        },
        crypto::{
            Ed25519PublicKey as ScryptoEd25519PublicKey,
            Ed25519PublicKeyHash as ScryptoEd25519PublicKeyHash,
            Hash as ScryptoHash, PublicKey as ScryptoPublicKey,
            PublicKeyHash as ScryptoPublicKeyHash,
            Secp256k1PublicKeyHash as ScryptoSecp256k1PublicKeyHash,
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
        prelude::{
            AllowedIds as ScryptoAllowedIds,
            DynamicComponentAddress as ScryptoDynamicComponentAddress,
            DynamicGlobalAddress as ScryptoDynamicGlobalAddress,
            DynamicResourceAddress as ScryptoDynamicResourceAddress,
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
            NonFungibleIdType as ScryptoNonFungibleIdType, XRD,
        },
        types::{
            ComponentAddress as ScryptoComponentAddress,
            EntityType as ScryptoEntityType,
            GlobalAddress as ScryptoGlobalAddress, NodeId as ScryptoNodeId,
            ResourceAddress as ScryptoResourceAddress,
        },
    };

    pub(crate) use radix_engine::system::system_modules::execution_trace::ResourceSpecifier as ScryptoResourceSpecifier;

    pub use radix_engine_toolkit::models::{
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
    };

    pub(crate) use enum_as_inner::EnumAsInner;
    pub(crate) use log::*;
    pub(crate) use serde::{Deserialize, Serialize};
    pub(crate) use serde_with::{DeserializeFromStr, SerializeDisplay};
    pub(crate) use std::str::FromStr;

    #[cfg(test)]
    pub(crate) use serde_json::json;

    #[cfg(test)]
    pub(crate) use std::collections::HashSet;
}

pub use prelude::*;
