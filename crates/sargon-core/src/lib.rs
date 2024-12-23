#![feature(core_intrinsics)]

mod assert_json;
mod assert_network_request;
mod error;
mod has_sample_values;
mod hash;
mod secure_random_bytes;
mod types;
mod unsafe_id_stepper;
mod utils;

pub mod prelude {
    pub use crate::assert_json::*;
    pub use crate::assert_network_request::*;
    pub use crate::error::*;
    pub use crate::has_sample_values::*;
    pub use crate::hash::*;
    pub use crate::secure_random_bytes::*;
    pub use crate::types::*;
    pub use crate::unsafe_id_stepper::*;
    pub use crate::utils::*;

    pub use radix_rust::prelude::{
        indexmap, BTreeSet, HashMap, HashSet, IndexMap, IndexSet,
    };
    pub use std::marker::PhantomData;

    pub use ::hex::decode as hex_decode;
    pub use ::hex::encode as hex_encode;
    pub use iso8601_timestamp::Timestamp;
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
    // pub use futures::future::join_all;
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
}
