use crate::prelude::*;
use sargon::EntitiesLinkedToFactorSource as InternalEntitiesLinkedToFactorSource;

/// This is the result of checking what entities are controlled by a given `FactorSource`.
#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct EntitiesLinkedToFactorSource {
    /// The accessibility of the factor source.
    pub accessibility: FactorSourceAccessibility,

    /// The visible accounts linked to the factor source.
    pub accounts: Vec<Account>,

    /// The hidden accounts linked to the factor source.
    pub hidden_accounts: Vec<Account>,

    /// The visible personas linked to the factor source.
    pub personas: Vec<Persona>,

    /// The hidden personas linked to the factor source.
    pub hidden_personas: Vec<Persona>,
}
