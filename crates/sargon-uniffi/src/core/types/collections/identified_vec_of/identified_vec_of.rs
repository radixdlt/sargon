use crate::prelude::*;

use sargon::IdentifiedVecOf;

/// Converting `IdentifiedVecOf` to/from `Vec`

pub trait FromIdentifiedVecOf<
    InternalElement: Debug + PartialEq + Eq + Clone + sargon::Identifiable,
    Element,
>
{
    fn into_vec(self) -> Vec<Element>;
}

impl<
        InternalElement: Debug + PartialEq + Eq + Clone + sargon::Identifiable,
        Element,
    > FromIdentifiedVecOf<InternalElement, Element>
    for IdentifiedVecOf<InternalElement>
where
    Element: From<InternalElement>,
{
    fn into_vec(self) -> Vec<Element> {
        self.into_iter().map(Element::from).collect()
    }
}

/// Converting `Vec` to/from `Vec` from Sargon
pub trait FromInternalVec<InternalElement, Element> {
    fn into_vec(self) -> Vec<Element>;
}

pub trait IntoInternalVec<InternalElement, Element> {
    fn into_internal_vec(self) -> Vec<InternalElement>;
}

impl<InternalElement, Element> FromInternalVec<InternalElement, Element>
    for Vec<InternalElement>
where
    Element: From<InternalElement>,
{
    fn into_vec(self) -> Vec<Element> {
        self.into_iter().map(Element::from).collect()
    }
}

// impl<InternalElement, Element> IntoInternalVec<InternalElement, Element>
//     for Vec<Element>
// where
//     Element: Into<InternalElement>,
// {
//     fn into_internal_vec(self) -> Vec<InternalElement> {
//         self.into_iter().map(Into::into).collect()
//     }
// }

// impl<
//         InternalElement: Debug + PartialEq + Eq + Clone + sargon::Identifiable,
//         Element,
//     > IntoInternalVec<InternalElement, Element> for Vec<Element>
// where
//     Element: Into<InternalElement>,
// {
//     fn into_internal_vec(self) -> IdentifiedVecOf<InternalElement> {
//         let internal_vec: Vec<InternalElement> = self.into_internal_vec();
//         internal_vec.into()
//     }
// }

pub trait FromInternalHashMap<InternalKey, InternalElement, Key, Element> {
    fn into_hash_map(self) -> HashMap<Key, Element>;
}

impl<InternalKey, InternalElement, Key, Element>
FromInternalHashMap<InternalKey, InternalElement, Key, Element>
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
FromInternalHashMap<
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
FromInternalHashMap<InternalKey, InternalElement, Key, Vec<Element>>
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

pub trait IntoInternalHashMap<Key, Element, InternalKey, InternalElement> {
    fn into_internal_hash_map(self) -> HashMap<InternalKey, InternalElement>;
}

impl<Key, Element, InternalKey, InternalElement>
IntoInternalHashMap<Key, Element, InternalKey, InternalElement>
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
IntoInternalHashMap<
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
IntoInternalHashMap<Key, Element, InternalKey, Vec<InternalElement>>
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
            .map(|(k, v)| (k.into(), v.into_internal()))
            .collect()
    }
}

pub trait FromInternal<Internal, External> {
    fn into_type(self) -> External;
}

impl<InternalElement, Element> FromInternal<Vec<InternalElement>, Vec<Element>>
    for Vec<InternalElement>
where
    Element: From<InternalElement>,
{
    fn into_type(self) -> Vec<Element> {
        self.into_iter().map(Element::from).collect()
    }
}

pub trait IntoInternal<External, Internal> {
    fn into_internal(self) -> Internal;
}

impl<InternalElement, Element> IntoInternal<Vec<Element>, IdentifiedVecOf<InternalElement>>
    for Vec<Element>
where
    InternalElement: Debug + PartialEq + Eq + Clone + sargon::Identifiable,
    Element: Into<InternalElement>,
{
    fn into_internal(self) -> IdentifiedVecOf<InternalElement> {
        self.into_iter().map(Into::into).collect()
    }
}

impl<InternalElement, Element> IntoInternal<Vec<Element>, Vec<InternalElement>>
    for Vec<Element>
where
    Element: Into<InternalElement>,
{
    fn into_internal(self) -> Vec<InternalElement> {
        self.into_iter().map(Into::into).collect()
    }
}

impl<Key, Element, InternalKey, InternalElement> IntoInternal<HashMap<Key, Element>, HashMap<InternalKey, InternalElement>>
    for HashMap<Key, Element>
where
    InternalKey: std::hash::Hash + Eq,
    Key: Into<InternalKey>,
    Element: Into<InternalElement>,
{
    fn into_internal(self) -> HashMap<InternalKey, InternalElement> {
        self.into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect()
    }
}

// impl<Key1, Key2, Element, InternalKey1, InternalKey2, InternalElement> IntoInternal<HashMap<Key1, HashMap<Key2, Element>>, HashMap<InternalKey1, HashMap<InternalKey2, InternalElement>>>
//     for HashMap<Key1, HashMap<Key2, Element>>
// where
//     InternalKey1: std::hash::Hash + Eq,
//     InternalKey2: std::hash::Hash + Eq,
//     Key1: Into<InternalKey1>,
//     Key2: Into<InternalKey2>,
//     Element: Into<InternalElement>,
// {
//     fn into_internal(self) -> HashMap<InternalKey1, HashMap<InternalKey2, InternalElement>> {
//         self.into_iter()
//             .map(|(k, v)| (k.into(), v.into_internal()))
//             .collect()
//     }
// }

// impl<Key, Element, InternalKey, InternalElement> IntoInternal<HashMap<Key, Vec<Element>>, HashMap<InternalKey, Vec<InternalElement>>>
//     for HashMap<Key, Vec<Element>>
// where
//     InternalKey: std::hash::Hash + Eq,
//     Key: Into<InternalKey>,
//     Element: Into<InternalElement>,
// {
//     fn into_internal(self) -> HashMap<InternalKey, Vec<InternalElement>> {
//         self.into_iter()
//             .map(|(k, v)| (k.into(), v.into_internal()))
//             .collect()
//     }
// }