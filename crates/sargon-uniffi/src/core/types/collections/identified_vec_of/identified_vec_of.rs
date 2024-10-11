use crate::prelude::*;

use sargon::IdentifiedVecOf;

pub trait MapFromInternalIdentifiedVecOf<
    InternalElement: Debug + PartialEq + Eq + Clone + sargon::Identifiable,
    Element,
>
{
    fn into_vec(self) -> Vec<Element>;
}

pub trait MapToInternalIdentifiedVecOf<
    InternalElement: Debug + PartialEq + Eq + Clone + sargon::Identifiable,
    Element,
>
{
    fn into_identified_vec(self) -> IdentifiedVecOf<InternalElement>;
}

impl<
        InternalElement: Debug + PartialEq + Eq + Clone + sargon::Identifiable,
        Element,
    > MapFromInternalIdentifiedVecOf<InternalElement, Element>
    for IdentifiedVecOf<InternalElement>
where
    Element: From<InternalElement>,
{
    fn into_vec(self) -> Vec<Element> {
        self.into_iter().map(Element::from).collect()
    }
}

impl<
        InternalElement: Debug + PartialEq + Eq + Clone + sargon::Identifiable,
        Element,
    > MapToInternalIdentifiedVecOf<InternalElement, Element> for Vec<Element>
where
    Element: Into<InternalElement>,
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

impl<InternalElement, Element> MapFromInternalVec<InternalElement, Element>
    for Vec<InternalElement>
where
    Element: From<InternalElement>,
{
    fn into_vec(self) -> Vec<Element> {
        self.into_iter().map(Element::from).collect()
    }
}

impl<InternalElement, Element> MapIntoInternalVec<InternalElement, Element>
    for Vec<Element>
where
    Element: Into<InternalElement>,
{
    fn into_internal_vec(self) -> Vec<InternalElement> {
        self.into_iter().map(Into::into).collect()
    }
}

pub trait MapFromInternalHashMap<InternalKey, InternalElement, Key, Element> {
    fn into_hash_map(self) -> HashMap<Key, Element>;
}

impl<InternalKey, InternalElement, Key, Element>
    MapFromInternalHashMap<InternalKey, InternalElement, Key, Element>
    for HashMap<InternalKey, InternalElement>
where
    Key: From<InternalKey> + std::hash::Hash + Eq,
    Element: From<InternalElement>,
{
    fn into_hash_map(self) -> HashMap<Key, Element> {
        self.into_iter()
            .map(|(k, v)| (Key::from(k), Element::from(v)))
            .collect()
    }
}

impl<InternalKey1, InternalElement, InternalKey2, Key1, Key2, Element>
    MapFromInternalHashMap<
        InternalKey1,
        InternalElement,
        Key1,
        HashMap<Key2, Element>,
    > for HashMap<InternalKey1, HashMap<InternalKey2, InternalElement>>
where
    Key1: From<InternalKey1> + std::hash::Hash + Eq,
    Key2: From<InternalKey2> + std::hash::Hash + Eq,
    Element: From<InternalElement>,
{
    fn into_hash_map(self) -> HashMap<Key1, HashMap<Key2, Element>> {
        self.into_iter()
            .map(|(k, v)| (Key1::from(k), v.into_hash_map()))
            .collect()
    }
}

impl<InternalKey, InternalElement, Key, Element>
    MapFromInternalHashMap<InternalKey, InternalElement, Key, Vec<Element>>
    for HashMap<InternalKey, Vec<InternalElement>>
where
    Key: From<InternalKey> + std::hash::Hash + Eq,
    Element: From<InternalElement>,
{
    fn into_hash_map(self) -> HashMap<Key, Vec<Element>> {
        self.into_iter()
            .map(|(k, v)| (Key::from(k), v.into_vec()))
            .collect()
    }
}

pub trait MapIntoInternalHashMap<Key, Element, InternalKey, InternalElement> {
    fn into_internal_hash_map(self) -> HashMap<InternalKey, InternalElement>;
}

impl<Key, Element, InternalKey, InternalElement>
    MapIntoInternalHashMap<Key, Element, InternalKey, InternalElement>
    for HashMap<Key, Element>
where
    InternalKey: std::hash::Hash + Eq,
    Key: Into<InternalKey>,
    Element: Into<InternalElement>,
{
    fn into_internal_hash_map(self) -> HashMap<InternalKey, InternalElement> {
        self.into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect()
    }
}

impl<Key1, Key2, Element, InternalKey1, InternalKey2, InternalElement>
    MapIntoInternalHashMap<
        Key1,
        Element,
        InternalKey1,
        HashMap<InternalKey2, InternalElement>,
    > for HashMap<Key1, HashMap<Key2, Element>>
where
    InternalKey1: std::hash::Hash + Eq,
    InternalKey2: std::hash::Hash + Eq,
    Key1: Into<InternalKey1>,
    Key2: Into<InternalKey2>,
    Element: Into<InternalElement>,
{
    fn into_internal_hash_map(
        self,
    ) -> HashMap<InternalKey1, HashMap<InternalKey2, InternalElement>> {
        self.into_iter()
            .map(|(k, v)| (k.into(), v.into_internal_hash_map()))
            .collect()
    }
}

impl<Key, Element, InternalKey, InternalElement>
    MapIntoInternalHashMap<Key, Element, InternalKey, Vec<InternalElement>>
    for HashMap<Key, Vec<Element>>
where
    InternalKey: std::hash::Hash + Eq,
    Key: Into<InternalKey>,
    Element: Into<InternalElement>,
{
    fn into_internal_hash_map(
        self,
    ) -> HashMap<InternalKey, Vec<InternalElement>> {
        self.into_iter()
            .map(|(k, v)| (k.into(), v.into_internal_vec()))
            .collect()
    }
}
