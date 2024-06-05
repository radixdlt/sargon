use crate::prelude::*;
use std::any::TypeId as StdTypeId;
use uniffi::TypeId as UFTypeId;
use uniffi::{
    check_remaining,
    deps::bytes::{Buf, BufMut},
    metadata, Lift, Lower, LowerReturn, MetadataBuffer, RustBuffer,
};

// We turn an `[Rust] IndexMap -> Array/List [FFI]``
#[derive(Clone)]
pub struct OrderedSet<V>(IndexSet<V>);
impl<V> From<IndexSet<V>> for IndexSet<V> {
    fn from(value: IndexSet<V>) -> Self {
        Self(value)
    }
}
impl<V> std::ops::Deref for OrderedSet<V> {
    type Target = IndexSet<V>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

unsafe impl<UT, V: Lower<UT>> Lower<UT> for OrderedSet<V> {
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
}

unsafe impl<UT, V: LowerReturn<UT>> LowerReturn<UT> for OrderedSet<V> {
    type ReturnType = <Self as LowerReturn<UT>>::FfiType;

    fn lower_return(obj: Self) -> uniffi::Result<Self::ReturnType, RustBuffer> {
        Ok(<Self as Lower<UT>>::lower(obj))
    }
}

unsafe impl<UT, V: Lift<UT>> Lift<UT> for OrderedSet<V> {
    type FfiType = RustBuffer;

    fn try_read(buf: &mut &[u8]) -> uniffi::Result<Self> {
        check_remaining(buf, 4)?;
        let len = usize::try_from(buf.get_i32())?;
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(<V as Lift<UT>>::try_read(buf)?)
        }
        // import_identified_vec_of_from(vec).map_err(|e| e.into())
        uniffi::Result::Ok(Self::from(IndexSet::from_iter(vec)))
    }

    fn try_lift(buf: RustBuffer) -> uniffi::Result<Self> {
        Self::try_lift_from_rust_buffer(buf)
    }
}

impl<UT, V: UFTypeId<UT>> UFTypeId<UT> for IndexSet<V> {
    const TYPE_ID_META: MetadataBuffer =
        MetadataBuffer::from_code(metadata::codes::TYPE_VEC)
            .concat(V::TYPE_ID_META);
}
