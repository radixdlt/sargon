use profile::prelude::*;

/// This is the result of checking what entities are linked to a given `SecurityStructure`.
#[derive(Clone, Debug, PartialEq)]
pub struct EntitiesLinkedToSecurityStructure {
    /// The visible accounts linked to the security structure.
    pub accounts: Accounts,

    /// The hidden accounts linked to the security structure.
    pub hidden_accounts: Accounts,

    /// The visible personas linked to the security structure.
    pub personas: Personas,

    /// The hidden personas linked to the security structure.
    pub hidden_personas: Personas,
}

impl EntitiesLinkedToSecurityStructure {
    pub fn new(
        accounts: Accounts,
        hidden_accounts: Accounts,
        personas: Personas,
        hidden_personas: Personas,
    ) -> Self {
        Self {
            accounts,
            hidden_accounts,
            personas,
            hidden_personas,
        }
    }
}

impl HasSampleValues for EntitiesLinkedToSecurityStructure {
    fn sample() -> Self {
        Self::new(
            Accounts::sample(),
            Accounts::new(),
            Personas::sample(),
            Personas::new(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
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
    type SUT = EntitiesLinkedToSecurityStructure;

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
