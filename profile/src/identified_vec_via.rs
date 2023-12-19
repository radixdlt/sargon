use identified_vec::{
    identified_vec_into_iterator::IdentifiedVecIntoIterator, Identifiable, IdentifiedVecOf,
    IsIdentifiableVecOfVia, ViaMarker,
};
use std::fmt::{Debug, Display};
#[derive(std::fmt::Debug, Clone, Eq, PartialEq)]
pub struct IdentifiedVecVia<Element: Identifiable + Debug + Clone>(IdentifiedVecOf<Element>);

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl<Element: Identifiable + Debug + Clone> serde::Serialize for IdentifiedVecVia<Element>
where
    Element: serde::Serialize + Identifiable + std::fmt::Debug + Clone,
{
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        IdentifiedVecOf::serialize(&self.0, serializer)
    }
}

impl<'de, Element: Identifiable + Debug + Clone> serde::Deserialize<'de>
    for IdentifiedVecVia<Element>
where
    Element: serde::Deserialize<'de> + Identifiable + std::fmt::Debug + Clone,
{
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let id_vec_of = IdentifiedVecOf::<Element>::deserialize(deserializer)?;
        return Ok(Self::from_identified_vec_of(id_vec_of));
    }
}
