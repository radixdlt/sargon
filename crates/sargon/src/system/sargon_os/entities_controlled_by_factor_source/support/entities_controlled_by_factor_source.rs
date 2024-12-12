use crate::prelude::*;

/// This is the result of checking what entities are controlled by a given `FactorSource`.
#[derive(Clone, Debug, PartialEq)]
pub struct EntitiesControlledByFactorSource {
    /// The accessibility of the factor source.
    pub accessibility: FactorSourceAccessibility,

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
        accessibility: FactorSourceAccessibility,
        accounts: Accounts,
        hidden_accounts: Accounts,
        personas: Personas,
        hidden_personas: Personas,
    ) -> Self {
        Self {
            accessibility,
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
            FactorSourceAccessibility::sample(),
            Accounts::sample(),
            Accounts::new(),
            Personas::sample(),
            Personas::new(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            FactorSourceAccessibility::sample_other(),
            Accounts::sample_other(),
            Accounts::new(),
            Personas::sample_other(),
            Personas::new(),
        )
    }
}
