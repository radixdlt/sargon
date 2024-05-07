use radix_engine::types::indexmap::map::Values;
use uniffi::{
    check_remaining,
    deps::bytes::{Buf, BufMut},
    metadata, Lift, Lower, LowerReturn, MetadataBuffer, RustBuffer,
};

use crate::prelude::*;

use std::{fmt::Debug, hash::Hasher};
use std::{hash::Hash, ops::DerefMut};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderedMap<V: Debug + PartialEq + Eq + Clone + Identifiable>(
    IndexMap<<V as Identifiable>::ID, V>,
);

impl<V: Debug + PartialEq + Eq + Clone + Identifiable> OrderedMap<V> {
    pub fn ids(&self) -> IndexSet<&<V as Identifiable>::ID> {
        IndexSet::from_iter(self.keys().into_iter())
    }

    /// Insert an item in the map, using `id()` on item as key.
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
}

impl<V: Debug + PartialEq + Eq + Clone + Identifiable> Default
    for OrderedMap<V>
{
    fn default() -> Self {
        Self::from(IndexMap::new())
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
