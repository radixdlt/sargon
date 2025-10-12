#![feature(trivial_bounds)]
#![allow(trivial_bounds)]

mod factor_instance;
mod factor_source;
mod factor_source_category;
mod factor_source_common;
mod factor_source_crypto_parameters;
mod factor_source_flag;
mod factor_source_id;
mod factor_source_id_from_address;
mod factor_source_id_from_hash;
mod factor_source_id_spot_check;
mod factor_source_kind;
mod factor_sources;
mod factor_sources_of_kind;
mod hd_factor_instance_account_creation;
mod hd_factor_instance_identity_creation;
mod hd_transaction_signing_factor_instance;
mod hierarchical_deterministic_factor_instance;
mod is_entity_path;
mod is_factor_source;
mod mfa_factor_sources;
mod samples;

pub mod prelude {

    pub(crate) use bytes::prelude::*;
    pub(crate) use identified_vec_of::prelude::*;

    pub(crate) use cap26_models::prelude::*;
    pub(crate) use ecc::prelude::*;
    pub(crate) use hierarchical_deterministic::prelude::*;
    pub(crate) use network::prelude::*;

    pub(crate) use once_cell::sync::Lazy;

    pub use crate::factor_instance::*;
    pub use crate::factor_source::*;
    pub use crate::factor_source_category::*;
    pub use crate::factor_source_common::*;
    pub use crate::factor_source_crypto_parameters::*;
    pub use crate::factor_source_flag::*;
    pub use crate::factor_source_id::*;
    pub use crate::factor_source_id_from_address::*;
    pub use crate::factor_source_id_from_hash::*;
    pub use crate::factor_source_id_spot_check::*;
    pub use crate::factor_source_kind::*;
    pub use crate::factor_sources::*;
    pub use crate::factor_sources_of_kind::*;
    pub use crate::hd_factor_instance_account_creation::*;
    pub use crate::hd_factor_instance_identity_creation::*;
    pub use crate::hd_transaction_signing_factor_instance::*;
    pub use crate::hierarchical_deterministic_factor_instance::*;
    pub use crate::is_entity_path::*;
    pub use crate::is_factor_source::*;
    pub use crate::mfa_factor_sources::*;
    pub use crate::samples::*;

    pub(crate) use radix_common::{
        crypto::PublicKey as ScryptoPublicKey,
        prelude::NonFungibleGlobalId as ScryptoNonFungibleGlobalId,
        types::FromPublicKey as _,
    };
    pub(crate) use radix_engine_interface::blueprints::resource::ResourceOrNonFungible as ScryptoResourceOrNonFungible;

    pub(crate) use enum_as_inner::EnumAsInner;
    pub(crate) use indexmap::{IndexMap, IndexSet};
    pub(crate) use serde::{
        de, ser::SerializeStruct, Deserialize, Deserializer, Serialize,
        Serializer,
    };
    #[cfg(test)]
    pub(crate) use serde_json::json;
    pub(crate) use std::collections::{HashMap, HashSet};
    pub(crate) use zeroize::*;
    #[cfg(debug_assertions)]
    pub(crate) use hash::Hash;
}

pub use prelude::*;
