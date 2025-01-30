use crate::prelude::*;

use identified_vec_of::IdentifiedVecOf;

// ==========================
// === From InternalType ====
// ==========================
pub trait FromInternal<InternalType, Type> {
    fn into_type(self) -> Type;
}

// === IntoIterator ====
impl<T, InternalElement, Element> FromInternal<T, Vec<Element>> for T
where
    T: IntoIterator<Item = InternalElement>,
    Element: From<InternalElement>,
{
    fn into_type(self) -> Vec<Element> {
        self.into_iter().map(Element::from).collect()
    }
}

// ==========================
// === Into InternalType ====
// ==========================

pub trait IntoInternal<Type, InternalType> {
    fn into_internal(self) -> InternalType;
}

impl<InternalElement, Element>
    IntoInternal<Option<Element>, Option<InternalElement>> for Option<Element>
where
    Element: Into<InternalElement>,
{
    fn into_internal(self) -> Option<InternalElement> {
        self.map(Into::into)
    }
}

// =====    Vec  ========
impl<InternalElement, Element> IntoInternal<Vec<Element>, Vec<InternalElement>>
    for Vec<Element>
where
    Element: Into<InternalElement>,
{
    fn into_internal(self) -> Vec<InternalElement> {
        self.into_iter().map(Into::into).collect()
    }
}

// ====  IndexSet  ======
impl<InternalElement, Element>
    IntoInternal<Vec<Element>, IndexSet<InternalElement>> for Vec<Element>
where
    Element: Into<InternalElement>,
    InternalElement: std::hash::Hash + Eq,
{
    fn into_internal(self) -> IndexSet<InternalElement> {
        self.into_iter().map(Into::into).collect::<IndexSet<_>>()
    }
}

// === IdentifiedVecOf ====
impl<InternalElement, Element>
    IntoInternal<Vec<Element>, IdentifiedVecOf<InternalElement>>
    for Vec<Element>
where
    InternalElement: Debug + PartialEq + Eq + Clone + sargon::Identifiable,
    Element: Into<InternalElement>,
{
    fn into_internal(self) -> IdentifiedVecOf<InternalElement> {
        self.into_iter().map(Into::into).collect()
    }
}

// HashMap Conversion =================================================================================================

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
            .map(|(k, v)| (Key::from(k), v.into_type()))
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
