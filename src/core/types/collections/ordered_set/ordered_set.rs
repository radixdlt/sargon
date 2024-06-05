use crate::prelude::*;

/// An set of values preserving insertion order.
///
/// This is useful over `IdentifiedVecOf` when there is no natural
/// `ID` to use, e.g. when the items themselves are IDs.
///
/// Is `UniFFI` compatible (as Vec), is `Serde` compatible if `V` is.
///
/// The implementation is using an `IndexSet`,
/// and we have implemented `std::hash::Hash` for `OrderedSet`.
#[derive(PartialEq, Eq, Clone)]
pub struct OrderedSet<V: std::hash::Hash + PartialEq + Eq + Clone>(
    pub(super) IndexSet<V>,
);

impl<V: std::hash::Hash + PartialEq + Eq + Clone> OrderedSet<V> {
    pub fn new() -> Self {
        Self::from(IndexSet::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
