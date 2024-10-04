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

impl<V: Debug + PartialEq + Eq + Clone + sargon::Identifiable> Default
    for IdentifiedVecOf<V>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<V> Hash for IdentifiedVecOf<V>
where
    V: Debug + PartialEq + Eq + Clone + sargon::Identifiable + Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<V: Debug + PartialEq + Eq + Clone + sargon::Identifiable> Index<usize>
    for IdentifiedVecOf<V>
{
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<V: Debug + PartialEq + Eq + Clone + sargon::Identifiable>
    From<IndexMap<<V as sargon::Identifiable>::ID, V>> for IdentifiedVecOf<V>
{
    fn from(value: IndexMap<<V as sargon::Identifiable>::ID, V>) -> Self {
        Self(InternalIden)
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
