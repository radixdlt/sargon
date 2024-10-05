use crate::prelude::*;
use radix_rust::prelude::IndexMap;
use uniffi::deps::anyhow::Ok;
use std::any::TypeId as StdTypeId;
use uniffi::TypeId as UFTypeId;
use uniffi::{
    check_remaining,
    deps::bytes::{Buf, BufMut},
    metadata, Lift, Lower, LowerReturn, MetadataBuffer, RustBuffer,
};
use sargon::IdentifiedVecOf as InternalIdentifiedVecOf;

// We turn an `[Rust] IndexMap -> Array/List [FFI]``

unsafe impl<UT, V: Debug + Eq + Clone + sargon::Identifiable + Lower<UT>> Lower<UT>
    for IdentifiedVecOf<V>
{
    type FfiType = RustBuffer;

    fn write(obj: Self, buf: &mut Vec<u8>) {
        let len = i32::try_from(obj.0.len()).unwrap();
        buf.put_i32(len); // We limit arrays to i32::MAX items
        for value in &obj.0 {
            <V as Lower<UT>>::write(value, buf);
        }
    }

    fn lower(obj: Self) -> RustBuffer {
        Self::lower_into_rust_buffer(obj)
    }
}

unsafe impl<UT, V: Debug + Eq + Clone + sargon::Identifiable + Lower<UT>>
    LowerReturn<UT> for IdentifiedVecOf<V>
{
    type ReturnType = <Self as Lower<UT>>::FfiType;

    fn lower_return(obj: Self) -> uniffi::Result<Self::ReturnType, RustBuffer> {
        Ok(<Self as Lower<UT>>::lower(obj))
    }
}
unsafe impl<UT, V: Debug + Eq + Clone + sargon::Identifiable + Lift<UT> + 'static>
    Lift<UT> for IdentifiedVecOf<V>
{
    type FfiType = RustBuffer;

    fn try_read(buf: &mut &[u8]) -> uniffi::Result<Self> {
        check_remaining(buf, 4)?;
        let len = usize::try_from(buf.get_i32())?;
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(<V as Lift<UT>>::try_read(buf)?)
        }

        let internal = sargon::import_identified_vec_of_from(vec).map_err(|e| e.into())?;
        Ok(IdentifiedVecOf { 0: internal })
    }

    fn try_lift(buf: RustBuffer) -> uniffi::Result<Self> {
        Self::try_lift_from_rust_buffer(buf)
    }
}

impl<UT, V: Debug + Eq + Clone + sargon::Identifiable + UFTypeId<UT>> UFTypeId<UT>
    for IdentifiedVecOf<V>
{
    const TYPE_ID_META: MetadataBuffer =
        MetadataBuffer::from_code(metadata::codes::TYPE_VEC)
            .concat(V::TYPE_ID_META);
}

#[cfg(test)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Record,
)]
#[debug("({}: {})", id, name)]
#[display("{}", name)]
pub(super) struct User {
    pub(super) id: u8,
    pub(super) name: String,
}

#[cfg(test)]
impl User {
    pub(super) fn new(id: u8, name: impl AsRef<str>) -> Self {
        Self {
            id,
            name: name.as_ref().to_owned(),
        }
    }
}

#[cfg(test)]
impl Identifiable for User {
    type ID = u8;
    fn id(&self) -> Self::ID {
        self.id
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = IdentifiedVecOf<User>;

    #[test]
    fn manual_perform_uniffi_conversion_successful() {
        let test_expected = |from: SUT, to: SUT| {
            let ffi_side =
                <SUT as Lower<crate::UniFfiTag>>::lower(from.clone());
            let from_ffi =
                <SUT as Lift<crate::UniFfiTag>>::try_lift(ffi_side).unwrap();
            assert_eq!(from_ffi, to.clone());

            let ffi_side_lower_return =
                <SUT as LowerReturn<crate::UniFfiTag>>::lower_return(from)
                    .unwrap();
            let from_ffi_lower_return =
                <SUT as Lift<crate::UniFfiTag>>::try_lift(
                    ffi_side_lower_return,
                )
                .unwrap();
            assert_eq!(from_ffi_lower_return, to);
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
