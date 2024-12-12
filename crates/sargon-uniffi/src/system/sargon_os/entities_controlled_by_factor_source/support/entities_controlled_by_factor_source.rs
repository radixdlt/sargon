use crate::prelude::*;
use sargon::EntitiesControlledByFactorSource as InternalEntitiesControlledByFactorSource;

/// This is the result of checking what entities are controlled by a given `FactorSource`.
#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct EntitiesControlledByFactorSource {
    /// The accessibility of the factor source.
    pub accessibility: FactorSourceAccessibility,

    /// The visible accounts controlled by the factor source.
    pub accounts: Vec<Account>,

    /// The hidden accounts controlled by the factor source.
    pub hidden_accounts: Vec<Account>,

    /// The visible personas controlled by the factor source.
    pub personas: Vec<Persona>,

    /// The hidden personas controlled by the factor source.
    pub hidden_personas: Vec<Persona>,
}
