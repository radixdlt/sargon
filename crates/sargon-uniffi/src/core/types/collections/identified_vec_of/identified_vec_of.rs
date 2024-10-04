use radix_rust::prelude::{IndexMap, IndexSet};

use crate::prelude::*;

use std::{
    any::TypeId,
    fmt::{Debug, Display, Formatter},
    hash::Hasher,
    ops::Index,
};
use std::{hash::Hash, ops::DerefMut};
use sargon::IdentifiedVecOf as InternalIdentifiedVecOf;

/// A collection which **retains the insertion order** of its **unique** [`Identifiable`]
/// items, with **constant time** look up of an item by its `id` - a stable key
/// which instances of the `Item` itself can calculate.
///
/// The implementation is
#[derive(Clone, PartialEq, Eq)]
pub struct IdentifiedVecOf<V: Debug + PartialEq + Eq + Clone + sargon::Identifiable>(
    pub(super) InternalIdentifiedVecOf<V>,
);