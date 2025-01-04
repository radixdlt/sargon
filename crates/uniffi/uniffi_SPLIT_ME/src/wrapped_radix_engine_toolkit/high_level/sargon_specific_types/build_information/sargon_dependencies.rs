use crate::prelude::*;
use sargon::SargonDependencies as InternalSargonDependencies;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct SargonDependencies {
    pub radix_engine_toolkit: DependencyInformation,
    pub scrypto_radix_engine: DependencyInformation,
}
