use crate::prelude::*;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    SerializeDisplay,
    derive_more::Display,
)]
pub enum HDIndex {
    Unsecurified(HDIndexUnsecurified),
    Securified(HDIndexSecurified),
}

impl From<HDPathValue> for HDIndex {
    fn from(value: HDPathValue) -> Self {
    if value >= U31_MAX {
        HDIndex::Securified(HDIndexSecurified::from::)
    } else {
        HDIndex::Unsecurified(HDIndexUnsecurified::new(value).unwrap())
    }
    }
}