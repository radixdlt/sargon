use crate::prelude::*;
use sargon::ResourceIdentifier as InternalResourceIdentifier;

/// An enum representation of an resource for which the user can set up its preferences.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
pub enum ResourceIdentifier {
    Fungible(ResourceAddress),
    NonFungible(ResourceAddress),
    PoolUnit(PoolAddress),
}
