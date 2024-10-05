use crate::prelude::*;

/// An enum representation of an resource for which the user can set up its preferences.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Ord,
    PartialOrd,
    derive_more::Display,
    uniffi::Enum,
)]
pub enum ResourceIdentifier {
    Fungible(ResourceAddress),
    NonFungible(ResourceAddress),
    PoolUnit(PoolAddress),
}

impl Identifiable for ResourceIdentifier {
    type ID = Self;
    fn id(&self) -> Self::ID {
        self.clone()
    }
}