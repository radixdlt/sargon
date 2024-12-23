use radix_rust::prelude::IndexSet;
use std::borrow::Borrow;

use crate::prelude::*;

impl<V: Debug + PartialEq + Eq + Clone + Identifiable> IdentifiedVecOf<V> {
    /// The number of items in this collection.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// If this collection is empty or not
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the ids of the items.
    pub fn ids(&self) -> IndexSet<&<V as Identifiable>::ID> {
        IndexSet::from_iter(self.0.keys())
    }

    /// Check if the `item` exists in this map by calculating the ID of the item
    /// and checking if any other item with the same ID exists.
    pub fn contains_by_id(&self, item: &V) -> bool {
        self.contains_id(item.id())
    }

    /// Return `true`` if an item with `id` exists in the collection.
    pub fn contains_id(&self, id: impl Borrow<V::ID>) -> bool {
        self.0.contains_key(id.borrow())
    }

    /// Get an item by index
    ///
    /// Valid indices are `0 <= index < self.len()``
    ///
    /// Computes in **O(1)** time.
    pub fn get_at_index(&self, index: usize) -> Option<&V> {
        self.0.get_index(index).map(|pair| pair.1)
    }

    /// Return a reference to the item, if it is present, else `None``.
    ///
    /// Computes in **O(1)** time (average).
    pub fn get_id(&self, id: impl Borrow<V::ID>) -> Option<&V> {
        self.0.get(id.borrow())
    }

    /// Return a Vec of references to the items of the collection, in their order.
    pub fn get_all(&self) -> Vec<&V> {
        self.0.values().collect_vec()
    }

    /// Returns the item at index 0 in this [`IdentifiedVecOf`] if any exists.
    pub fn first(&self) -> Option<&V> {
        self.get_at_index(0)
    }

    /// Return a Vec with items of the collection, in their order.
    pub fn items(&self) -> Vec<V> {
        self.into_iter().collect_vec()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::identified_vec_of::User;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = IdentifiedVecOf<User>;

    #[test]
    fn get_id() {
        let sut = SUT::sample();
        assert_eq!(sut.get_id(0), Some(&User::alice()));
        assert_eq!(
            sut.get_id(2), /* Can also omit & */
            Some(&User::carol())
        );
        assert_eq!(sut.get_id(200), None);
    }

    #[test]
    fn get_index() {
        let sut = SUT::sample();
        assert_eq!(sut.get_at_index(0), Some(&User::alice()));
        assert_eq!(sut.get_at_index(1), Some(&User::carol()));
        assert_eq!(sut.get_at_index(999), None);
    }

    #[test]
    fn len() {
        assert_eq!(SUT::sample().len(), 4);
        assert_eq!(SUT::sample_other().len(), 3);
        assert_eq!(SUT::new().len(), 0);
    }

    #[test]
    fn is_empty() {
        assert!(!SUT::sample().is_empty());
        assert!(!SUT::sample_other().is_empty());
        assert!(SUT::new().is_empty());
    }

    #[test]
    fn contains_id() {
        let sut = SUT::sample();
        assert!(sut.contains_id(0));
        assert!(!sut.contains_id(200));
    }

    #[test]
    fn contains_by_id() {
        let sut = SUT::sample();
        assert!(sut.contains_by_id(&User::alice()));
        assert!(!sut.contains_by_id(&User::frank()));
    }

    #[test]
    fn items() {
        let sut = SUT::sample_other();
        assert_eq!(sut.items(), vec![User::bob(), User::david(), User::frank()])
    }

    #[test]
    fn get_all() {
        let sut = SUT::sample_other();
        assert_eq!(
            sut.get_all(),
            vec![&User::bob(), &User::david(), &User::frank()]
        )
    }
}
