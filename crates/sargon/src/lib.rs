#![feature(async_closure)]
#![feature(let_chains)]
#![feature(core_intrinsics)]
#![allow(unused_imports)]
#![allow(internal_features)]

mod core;
mod gateway_api;
mod hierarchical_deterministic;
mod profile;
mod radix_connect;
mod system;
mod wrapped_radix_engine_toolkit;

pub mod prelude {

    pub use crate::core::*;
    pub use crate::gateway_api::*;
    pub use crate::hierarchical_deterministic::*;
    pub use crate::profile::*;
    pub use crate::radix_connect::*;
    pub use crate::system::*;
    pub use crate::wrapped_radix_engine_toolkit::*;

    pub(crate) use radix_rust::prelude::{
        BTreeSet, HashMap, HashSet, IndexMap, IndexSet,
    };

    pub(crate) use hex::decode as hex_decode;
    pub(crate) use hex::encode as hex_encode;
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

    pub use radix_common::math::traits::CheckedMul as ScryptoCheckedMul;
    pub(crate) use std::cmp::Ordering;
    pub(crate) use std::collections::BTreeMap;
    pub(crate) use std::fmt::{Debug, Display, Formatter};
    pub(crate) use std::fs;
    pub(crate) use std::hash::Hash as StdHash;
    pub use std::ops::{Add, AddAssign, Deref, Div, Mul, Neg, Sub};
    pub(crate) use std::str::FromStr;
    pub(crate) use std::sync::{Arc, RwLock};

    pub(crate) use strum::FromRepr;
    pub(crate) use url::Url;
    pub(crate) use uuid::Uuid;

    pub(crate) use enum_as_inner::EnumAsInner;
    pub(crate) use paste::*;
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