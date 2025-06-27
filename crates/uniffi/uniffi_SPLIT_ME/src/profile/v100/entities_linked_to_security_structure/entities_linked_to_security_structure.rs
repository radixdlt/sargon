use crate::prelude::*;
use sargon::EntitiesLinkedToSecurityStructure as InternalEntitiesLinkedToSecurityStructure;

/// This is the result of checking what entities are controlled by a given `SecurityStructure`.
#[derive(Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct EntitiesLinkedToSecurityStructure {
    /// The metadata of the linked security structure.
    pub metadata: SecurityStructureMetadata,

    /// The visible accounts linked to the security structure.
    pub accounts: Vec<Account>,

    /// The hidden accounts linked to the security structure.
    pub hidden_accounts: Vec<Account>,

    /// The visible personas linked to the security structure.
    pub personas: Vec<Persona>,

    /// The hidden personas linked to the security structure.
    pub hidden_personas: Vec<Persona>,
}
