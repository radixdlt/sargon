use crate::prelude::*;
use sargon::EntitiesControlledByFactorSource as InternalEntitiesControlledByFactorSource;

/// This is the result of checking what entities are controlled by a given `FactorSource`.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct EntitiesControlledByFactorSource {
    /// The factor source that controls the entities.
    pub factor_source: FactorSource,

    /// Whether the mnemonic of the factor source is present in keychain.
    pub is_mnemonic_present_in_keychain: bool,

    /// Whether the mnemonic of the factor source is marked as backed up.
    pub is_mnemonic_marked_as_backed_up: bool,

    /// The visible accounts controlled by the factor source.
    pub accounts: Vec<Account>,

    /// The hidden accounts controlled by the factor source.
    pub hidden_accounts: Vec<Account>,

    /// The visible personas controlled by the factor source.
    pub personas: Vec<Persona>,

    /// The hidden personas controlled by the factor source.
    pub hidden_personas: Vec<Persona>,
}
