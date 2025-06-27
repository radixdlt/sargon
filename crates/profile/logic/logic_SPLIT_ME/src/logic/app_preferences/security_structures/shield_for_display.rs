use crate::prelude::*;

decl_identified_vec_of!(
    /// A collection of [`ShieldForDisplay`]
    ShieldsForDisplay,
    ShieldForDisplay
);

impl Identifiable for ShieldForDisplay {
    type ID = SecurityStructureID;

    fn id(&self) -> Self::ID {
        self.metadata.id
    }
}

impl HasSampleValues for ShieldsForDisplay {
    fn sample() -> Self {
        Self::from_iter([
            ShieldForDisplay::sample(),
            ShieldForDisplay::sample_other(),
        ])
    }

    fn sample_other() -> Self {
        Self::from_iter([ShieldForDisplay::sample_other()])
    }
}

/// A minimal version of a Security Structure meant for display purposes within wallet
#[derive(
    Serialize,
    Deserialize,
    Debug,
    PartialEq,
    Eq,
    Clone,
    Hash,
    derive_more::Display,
)]
#[serde(rename_all = "camelCase")]
#[display("{}", self.metadata.display_name)]
pub struct ShieldForDisplay {
    pub metadata: SecurityStructureMetadata,
    pub number_of_linked_accounts: usize,
    pub number_of_linked_hidden_accounts: usize,
    pub number_of_linked_personas: usize,
    pub number_of_linked_hidden_personas: usize,
}

impl ShieldForDisplay {
    pub fn new(
        metadata: SecurityStructureMetadata,
        number_of_linked_accounts: usize,
        number_of_linked_hidden_accounts: usize,
        number_of_linked_personas: usize,
        number_of_linked_hidden_personas: usize,
    ) -> Self {
        Self {
            metadata,
            number_of_linked_accounts,
            number_of_linked_hidden_accounts,
            number_of_linked_personas,
            number_of_linked_hidden_personas,
        }
    }

    pub fn with_linked(
        linked_entities: EntitiesLinkedToSecurityStructure,
    ) -> Self {
        Self {
            metadata: linked_entities.metadata,
            number_of_linked_accounts: linked_entities.accounts.len(),
            number_of_linked_hidden_accounts: linked_entities
                .hidden_accounts
                .len(),
            number_of_linked_personas: linked_entities.personas.len(),
            number_of_linked_hidden_personas: linked_entities
                .hidden_personas
                .len(),
        }
    }
}

impl HasSampleValues for ShieldForDisplay {
    fn sample() -> Self {
        Self::new(SecurityStructureMetadata::sample(), 2, 1, 3, 1)
    }

    fn sample_other() -> Self {
        Self::new(SecurityStructureMetadata::sample_other(), 0, 1, 3, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ShieldForDisplay;

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
