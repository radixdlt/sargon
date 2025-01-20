use crate::prelude::*;
use sargon::ProvisionalSecurifiedConfig as InternalProvisionalSecurifiedConfig;

/// Intermediary state of changing the security structure of an entity.
/// Only a single variant for now but we might update it later. E.g.
/// we could have one state for when user has selected a shield but not
/// derived the factor instances yet.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum ProvisionalSecurifiedConfig {
    /// User has fully prepared a `SecurityStructureOfFactorInstances` but
    /// not made a transaction to apply it to the entity yet.
    FactorInstancesDerived {
        value: SecurityStructureOfFactorInstances,
    },
}
