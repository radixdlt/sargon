use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntitiesControlledByFactorSource {
    /// The factor source that controls the entities.
    pub factor_source: FactorSource,

    /// Whether the mnemonic of the factor source is present in keychain.
    pub is_mnemonic_present_in_keychain: bool,

    /// Whether the mnemonic of the factor source is marked as backed up.
    pub is_mnemonic_marked_as_backed_up: bool,

    /// The visible accounts controlled by the factor source.
    pub accounts: Accounts,

    /// The hidden accounts controlled by the factor source.
    pub hidden_accounts: Accounts,

    /// The visible personas controlled by the factor source.
    pub personas: Personas,

    /// The hidden personas controlled by the factor source.
    pub hidden_personas: Personas,
}
