use crate::prelude::*;

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, derive_more::Display, uniffi::Enum,
)]
pub enum ResourcePreferenceKind {
    Fungible(ResourceAddress),
    NonFungible(NonFungibleGlobalId),
    PoolUnit(PoolAddress),
}
