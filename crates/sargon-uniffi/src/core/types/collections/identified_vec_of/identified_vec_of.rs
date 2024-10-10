use crate::prelude::*;

use sargon::IdentifiedVecOf;

pub trait MapFromInternalIdentifiedVecOf<InternalElement: Debug + PartialEq + Eq + Clone + sargon::Identifiable, Element> {
    fn into_vec(self) -> Vec<Element>;
}

pub trait MapToInternalIdentifiedVecOf<InternalElement: Debug + PartialEq + Eq + Clone + sargon::Identifiable, Element> {
    fn into_identified_vec(self) -> IdentifiedVecOf<InternalElement>;
}

impl<InternalElement: Debug + PartialEq + Eq + Clone + sargon::Identifiable, Element> MapFromInternalIdentifiedVecOf<InternalElement, Element> for IdentifiedVecOf<InternalElement>
where Element: From<InternalElement>
{
    fn into_vec(self) -> Vec<Element> {
        self.into_iter().map(Element::from).collect()
    }
}

impl<InternalElement: Debug + PartialEq + Eq + Clone + sargon::Identifiable, Element> MapToInternalIdentifiedVecOf<InternalElement, Element> for Vec<Element> 
where Element: Into<InternalElement>,
{
    fn into_identified_vec(self) -> IdentifiedVecOf<InternalElement> {
        self.into_internal_vec().into()
    }
}

pub trait MapFromInternalVec<InternalElement, Element> {
    fn into_vec(self) -> Vec<Element>;
}

pub trait MapIntoInternalVec<InternalElement, Element> {
    fn into_internal_vec(self) -> Vec<InternalElement>;
}

impl<InternalElement, Element> MapFromInternalVec<InternalElement, Element> for Vec<InternalElement>
where
    Element: From<InternalElement>,
{
    fn into_vec(self) -> Vec<Element> {
        self.into_iter().map(Element::from).collect()
    }
}

impl<InternalElement, Element> MapIntoInternalVec<InternalElement, Element> for Vec<Element>
where
Element: Into<InternalElement>,
{
    fn into_internal_vec(self) -> Vec<InternalElement> {
        self.into_iter().map(Into::into).collect()
    }
}