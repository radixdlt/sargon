use crate::prelude::*;
use std::ops::Deref;

// From https://stackoverflow.com/a/62948428

/// A container for values that can only be deref'd immutably.
#[derive(
    Clone,
    Default,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
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
