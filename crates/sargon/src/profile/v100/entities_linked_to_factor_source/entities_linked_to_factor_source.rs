use crate::prelude::*;

/// This is the result of checking what entities are linked to a given `FactorSource`.
#[derive(Clone, Debug, PartialEq)]
pub struct EntitiesLinkedToFactorSource {
    /// The integrity of the factor source.
    pub integrity: FactorSourceIntegrity,

    /// The visible accounts linked to the factor source.
    pub accounts: Accounts,

    /// The hidden accounts linked to the factor source.
    pub hidden_accounts: Accounts,

    /// The visible personas linked to the factor source.
    pub personas: Personas,

    /// The hidden personas linked to the factor source.
    pub hidden_personas: Personas,
}

impl EntitiesLinkedToFactorSource {
    pub fn new(
        integrity: FactorSourceIntegrity,
        accounts: Accounts,
        hidden_accounts: Accounts,
        personas: Personas,
        hidden_personas: Personas,
    ) -> Self {
        Self {
            integrity,
            accounts,
            hidden_accounts,
            personas,
            hidden_personas,
        }
    }
}

impl HasSampleValues for EntitiesLinkedToFactorSource {
    fn sample() -> Self {
        Self::new(
            FactorSourceIntegrity::sample(),
            Accounts::sample(),
            Accounts::new(),
            Personas::sample(),
            Personas::new(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            FactorSourceIntegrity::sample_other(),
            Accounts::sample_other(),
            Accounts::new(),
            Personas::sample_other(),
            Personas::new(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = EntitiesLinkedToFactorSource;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
