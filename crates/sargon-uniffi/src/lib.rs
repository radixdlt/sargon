#![feature(async_closure)]
#![feature(let_chains)]
#![feature(core_intrinsics)]
#![allow(unused_imports)]
#![allow(internal_features)]

mod core;
mod hierarchical_deterministic;
mod home_cards;
mod keys_collector;
mod profile;
mod radix_connect;
mod security_center;
mod signing;
mod system;
mod types;
mod wrapped_radix_engine_toolkit;

pub fn apa() -> u8 { 0 }

pub mod prelude {

    pub use crate::core::*;
    pub use crate::hierarchical_deterministic::*;
    pub use crate::home_cards::*;
    pub use crate::keys_collector::*;
    pub use crate::profile::*;
    pub use crate::radix_connect::*;
    pub use crate::security_center::*;
    pub use crate::signing::*;
    pub use crate::system::*;
    pub use crate::types::*;
    pub use crate::wrapped_radix_engine_toolkit::*;

    pub use radix_engine_toolkit::functions::{
        transaction_v1::manifest::{
            from_payload_bytes as RET_from_payload_bytes_manifest_v1,
            to_payload_bytes as RET_to_payload_bytes_manifest_v1,
        },
        transaction_v2::{
            subintent_manifest::{
                from_payload_bytes as RET_from_payload_bytes_subintent_manifest,
                to_payload_bytes as RET_to_payload_bytes_subintent_manifest,
            },
            transaction_manifest::{
                from_payload_bytes as RET_from_payload_bytes_manifest_v2,
                to_payload_bytes as RET_to_payload_bytes_manifest_v2,
            },
        },
    };

    pub(crate) use sargon_uniffi_conversion_macros::*;

    pub(crate) use sargon::prelude::{
        DeserializeBytes, DeserializeStr, HasSampleValues, HashMap, HashSet,
        SerializeToBytes, SerializeToString,
    };

    pub(crate) use ::hex::decode as hex_decode;
    pub(crate) use ::hex::encode as hex_encode;
    pub(crate) use iso8601_timestamp::Timestamp;
    pub(crate) use itertools::Itertools;
    pub(crate) use log::{debug, error, info, trace, warn};

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
    pub(crate) use enum_iterator::all;
    pub(crate) use paste::*;
}

pub use prelude::*;

pub use url::Url;

// `Url` gets converted to a `String` to pass across the FFI.
uniffi::custom_type!(Url, String, {
    // Remote is required since `Url` is from a different crate
    remote,
    try_lift: |val| Ok(Url::parse(&val)?),
    lower: |obj| obj.into(),
});

// `Timestamp` gets converted to a `String` to pass across the FFI.
uniffi::custom_type!(Timestamp, String, {
    // Remote is required since `Timestamp` is from a different crate
    remote,
    try_lift: |val| {
        Timestamp::parse(val.as_str())
        .ok_or(CommonError::InvalidISO8601String { bad_value: val })
        .map_err(|e| e.into())
    },
    lower: |obj| obj.to_string(),
});

// `Uuid` gets converted to a `String` to pass across the FFI.
uniffi::custom_type!(Uuid, String, {
    // Remote is required since `Uuid` is from a different crate
    remote,
    try_lift: |val|  Ok(Uuid::try_parse(val.as_str())?),
    lower: |obj| obj.to_string(),
});

uniffi::include_scaffolding!("sargon");
