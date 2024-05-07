use indexmap::{IndexMap, IndexSet};
use uniffi::{
    check_remaining,
    deps::bytes::{Buf, BufMut},
    metadata, Lift, Lower, LowerReturn, MetadataBuffer, RustBuffer,
};

use crate::prelude::*;

use std::{
    fmt::{Debug, Display, Formatter},
    hash::Hasher,
};
use std::{hash::Hash, ops::DerefMut};

/// The `Identifiable` trait allows you to use the
/// `IdentifiedVecOf<User> instead of the more verbose
/// `IdentifiedVec<SomeUserID, User>` but also allows you to
/// skip the `id_of_element: fn(&Element) -> ID` closure when
/// initializing a new identified vec.
pub trait Identifiable {
    /// The type that your `Element` will use as its globally unique and stable ID,
    /// must impl `Hash` since it is used as a key in `IdentifiedVecOf`'s internal
    /// `HashMap`. Must impl `Clone` since we need to be able to clone it as a key
    type ID: Eq + Hash + Clone + Debug;

    /// Return `Element`'s globally unique and stable ID, used to uniquely identify
    /// the `Element` in the `IdentifiedVecOf` collection of elements.
    fn id(&self) -> Self::ID;
}

#[derive(Clone, PartialEq, Eq)]
pub struct OrderedMap<V: Debug + PartialEq + Eq + Clone + Identifiable>(
    IndexMap<<V as Identifiable>::ID, V>,
);

impl<V: Debug + PartialEq + Eq + Clone + Identifiable> OrderedMap<V> {
    pub fn ids(&self) -> IndexSet<&<V as Identifiable>::ID> {
        IndexSet::from_iter(self.keys())
    }

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

    pub fn insert_at(&mut self, item: V, index: usize) -> Option<V> {
        self.0.shift_insert(index, item.id(), item)
    }

    /// Append a new member to the end of the `OrderedMap`, if the `OrderedMap` doesn't already contain it.
    ///
    /// - Parameter item: The element to add to the `OrderedMap`.
    /// - Returns: A pair `(inserted, index)`, where `inserted` is a Boolean value indicating whether
    ///   the operation added a new element, and `index` is the index of `item` in the resulting
    ///   `OrderedMap`.
    /// - Complexity: The operation is expected to perform O(1) copy, hash, and compare operations on
    ///   the `ID` type, if it implements high-quality hashing.
    pub fn append(&mut self, item: V) -> (bool, usize) {
        if let Some(existing) = self.0.get_full(&item.id()) {
            return (false, existing.0);
        }
        assert!(self.insert(item).is_none());
        (true, self.len())
    }

    pub fn new() -> Self {
        Self::from(IndexMap::new())
    }

    pub fn first(&self) -> Option<&V> {
        self.0.first().map(|pair| pair.1)
    }

    pub fn just(item: V) -> Self {
        Self::from_iter([item])
    }

    /// Check if the `item` exists in this map by calculating
    /// the ID of the item and checking if any other item with
    /// the same ID exists.
    pub fn contains(&self, item: &V) -> bool {
        self.contains_id(&item.id())
    }

    pub fn contains_id(&self, id: &V::ID) -> bool {
        (*self).contains_key(id)
    }

    pub fn get_at_index(&self, index: usize) -> Option<&V> {
        (*self).get_index(index).map(|pair| pair.1)
    }

    pub fn get_id(&self, id: &V::ID) -> Option<&V> {
        (*self).get(id)
    }

    pub fn get_all(&self) -> Vec<&V> {
        (*self).values().collect_vec()
    }

    pub fn items(&self) -> Vec<V> {
        self.into_iter().collect_vec()
    }

    pub fn remove_id(&mut self, id: &V::ID) -> Option<V> {
        (*self).shift_remove_entry(id).map(|pair| pair.1)
    }

    pub fn remove(&mut self, value: &V) -> Option<V> {
        (*self).remove_id(&value.id())
    }

    /// Returns `false` if no element of `id` was found, otherwise if found, this
    /// existing element gets updated by `mutate` closure and this function returns
    /// `true`.
    #[inline]
    pub fn update_with<F>(&mut self, id: &V::ID, mut mutate: F) -> bool
    where
        F: FnMut(&mut V),
    {
        let Some(existing) = (*self).get_mut(id) else {
            return false;
        };
        mutate(existing);
        true
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
        F: FnMut(&mut V) -> Result<V>,
    {
        let Some(existing) = (*self).get_mut(id) else {
            return Err(CommonError::ElementDoesNotExist {
                id: format!("{:?}", id),
            });
        };
        let mutated = mutate(existing)?;
        *existing = mutated;
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
        let Some(existing) = (*self).get_mut(id) else {
            return Ok(false);
        };
        let mutated = mutate(existing)?;
        *existing = mutated;
        Ok(true)
    }

    #[inline]
    pub fn iter(&self) -> OrderedMapIterator<V> {
        OrderedMapIterator {
            ordered_map: self,
            index: 0,
        }
    }
}

// ===============
// where V: Display
// ===============
impl<V> Display for OrderedMap<V>
where
    V: Debug + PartialEq + Eq + Clone + Identifiable + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())?;
        Ok(())
    }
}

impl<V> OrderedMap<V>
where
    V: Debug + PartialEq + Eq + Clone + Identifiable + Display,
{
    fn description(&self) -> String {
        [
            "[".to_owned(),
            self.clone()
                .into_iter()
                .map(|e| format!("{}", e))
                .join(", "),
            "]".to_owned(),
        ]
        .join("")
    }
}

// ===============
// where V: Debug
// ===============
impl<V> Debug for OrderedMap<V>
where
    V: Debug + PartialEq + Eq + Clone + Identifiable,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.debug_description())?;
        Ok(())
    }
}

impl<V> OrderedMap<V>
where
    V: Debug + PartialEq + Eq + Clone + Identifiable,
{
    fn debug_description(&self) -> String {
        [
            "[".to_owned(),
            self.clone()
                .into_iter()
                .map(|e| format!("{:?}", e))
                .join(", "),
            "]".to_owned(),
        ]
        .join("")
    }
}

impl<V: Debug + PartialEq + Eq + Clone + Identifiable> Default
    for OrderedMap<V>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<V> Hash for OrderedMap<V>
where
    V: Debug + PartialEq + Eq + Clone + Identifiable + Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        for key in self.keys() {
            key.hash(state);
        }
    }
}

impl<V: Debug + PartialEq + Eq + Clone + Identifiable> Serialize
    for OrderedMap<V>
where
    V: Serialize,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self)
    }
}

impl<'de, V: Debug + PartialEq + Eq + Clone + Identifiable> Deserialize<'de>
    for OrderedMap<V>
where
    V: Deserialize<'de>,
{
    #[inline]
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        let items = Vec::<V>::deserialize(deserializer)?;
        Ok(Self::from_iter(items))
    }
}

impl<V: Debug + PartialEq + Eq + Clone + Identifiable> Deref for OrderedMap<V> {
    type Target = IndexMap<<V as Identifiable>::ID, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<V: Debug + PartialEq + Eq + Clone + Identifiable> DerefMut
    for OrderedMap<V>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<V: Debug + PartialEq + Eq + Clone + Identifiable> FromIterator<V>
    for OrderedMap<V>
{
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        let mut map = IndexMap::<<V as Identifiable>::ID, V>::new();
        for item in iter {
            let _ = map.insert(item.id(), item);
        }
        Self::from(map)
    }
}

impl<'a, V: Debug + PartialEq + Eq + Clone + Identifiable> IntoIterator
    for &'a OrderedMap<V>
{
    type Item = V;
    type IntoIter = OrderedMapIterator<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        OrderedMapIterator {
            ordered_map: self,
            index: 0,
        }
    }
}

pub struct OrderedMapIterator<
    'a,
    V: Debug + PartialEq + Eq + Clone + Identifiable,
> {
    ordered_map: &'a OrderedMap<V>,
    index: usize,
}

impl<'a, V: Debug + PartialEq + Eq + Clone + Identifiable> Iterator
    for OrderedMapIterator<'a, V>
{
    type Item = V;
    fn next(&mut self) -> Option<V> {
        if self.index < self.ordered_map.len() {
            let elem = self.ordered_map.get_index(self.index);
            self.index += 1;
            elem.map(|pair| pair.1.clone())
        } else {
            None
        }
    }
}

impl<V: Debug + PartialEq + Eq + Clone + Identifiable>
    From<IndexMap<<V as Identifiable>::ID, V>> for OrderedMap<V>
{
    fn from(value: IndexMap<<V as Identifiable>::ID, V>) -> Self {
        Self(value)
    }
}

// We turn an `[Rust] IndexMap -> Array/List [FFI]``

unsafe impl<UT, V: Debug + Hash + Eq + Clone + Identifiable + Lower<UT>>
    Lower<UT> for OrderedMap<V>
{
    type FfiType = RustBuffer;

    fn write(obj: Self, buf: &mut Vec<u8>) {
        let len = i32::try_from(obj.len()).unwrap();
        buf.put_i32(len); // We limit arrays to i32::MAX items
        for value in &obj {
            <V as Lower<UT>>::write(value, buf);
        }
    }

    fn lower(obj: Self) -> RustBuffer {
        Self::lower_into_rust_buffer(obj)
    }

    const TYPE_ID_META: MetadataBuffer =
        MetadataBuffer::from_code(metadata::codes::TYPE_VEC)
            .concat(V::TYPE_ID_META);
}

unsafe impl<UT, V: Debug + Hash + Eq + Clone + Identifiable + Lower<UT>>
    LowerReturn<UT> for OrderedMap<V>
{
    type ReturnType = <Self as Lower<UT>>::FfiType;

    fn lower_return(obj: Self) -> uniffi::Result<Self::ReturnType, RustBuffer> {
        Ok(<Self as Lower<UT>>::lower(obj))
    }

    const TYPE_ID_META: MetadataBuffer = <Self as Lower<UT>>::TYPE_ID_META;
}
unsafe impl<UT, V: Debug + Hash + Eq + Clone + Identifiable + Lift<UT>> Lift<UT>
    for OrderedMap<V>
{
    type FfiType = RustBuffer;

    fn try_read(buf: &mut &[u8]) -> uniffi::Result<Self> {
        check_remaining(buf, 4)?;
        let len = usize::try_from(buf.get_i32())?;
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(<V as Lift<UT>>::try_read(buf)?)
        }
        Ok(<Self as FromIterator<V>>::from_iter(vec))
    }

    fn try_lift(buf: RustBuffer) -> uniffi::Result<Self> {
        Self::try_lift_from_rust_buffer(buf)
    }

    const TYPE_ID_META: MetadataBuffer =
        MetadataBuffer::from_code(metadata::codes::TYPE_VEC)
            .concat(V::TYPE_ID_META);
}
