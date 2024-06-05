use crate::prelude::*;

use super::{export_ordered_set, import_ordered_set_from};

use std::any::TypeId as StdTypeId;
use uniffi::TypeId as UFTypeId;
use uniffi::{
    check_remaining,
    deps::bytes::{Buf, BufMut},
    metadata, Lift, Lower, LowerReturn, MetadataBuffer, RustBuffer,
};

unsafe impl<UT, V: Clone + std::hash::Hash + PartialEq + Eq + Lower<UT>>
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

unsafe impl<UT, V: Clone + std::hash::Hash + PartialEq + Eq + Lower<UT>>
    LowerReturn<UT> for OrderedSet<V>
{
    type ReturnType = <Self as Lower<UT>>::FfiType;

    fn lower_return(obj: Self) -> uniffi::Result<Self::ReturnType, RustBuffer> {
        Ok(<Self as Lower<UT>>::lower(obj))
    }
}

unsafe impl<UT, V: Clone + std::hash::Hash + PartialEq + Eq + Lift<UT> + 'static> Lift<UT>
    for OrderedSet<V>
{
    type FfiType = RustBuffer;

    fn try_read(buf: &mut &[u8]) -> uniffi::Result<Self> {
        check_remaining(buf, 4)?;
        let len = usize::try_from(buf.get_i32())?;
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(<V as Lift<UT>>::try_read(buf)?)
        }
        import_ordered_set_from(vec).map_err(|e| e.into())
    }

    fn try_lift(buf: RustBuffer) -> uniffi::Result<Self> {
        Self::try_lift_from_rust_buffer(buf)
    }
}

impl<UT, V: Clone + std::hash::Hash + PartialEq + Eq + UFTypeId<UT>>
    UFTypeId<UT> for OrderedSet<V>
{
    const TYPE_ID_META: MetadataBuffer =
        MetadataBuffer::from_code(metadata::codes::TYPE_VEC)
            .concat(V::TYPE_ID_META);
}
