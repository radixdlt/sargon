use identified_vec::{
    identified_vec_into_iterator::IdentifiedVecIntoIterator, Identifiable, IdentifiedVecOf,
    IsIdentifiableVecOfVia, IsIdentifiedVec, IsIdentifiedVecOf, ViaMarker,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Display, Formatter};
use uniffi::{
    check_remaining,
    deps::bytes::{Buf, BufMut},
    metadata, Lift, Lower, LowerReturn, MetadataBuffer, RustBuffer,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct IdentifiedVecVia<Element: Identifiable + Debug + Clone> {
    id_vec: IdentifiedVecOf<Element>,
}

impl<Element: Identifiable + Debug + Clone> IdentifiedVecVia<Element> {
    pub fn new() -> Self {
        Self::from_identified_vec_of(IdentifiedVecOf::new())
    }

    pub fn from_iter<I>(unique_elements: I) -> Self
    where
        I: IntoIterator<Item = Element>,
    {
        Self::from_identified_vec_of(IdentifiedVecOf::from_iter(unique_elements))
    }

    pub fn len(&self) -> usize {
        self.id_vec.len()
    }
}
impl<Element: Identifiable + Debug + Clone> ViaMarker for IdentifiedVecVia<Element> {}
impl<Element: Identifiable + Debug + Clone> IsIdentifiableVecOfVia<Element>
    for IdentifiedVecVia<Element>
{
    fn via_mut(&mut self) -> &mut IdentifiedVecOf<Element> {
        &mut self.id_vec
    }

    fn via(&self) -> &IdentifiedVecOf<Element> {
        &self.id_vec
    }

    fn from_identified_vec_of(identified_vec_of: IdentifiedVecOf<Element>) -> Self {
        Self {
            id_vec: identified_vec_of,
        }
    }
}

impl<Element: Identifiable + Debug + Clone> Display for IdentifiedVecVia<Element> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.id_vec, f)
    }
}

impl<Element: Identifiable + Debug + Clone> IntoIterator for IdentifiedVecVia<Element> {
    type Item = Element;
    type IntoIter = IdentifiedVecIntoIterator<<Element as Identifiable>::ID, Element>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.id_vec)
    }
}

impl<Element: Identifiable + Debug + Clone> Serialize for IdentifiedVecVia<Element>
where
    Element: Serialize + Identifiable + Debug + Clone,
{
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        IdentifiedVecOf::serialize(&self.id_vec, serializer)
    }
}

impl<'de, Element: Identifiable + Debug + Clone> Deserialize<'de> for IdentifiedVecVia<Element>
where
    Element: Deserialize<'de> + Identifiable + Debug + Clone,
{
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let id_vec_of = IdentifiedVecOf::<Element>::deserialize(deserializer)?;
        return Ok(Self::from_identified_vec_of(id_vec_of));
    }
}

unsafe impl<UT, T: Identifiable + Debug + Clone + Lower<UT>> Lower<UT> for IdentifiedVecVia<T> {
    type FfiType = RustBuffer;

    fn write(obj: IdentifiedVecVia<T>, buf: &mut Vec<u8>) {
        let len = i32::try_from(obj.len()).unwrap();
        buf.put_i32(len); // We limit arrays to i32::MAX items
        for item in obj {
            <T as Lower<UT>>::write(item, buf);
        }
    }

    fn lower(obj: IdentifiedVecVia<T>) -> RustBuffer {
        Self::lower_into_rust_buffer(obj)
    }

    const TYPE_ID_META: MetadataBuffer =
        MetadataBuffer::from_code(metadata::codes::TYPE_VEC).concat(T::TYPE_ID_META);
}

unsafe impl<UT, T: Identifiable + Debug + Clone + Lower<UT>> LowerReturn<UT>
    for IdentifiedVecVia<T>
{
    type ReturnType = <Self as Lower<UT>>::FfiType;

    fn lower_return(obj: Self) -> uniffi::Result<Self::ReturnType, RustBuffer> {
        Ok(<Self as Lower<UT>>::lower(obj))
    }

    const TYPE_ID_META: MetadataBuffer = <Self as Lower<UT>>::TYPE_ID_META;
}

unsafe impl<UT, T: Identifiable + Debug + Clone + Lift<UT>> Lift<UT> for IdentifiedVecVia<T> {
    type FfiType = RustBuffer;

    fn try_read(buf: &mut &[u8]) -> uniffi::Result<IdentifiedVecVia<T>> {
        check_remaining(buf, 4)?;
        let len = usize::try_from(buf.get_i32())?;
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(<T as Lift<UT>>::try_read(buf)?)
        }
        Ok(IdentifiedVecVia::from_iter(vec.into_iter()))
    }

    fn try_lift(buf: RustBuffer) -> uniffi::Result<IdentifiedVecVia<T>> {
        Self::try_lift_from_rust_buffer(buf)
    }

    const TYPE_ID_META: MetadataBuffer =
        MetadataBuffer::from_code(metadata::codes::TYPE_VEC).concat(T::TYPE_ID_META);
}
