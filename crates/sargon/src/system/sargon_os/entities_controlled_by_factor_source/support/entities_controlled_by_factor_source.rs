use crate::prelude::*;

/// This is the result of checking what entities are controlled by a given `FactorSource`.
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

impl EntitiesControlledByFactorSource {
    fn new(
        factor_source: FactorSource,
        is_mnemonic_present_in_keychain: bool,
        is_mnemonic_marked_as_backed_up: bool,
        accounts: Accounts,
        hidden_accounts: Accounts,
        personas: Personas,
        hidden_personas: Personas,
    ) -> Self {
        Self {
            factor_source,
            is_mnemonic_present_in_keychain,
            is_mnemonic_marked_as_backed_up,
            accounts,
            hidden_accounts,
            personas,
            hidden_personas,
        }
    }
}

impl HasSampleValues for EntitiesControlledByFactorSource {
    fn sample() -> Self {
        Self::new(
            FactorSource::sample(),
            true,
            true,
            Accounts::sample(),
            Accounts::new(),
            Personas::sample(),
            Personas::new(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            FactorSource::sample_other(),
            true,
            true,
            Accounts::sample_other(),
            Accounts::new(),
            Personas::sample_other(),
            Personas::new(),
        )
    }
}
