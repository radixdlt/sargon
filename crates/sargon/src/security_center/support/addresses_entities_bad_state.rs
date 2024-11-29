use crate::prelude::*;

/// A struct that represents the addresses of entities in a bad state.
#[derive(Debug, Clone, PartialEq)]
pub struct AddressesOfEntitiesInBadState {
    pub accounts: Accounts,
    pub hidden_accounts: Accounts,
    pub personas: Personas,
    pub hidden_personas: Personas,
}

impl AddressesOfEntitiesInBadState {
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

    pub fn empty() -> Self {
        Self::new(
            Accounts::new(),
            Accounts::new(),
            Personas::new(),
            Personas::new(),
        )
    }

    pub fn is_empty(&self) -> bool {
        self.accounts.is_empty()
            && self.hidden_accounts.is_empty()
            && self.personas.is_empty() // if it only contains hidden_personas, we don't consider it empty
    }
}

impl HasSampleValues for AddressesOfEntitiesInBadState {
    fn sample() -> Self {
        Self {
            accounts: Accounts::sample(),
            hidden_accounts: Accounts::new(),
            personas: Personas::sample(),
            hidden_personas: Personas::new(),
        }
    }

    fn sample_other() -> Self {
        Self {
            accounts: Accounts::new(),
            hidden_accounts: Accounts::sample_other(),
            personas: Personas::new(),
            hidden_personas: Personas::sample_other(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AddressesOfEntitiesInBadState;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn is_empty() {
        let sut = SUT::sample();
        assert!(!sut.is_empty());

        let sut = SUT::empty();
        assert!(sut.is_empty());

        let sut = SUT::new(
            Accounts::new(),
            Accounts::new(),
            Personas::new(),
            Personas::sample(),
        );
        assert!(sut.is_empty());
    }
}
