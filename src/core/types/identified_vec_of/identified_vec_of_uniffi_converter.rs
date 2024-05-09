use crate::prelude::*;
use indexmap::IndexMap;
use std::any::TypeId;
use uniffi::{
    check_remaining,
    deps::bytes::{Buf, BufMut},
    metadata, Lift, Lower, LowerReturn, MetadataBuffer, RustBuffer,
};

// We turn an `[Rust] IndexMap -> Array/List [FFI]``

unsafe impl<UT, V: Debug + Eq + Clone + Identifiable + Lower<UT>> Lower<UT>
    for IdentifiedVecOf<V>
{
    type FfiType = RustBuffer;

    fn write(obj: Self, buf: &mut Vec<u8>) {
        let len = i32::try_from(obj.0.len()).unwrap();
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

unsafe impl<UT, V: Debug + Eq + Clone + Identifiable + Lower<UT>>
    LowerReturn<UT> for IdentifiedVecOf<V>
{
    type ReturnType = <Self as Lower<UT>>::FfiType;

    fn lower_return(obj: Self) -> uniffi::Result<Self::ReturnType, RustBuffer> {
        Ok(<Self as Lower<UT>>::lower(obj))
    }

    const TYPE_ID_META: MetadataBuffer = <Self as Lower<UT>>::TYPE_ID_META;
}
unsafe impl<UT, V: Debug + Eq + Clone + Identifiable + Lift<UT> + 'static>
    Lift<UT> for IdentifiedVecOf<V>
{
    type FfiType = RustBuffer;

    fn try_read(buf: &mut &[u8]) -> uniffi::Result<Self> {
        check_remaining(buf, 4)?;
        let len = usize::try_from(buf.get_i32())?;

        if TypeId::of::<V>() == TypeId::of::<FactorSource>() && len == 0 {
            return Err(CommonError::FactorSourcesMustNotBeEmpty)
                .map_err(|e| e.into());
        }
        if TypeId::of::<V>() == TypeId::of::<SLIP10Curve>() && len == 0 {
            return Err(CommonError::SupportedCurvesMustNotBeEmpty)
                .map_err(|e| e.into());
        }

        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(<V as Lift<UT>>::try_read(buf)?)
        }
        let mut map = Self::new();
        for item in vec {
            map.try_insert_unique(item)?;
        }
        Ok(map)
    }

    fn try_lift(buf: RustBuffer) -> uniffi::Result<Self> {
        Self::try_lift_from_rust_buffer(buf)
    }

    const TYPE_ID_META: MetadataBuffer =
        MetadataBuffer::from_code(metadata::codes::TYPE_VEC)
            .concat(V::TYPE_ID_META);
}

#[cfg(test)]
mod tests {

    use super::super::User;
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = IdentifiedVecOf<User>;

    #[test]
    fn manual_perform_uniffi_conversion_successful() {
        let test_expected = |from: SUT, to: SUT| {
            let ffi_side = <SUT as Lower<crate::UniFfiTag>>::lower(from);
            let from_ffi =
                <SUT as Lift<crate::UniFfiTag>>::try_lift(ffi_side).unwrap();
            assert_eq!(from_ffi, to);
        };
        let test = |sut: SUT| test_expected(sut.clone(), sut);

        test(SUT::new());
        test(SUT::sample());
        test(SUT::sample_other());
    }

    #[test]
    fn manual_perform_uniffi_if_duplicates_throw() {
        // This is some advanced techniques...
        let mut bad_value_from_ffi_vec = Vec::new();
        bad_value_from_ffi_vec.put_i32(2); // duplicates
        <User as Lower<crate::UniFfiTag>>::write(
            User::alice(),
            &mut bad_value_from_ffi_vec,
        );
        <User as Lower<crate::UniFfiTag>>::write(
            User::alice(),
            &mut bad_value_from_ffi_vec,
        ); // duplicate!
        let bad_value_from_ffi = RustBuffer::from_vec(bad_value_from_ffi_vec);
        assert!(
            <SUT as Lift<crate::UniFfiTag>>::try_lift(bad_value_from_ffi)
                .is_err()
        );
    }

    #[test]
    fn manual_perform_uniffi_conversion_fail() {
        assert!(<SUT as Lift<crate::UniFfiTag>>::try_lift(RustBuffer::new())
            .is_err());
    }
}
