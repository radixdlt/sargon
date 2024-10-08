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
    InternalElement: Into<Element>,
{
    fn into_internal_vec(self) -> Vec<InternalElement> {
        self.into_iter().map(Into::into).collect()
    }
}

// #[cfg(test)]
// mod tests {

//     use super::super::super::User;
//     use super::*;

//     #[allow(clippy::upper_case_acronyms)]
//     type SUT = IdentifiedVecOf<User>;

//     #[test]
//     fn equality() {
//         assert_eq!(SUT::sample(), SUT::sample());
//         assert_eq!(SUT::sample_other(), SUT::sample_other());
//     }

//     #[test]
//     fn inequality() {
//         assert_ne!(SUT::sample(), SUT::sample_other());
//     }

//     #[test]
//     fn index() {
//         let sut = SUT::sample();
//         assert_eq!(sut[0], User::alice());
//         assert_eq!(sut[1], User::carol());
//     }
// }
