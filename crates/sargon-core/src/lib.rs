#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(trivial_bounds)]
#![allow(trivial_bounds)]

mod encryption;
mod has_sample_values;
mod hash;
mod image_url_utils;
mod is_network_aware;
mod metadata;
mod network_id;
mod secure_random_bytes;
mod types;
mod appendable_collection;
mod unsafe_id_stepper;
mod unsigned_ints;

pub fn parse_url(s: impl AsRef<str>) -> Result<Url, CommonError> {
    Url::try_from(s.as_ref()).map_err(|_| CommonError::InvalidURL {
        bad_value: s.as_ref().to_owned(),
    })
}

pub mod prelude {
    pub use identified_vec_of::prelude::*;
    pub use sargon_core_utils::prelude::*;

    pub use crate::encryption::*;
    pub use crate::has_sample_values::*;
    pub use crate::hash::*;
    pub use crate::appendable_collection::*;
    pub use crate::image_url_utils::*;
    pub use crate::is_network_aware::*;
    pub use crate::metadata::*;
    pub use crate::network_id::*;
    pub use crate::secure_random_bytes::*;
    pub use crate::types::*;
    pub use crate::unsafe_id_stepper::*;
    pub use crate::unsigned_ints::*;

    pub use radix_rust::prelude::{
        indexmap, BTreeSet, HashMap, HashSet, IndexMap, IndexSet,
    };
    pub use std::marker::PhantomData;

    pub use ::hex::decode as hex_decode;
    pub use ::hex::encode as hex_encode;
    pub use itertools::Itertools;
    pub use log::{debug, error, info, trace, warn};
    pub use serde::{
        de, ser::SerializeStruct, Deserialize, Deserializer, Serialize,
        Serializer,
    };
    pub use serde_json::json;
    pub use serde_repr::{Deserialize_repr, Serialize_repr};
    pub use serde_with::*;
    pub use zeroize::{Zeroize, ZeroizeOnDrop};

    pub use derive_more::derive::{AsRef, Debug as MoreDebug, Deref, Display};

    pub use std::cell::RefCell;
    pub use std::cmp::Ordering;
    pub use std::collections::BTreeMap;
    pub use std::fmt::{Debug, Display, Formatter};
    pub use std::fs;
    pub use std::hash::Hash as StdHash;
    pub use std::ops::{Add, AddAssign, Deref, Div, Mul, Neg, Sub};
    pub use std::str::FromStr;
    pub use std::sync::{Arc, RwLock};

    pub use strum::FromRepr;
    pub use strum::IntoEnumIterator;
    pub use url::Url;
    pub use uuid::Uuid;

    pub use enum_as_inner::EnumAsInner;
    pub use paste::*;

    pub(crate) use radix_common::{
        crypto::{
            blake2b_256_hash, verify_ed25519 as scrypto_verify_ed25519,
            verify_secp256k1 as scrypto_verify_secp256k1,
            Ed25519PrivateKey as ScryptoEd25519PrivateKey,
            Ed25519PublicKey as ScryptoEd25519PublicKey,
            Ed25519Signature as ScryptoEd25519Signature, Hash as ScryptoHash,
            IsHash as ScryptoIsHash, PublicKey as ScryptoPublicKey,
            Secp256k1PrivateKey as ScryptoSecp256k1PrivateKey,
            Secp256k1PublicKey as ScryptoSecp256k1PublicKey,
            Secp256k1Signature as ScryptoSecp256k1Signature,
        },
        data::scrypto::model::BytesNonFungibleLocalId as ScryptoBytesNonFungibleLocalId,
        math::{
            traits::CheckedMul as ScryptoCheckedMul,
            Decimal as ScryptoDecimal192, RoundingMode as ScryptoRoundingMode,
        },
        network::NetworkDefinition as ScryptoNetworkDefinition,
        prelude::Instant as ScryptoInstant,
    };

    pub(crate) use radix_engine_interface::prelude::{
        MetadataValue as ScryptoMetadataValue,
        ToMetadataEntry as ScryptoToMetadataEntry,
    };

    pub use radix_engine_interface::prelude::Epoch as ScryptoEpoch;

    pub use radix_transactions::model::{
        SignatureV1 as ScryptoSignature,
        SignatureWithPublicKeyV1 as ScryptoSignatureWithPublicKey,
    };
}

pub use prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_url() {
        assert!(parse_url("https://radixdlt.com").is_ok());
    }

    #[test]
    fn test_parse_url_invalid() {
        assert!(parse_url("https/radixdlt").is_err());
    }
}
