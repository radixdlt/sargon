use radix_rust::prelude::{IndexMap, IndexSet};

use crate::prelude::*;

use std::{
    any::TypeId,
    fmt::{Debug, Display, Formatter},
    hash::Hasher,
    ops::Index,
};
use std::{hash::Hash, ops::DerefMut};

/// A collection which **retains the insertion order** of its **unique** [`Identifiable`]
/// items, with **constant time** look up of an item by its `id` - a stable key
/// which instances of the `Item` itself can calculate.
///
/// The implementation is
#[derive(Clone, PartialEq, Eq)]
pub struct IdentifiedVecOf<V: Debug + PartialEq + Eq + Clone + Identifiable>(
    pub(super) IndexMap<<V as Identifiable>::ID, V>,
);

impl<V: Debug + PartialEq + Eq + Clone + Identifiable> IdentifiedVecOf<V> {
    /// Creates a new empty `IdentifiedVecOf`.
    pub fn new() -> Self {
        Self::from(IndexMap::new())
    }

    /// Creates a new `IdentifiedVecOf` with one single item.
    pub fn just(item: V) -> Self {
        Self::from_iter([item])
    }
}

impl<V: Debug + PartialEq + Eq + Clone + Identifiable> From<Vec<V>> for IdentifiedVecOf<V> {
    fn from(value: Vec<V>) -> Self {
        Self::from_iter(value)
    }
}

impl<V: Debug + PartialEq + Eq + Clone + Identifiable> Default
    for IdentifiedVecOf<V>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<V> Hash for IdentifiedVecOf<V>
where
    V: Debug + PartialEq + Eq + Clone + Identifiable + Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        for key in self.0.values() {
            key.hash(state);
        }
    }
}

impl<V: Debug + PartialEq + Eq + Clone + Identifiable> Index<usize>
    for IdentifiedVecOf<V>
{
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<V: Debug + PartialEq + Eq + Clone + Identifiable>
    From<IndexMap<<V as Identifiable>::ID, V>> for IdentifiedVecOf<V>
{
    fn from(value: IndexMap<<V as Identifiable>::ID, V>) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {

    use super::super::super::User;
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = IdentifiedVecOf<User>;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn index() {
        let sut = SUT::sample();
        assert_eq!(sut[0], User::alice());
        assert_eq!(sut[1], User::carol());
    }
}
