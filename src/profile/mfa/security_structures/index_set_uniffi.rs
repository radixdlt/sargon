use crate::prelude::*;
use std::any::TypeId as StdTypeId;
use uniffi::TypeId as UFTypeId;
use uniffi::{
    check_remaining,
    deps::bytes::{Buf, BufMut},
    metadata, Lift, Lower, LowerReturn, MetadataBuffer, RustBuffer,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OrderedSet<V: Debug + std::hash::Hash + PartialEq + Eq + Clone>(
    IndexSet<V>,
);
impl<V: Debug + std::hash::Hash + PartialEq + Eq + Clone> From<IndexSet<V>>
    for OrderedSet<V>
{
    fn from(value: IndexSet<V>) -> Self {
        Self(value)
    }
}
impl<V: Debug + std::hash::Hash + PartialEq + Eq + Clone> std::ops::Deref
    for OrderedSet<V>
{
    type Target = IndexSet<V>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<V: Debug + PartialEq + Eq + Clone + std::hash::Hash + 'static> Serialize
    for OrderedSet<V>
where
    V: Serialize,
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, V: Debug + PartialEq + Eq + Clone + std::hash::Hash + 'static>
    Deserialize<'de> for OrderedSet<V>
where
    V: Deserialize<'de>,
{
    #[inline]
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        let set = IndexSet::<V>::deserialize(deserializer)?;
        Ok(Self::from(set))
    }
}

impl<V: Debug + std::hash::Hash + PartialEq + Eq + Clone> FromIterator<V>
    for OrderedSet<V>
{
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        let mut set = IndexSet::<V>::new();
        for item in iter {
            let _ = set.insert(item);
        }
        Self::from(set)
    }
}

unsafe impl<UT, V: Debug + Clone + std::hash::Hash + PartialEq + Eq + Lower<UT>>
    Lower<UT> for OrderedSet<V>
{
    type FfiType = RustBuffer;

    fn write(obj: Self, buf: &mut Vec<u8>) {
        let len = i32::try_from(obj.len()).unwrap();
        buf.put_i32(len); // We limit arrays to i32::MAX items
        for value in obj.0.clone().into_iter() {
            <V as Lower<UT>>::write(value, buf);
        }
    }

    fn lower(obj: Self) -> RustBuffer {
        Self::lower_into_rust_buffer(obj)
    }
}

unsafe impl<UT, V: Debug + Clone + std::hash::Hash + PartialEq + Eq + Lower<UT>>
    LowerReturn<UT> for OrderedSet<V>
{
    type ReturnType = <Self as Lower<UT>>::FfiType;

    fn lower_return(obj: Self) -> uniffi::Result<Self::ReturnType, RustBuffer> {
        Ok(<Self as Lower<UT>>::lower(obj))
    }
}

unsafe impl<UT, V: Debug + Clone + std::hash::Hash + PartialEq + Eq + Lift<UT>>
    Lift<UT> for OrderedSet<V>
{
    type FfiType = RustBuffer;

    fn try_read(buf: &mut &[u8]) -> uniffi::Result<Self> {
        check_remaining(buf, 4)?;
        let len = usize::try_from(buf.get_i32())?;
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(<V as Lift<UT>>::try_read(buf)?)
        }
        uniffi::Result::Ok(Self::from(IndexSet::<V>::from_iter(vec)))
    }

    fn try_lift(buf: RustBuffer) -> uniffi::Result<Self> {
        Self::try_lift_from_rust_buffer(buf)
    }
}

impl<
        UT,
        V: Debug + Clone + std::hash::Hash + PartialEq + Eq + UFTypeId<UT>,
    > UFTypeId<UT> for OrderedSet<V>
{
    const TYPE_ID_META: MetadataBuffer =
        MetadataBuffer::from_code(metadata::codes::TYPE_VEC)
            .concat(V::TYPE_ID_META);
}
