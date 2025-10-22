use crate::prelude::*;

use std::borrow::Borrow;

impl<V: Debug + PartialEq + Eq + Clone + Identifiable> IdentifiedVecOf<V> {
    /// Insert an item in the map **unconditionally**, using `id()` on item as key.
    ///
    /// If an equivalent key already exists in the map: the key remains and
    /// retains in its place in the order, its corresponding value is updated
    /// with `value` and the older value is returned inside `Some(_)`.
    ///
    /// If no equivalent key existed in the map: the new key-value pair is
    /// inserted, last in order, and `None` is returned.
    ///
    /// Computes in **O(1)** time (amortized average).
    ///
    /// See also [`entry`](#method.entry) if you you want to insert *or* modify
    /// or if you need to get the index of the corresponding key-value pair.
    pub fn insert(&mut self, item: V) -> Option<V> {
        self.0.insert(item.id(), item)
    }

    /// Insert the `item` **if and only if** no item with the same ID already
    /// exists, if exists, an error is thrown.
    pub fn try_insert_unique(&mut self, item: V) -> Result<()> {
        if self.contains_by_id(&item) {
            return Err(CommonError::IdentifiableItemAlreadyExist {
                id: format!("{:?}", item.id()),
            });
        }
        assert!(self.insert(item).is_none());
        Ok(())
    }

    /// Inserts `item` at `index`, if it already exists then the item remains
    /// but **is moved to the new position** in the collection, and the item is
    /// updated with value, and the older value is returned inside `Some(_)`.
    ///
    /// If no equivalent items existed: the new item is inserted at the given index,
    /// and `None` is returned.
    ///
    /// #Panic
    /// Panics if index is out of bounds.
    pub fn insert_at(&mut self, item: V, index: usize) -> Option<V> {
        self.0.shift_insert(index, item.id(), item)
    }

    /// Append a new member to the end of the `IdentifiedVecOf`, if the `IdentifiedVecOf` doesn't already contain it.
    ///
    /// - Parameter item: The element to add to the `IdentifiedVecOf`.
    /// - Returns: A pair `(inserted, index)`, where `inserted` is a Boolean value indicating whether
    ///   the operation added a new element, and `index` is the index of `item` in the resulting
    ///   `IdentifiedVecOf`.
    /// - Complexity: The operation is expected to perform O(1) copy, hash, and compare operations on
    ///   the `ID` type, if it implements high-quality hashing.
    pub fn append(&mut self, item: V) -> (bool, usize) {
        if let Some(existing) = self.0.get_full(&item.id()) {
            return (false, existing.0);
        }
        assert!(self.insert(item).is_none());
        (true, self.len() - 1)
    }

    pub fn extend(&mut self, iter: impl IntoIterator<Item = V>) {
        self.0.extend(iter.into_iter().map(|i| (i.id(), i)))
    }

    /// Remove and return the item with the `id`.
    ///
    /// Like `Vec::remove``, the item is removed by shifting all of the elements
    /// that follow it, preserving their relative order.
    /// **This perturbs the index of all of those elements!**.
    ///
    /// Return `None` if no item with `id` is in the collection.
    ///
    /// Computes in `O(n)` time (average).
    pub fn remove_id(&mut self, id: &V::ID) -> Option<V> {
        self.0.shift_remove_entry(id).map(|pair| pair.1)
    }

    /// Returns `false` if no element of `id` was found, otherwise if found, this
    /// existing element gets updated by `mutate` closure and this function returns
    /// `true`.
    #[inline]
    pub fn update_with<F>(
        &mut self,
        id: impl Borrow<V::ID>,
        mut mutate: F,
    ) -> bool
    where
        F: FnMut(&mut V),
    {
        let Some(existing) = self.0.get_mut(id.borrow()) else {
            return false;
        };
        mutate(existing);
        true
    }

    /// Updates in place each item by `mutate`.
    #[inline]
    pub fn update_all_with<F>(&mut self, mut mutate: F)
    where
        F: FnMut(&mut V),
    {
        for item in self.0.values_mut() {
            mutate(item)
        }
    }

    /// Tries to mutate the value identified by `id`, if no such value exists
    /// an error is returned, the mutation is failable, if your return an `Err`
    /// in `mutate`, this method propagates that error.
    #[inline]
    pub fn try_try_update_with<F>(
        &mut self,
        id: &V::ID,
        mut mutate: F,
    ) -> Result<()>
    where
        F: FnMut(&mut V) -> Result<()>,
    {
        let Some(existing) = self.0.get_mut(id) else {
            return Err(CommonError::ElementDoesNotExist {
                id: format!("{:?}", id),
            });
        };
        mutate(existing)?;
        Ok(())
    }

    /// Tries to mutate the value identified by `id`, if no such value exists
    /// an error is returned, the mutation is failable, if your return an `Err`
    /// in `mutate`, this method propagates that error.
    #[inline]
    pub fn try_update_with<F>(
        &mut self,
        id: &V::ID,
        mut mutate: F,
    ) -> Result<()>
    where
        F: FnMut(&mut V),
    {
        let Some(existing) = self.0.get_mut(id) else {
            return Err(CommonError::ElementDoesNotExist {
                id: format!("{:?}", id),
            });
        };
        mutate(existing);
        Ok(())
    }

    /// Tries to mutate the value identified by `id`, if no such value exists
    /// then `Ok(false)` is returned, false meaning "no, not found", the mutation
    /// is failable, if your return an `Err` in `mutate`, this method propagates that error.
    #[inline]
    pub fn maybe_update_with<F>(
        &mut self,
        id: &V::ID,
        mut mutate: F,
    ) -> Result<bool>
    where
        F: FnMut(&mut V) -> Result<V>,
    {
        let Some(existing) = self.0.get_mut(id) else {
            return Ok(false);
        };
        let mutated = mutate(existing)?;
        *existing = mutated;
        Ok(true)
    }

    pub fn update_items(
        &mut self,
        items: impl IntoIterator<Item = V>,
    ) -> Result<()> {
        let backup = self.clone();
        match self.update_items_with_outcome(items) {
            IdentifiableVecOfUpdateItemsOutcome::NoNewItemInserted => Ok(()),
            IdentifiableVecOfUpdateItemsOutcome::NewItemInserted => {
                *self = backup;
                Err(CommonError::Unknown {
                    error_message: "Unexpected outcome: new item inserted"
                        .to_string(),
                })
            }
        }
    }
    fn update_items_with_outcome(
        &mut self,
        items: impl IntoIterator<Item = V>,
    ) -> IdentifiableVecOfUpdateItemsOutcome {
        let len_before = self.len();
        let items = items.into_iter().map(|i| (i.id(), i)).collect_vec();
        self.0.extend(items);
        let len_after = self.len();
        if len_before < len_after {
            IdentifiableVecOfUpdateItemsOutcome::NewItemInserted
        } else {
            IdentifiableVecOfUpdateItemsOutcome::NoNewItemInserted
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentifiableVecOfUpdateItemsOutcome {
    /// We only updated existing items, no new item were inserted
    NoNewItemInserted,
    /// Some of the items were not known, they were newly inserted
    NewItemInserted,
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::User;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = IdentifiedVecOf<User>;

    #[test]
    fn append_existing_is_noop() {
        let mut sut = SUT::sample();
        assert_eq!(sut.append(User::grace()), (false, 3))
    }

    #[test]
    fn append_existing_id_different_content_is_noop() {
        let mut sut = SUT::sample();
        assert_eq!(sut.append(User::new(User::grace().id, "Gemma")), (false, 3))
    }

    #[test]
    fn update_items_new_items() {
        let mut sut = SUT::sample_other();
        let sut_backup = sut.clone();
        let res = sut.update_items(vec![User::carol(), User::david()]);
        assert_eq!(
            res,
            Err(CommonError::Unknown {
                error_message: "Unexpected outcome: new item inserted"
                    .to_string()
            })
        );
        assert_eq!(sut, sut_backup); // not changed
    }

    #[test]
    fn append_new() {
        let mut sut = SUT::sample_other();
        assert_eq!(sut.append(User::carol()), (true, 3));
        assert_eq!(sut.to_string(), "[Bob, David, Frank, Carol]");
    }

    #[test]
    fn insert_by_id_new() {
        let mut sut = SUT::sample_other();
        assert_eq!(sut.insert(User::carol()), None);
        assert_eq!(sut.to_string(), "[Bob, David, Frank, Carol]");
    }

    #[test]
    fn insert_by_id_existing_is_updated() {
        let mut sut = SUT::sample_other();
        assert_eq!(
            sut.insert(User::new(User::bob().id, "Barry")),
            Some(User::bob())
        );
        assert_eq!(sut.to_string(), "[Barry, David, Frank]");
    }

    #[test]
    #[should_panic]
    fn insert_at_out_of_bounds() {
        let mut sut = SUT::sample();
        let _ = sut.insert_at(User::bob(), 999);
    }

    #[test]
    fn insert_existing_new_index() {
        let mut sut = SUT::sample_other();
        let _ = sut.insert_at(User::bob(), 2);
        assert_eq!(sut.to_string(), "[David, Frank, Bob]");
    }

    #[test]
    fn insert_at_index_0_new() {
        let mut sut = SUT::sample_other();
        let _ = sut.insert_at(User::alice(), 0);
        assert_eq!(sut.to_string(), "[Alice, Bob, David, Frank]");
    }

    #[test]
    fn insert_at_index_1_new() {
        let mut sut = SUT::sample_other();
        let _ = sut.insert_at(User::alice(), 1);
        assert_eq!(sut.to_string(), "[Bob, Alice, David, Frank]");
    }

    #[test]
    fn insert_at_index_last_new() {
        let mut sut = SUT::sample_other();
        let _ = sut.insert_at(User::alice(), 3);
        assert_eq!(sut.to_string(), "[Bob, David, Frank, Alice]");
    }

    #[test]
    fn try_inserting_unique_duplicate() {
        let mut sut = SUT::sample();
        assert_eq!(
            sut.try_insert_unique(User::grace()),
            Err(CommonError::IdentifiableItemAlreadyExist {
                id: "6".to_owned()
            })
        );
    }

    #[test]
    fn update_with_for_existing() {
        let foobar = User::new(0, "Foobar");
        let mut sut = SUT::sample();
        assert_eq!(sut.get_id(0), Some(&User::alice()));
        assert!(sut.update_with(0, |u| { *u = foobar.clone() }));
        assert_eq!(sut.get_id(0), Some(&foobar));
    }

    #[test]
    fn update_with_not_exists() {
        let mut sut = SUT::sample();
        assert!(!sut.update_with(1, |u| { *u = User::bob() }));
    }

    #[test]
    fn test_try_try_update_with_succeeds() {
        let foobar = User::new(0, "Foobar");
        let mut sut = SUT::sample();
        assert_eq!(sut.get_id(0), Some(&User::alice()));
        assert!(sut
            .try_try_update_with(&0, |u| {
                *u = foobar.clone();
                Ok(())
            })
            .is_ok());
        assert_eq!(sut.get_id(0), Some(&foobar));
    }

    #[test]
    fn test_try_try_update_with_not_exists() {
        let mut sut = SUT::sample();
        assert_eq!(
            sut.try_try_update_with(&1, |u| {
                *u = User::bob();
                Ok(())
            }),
            Err(CommonError::ElementDoesNotExist { id: "1".to_owned() })
        );
    }

    #[test]
    fn test_try_update_with_success() {
        let foobar = User::new(0, "Foobar");
        let mut sut = SUT::sample();
        assert_eq!(sut.get_id(0), Some(&User::alice()));
        assert!(sut.try_update_with(&0, |u| { *u = foobar.clone() }).is_ok());
        assert_eq!(sut.get_id(0), Some(&foobar));
    }

    #[test]
    fn test_try_update_with_not_exists() {
        let mut sut = SUT::sample();
        assert_eq!(
            sut.try_update_with(&1, |u| { *u = User::bob() }),
            Err(CommonError::ElementDoesNotExist { id: "1".to_owned() })
        );
    }
}
