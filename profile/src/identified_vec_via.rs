use identified_vec::{
    identified_vec_into_iterator::IdentifiedVecIntoIterator, Identifiable, IdentifiedVecOf,
    IsIdentifiableVecOfVia, IsIdentifiedVec, ViaMarker,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IdentifiedVecVia<Element: Identifiable + Debug + Clone> {
    id_vec: IdentifiedVecOf<Element>,
}

// https://github.com/mozilla/uniffi-rs/issues/1606#issuecomment-1598914207
// https://github.com/MathieuTricoire/convex-rs-ffi/blob/2a25bc9d593ade508c1c16a0ab6174fd6c980d53/convex-ffi/src/lib.rs#L81-L112
unsafe impl<UT, Element: Identifiable + Debug + Clone> uniffi::FfiConverter<UT>
    for IdentifiedVecVia<Element>
{
    uniffi::ffi_converter_rust_buffer_lift_and_lower!(Element);

    fn write(obj: Self, buf: &mut Vec<u8>) {
        let len = i32::try_from(obj.len()).unwrap();
        buf.put_i32(len);
        for element in obj.0.items() {
            <Element as uniffi::FfiConverter<UT>>::write(element, buf);
        }
    }

    fn try_read(buf: &mut &[u8]) -> uniffi::Result<Self> {
        uniffi::check_remaining(buf, 4)?;
        let len = usize::try_from(buf.get_i32())?;
        let mut identified_vec_via = IdentifiedVecVia::<Element>::new();
        for _ in 0..len {
            let element = <Element as uniffi::FfiConverter<UT>>::try_read(buf)?;
            identified_vec_via.append(element)
        }
        Ok(identified_vec_via)
    }

    const TYPE_ID_META: uniffi::MetadataBuffer =
        uniffi::MetadataBuffer::from_code(uniffi::metadata::codes::TYPE_HASH_MAP)
            .concat(<Element as uniffi::FfiConverter<UT>>::TYPE_ID_META);
}

impl<Element: Identifiable + Debug + Clone> ViaMarker for IdentifiedVecVia<Element> {}
impl<Element: Identifiable + Debug + Clone> IsIdentifiableVecOfVia<Element>
    for IdentifiedVecVia<Element>
{
    fn via_mut(&mut self) -> &mut IdentifiedVecOf<Element> {
        &mut self.0
    }

    fn via(&self) -> &IdentifiedVecOf<Element> {
        &self.0
    }

    fn from_identified_vec_of(identified_vec_of: IdentifiedVecOf<Element>) -> Self {
        Self(identified_vec_of)
    }
}

impl<Element: Identifiable + Debug + Clone> Display for IdentifiedVecVia<Element> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl<Element: Identifiable + Debug + Clone> IntoIterator for IdentifiedVecVia<Element> {
    type Item = Element;
    type IntoIter = IdentifiedVecIntoIterator<<Element as Identifiable>::ID, Element>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.0)
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
        IdentifiedVecOf::serialize(&self.0, serializer)
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
