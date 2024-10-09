use crate::prelude::*;

use sargon::IdentifiedVecOf as InternalIdentifiedVecOf;

pub trait InternalIdentifiedVecToVecConversion<InternalElement, Element> {
    fn into_vec(self) -> Vec<Element>;
}

pub trait InternalIdentifiedVecFromVecConversion<InternalElement, Element> {
    fn from_vec(value: Vec<Element>) -> Self;
}

impl<InternalElement: Debug + PartialEq + Eq + Clone + sargon::Identifiable, Element> InternalIdentifiedVecToVecConversion<InternalElement, Element> for InternalIdentifiedVecOf<InternalElement>
where Element: From<InternalElement>
{
    fn into_vec(self) -> Vec<Element> {
        self.into_iter().map(Element::from).collect()
    }
}

impl<InternalElement: Debug + PartialEq + Eq + Clone + sargon::Identifiable, Element> InternalIdentifiedVecFromVecConversion<InternalElement, Element> for InternalIdentifiedVecOf<InternalElement> 
where
    Element: Into<InternalElement>,
{
    fn from_vec(value: Vec<Element>) -> Self {
        value.into_iter().map(Element::into).collect()
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
    InternalElement: From<Element>,
{
    fn into_internal_vec(self) -> Vec<InternalElement> {
        self.into_iter().map(InternalElement::from).collect()
    }
}