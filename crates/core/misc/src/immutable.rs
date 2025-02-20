use crate::prelude::*;
use std::ops::Deref;

// From https://stackoverflow.com/a/62948428

/// A container for values that can only be deref'd immutably.
#[derive(
    Clone,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Debug,
)]
#[debug(bound(T: std::fmt::Debug))]
#[debug("{:?}", self.value)]
#[serde(transparent)]

pub struct Immutable<T> {
    value: T,
}

impl<T> From<T> for Immutable<T> {
    fn from(value: T) -> Self {
        Immutable::new(value)
    }
}

impl<T> Immutable<T> {
    pub fn new(value: T) -> Self {
        Immutable { value }
    }
}

impl<T> Deref for Immutable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
