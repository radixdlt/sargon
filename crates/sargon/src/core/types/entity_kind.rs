use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntityKind {
    Account,
    Persona,
}

impl HasSampleValues for EntityKind {
    fn sample() -> Self {
        Self::Account
    }

    fn sample_other() -> Self {
        Self::Persona
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = EntityKind;

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
