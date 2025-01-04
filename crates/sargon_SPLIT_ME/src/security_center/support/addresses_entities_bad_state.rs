use crate::prelude::*;

/// A struct that represents the addresses of entities in a bad state.
#[derive(Debug, Clone, PartialEq)]
pub struct AddressesOfEntitiesInBadState {
    pub accounts: Vec<AccountAddress>,
    pub hidden_accounts: Vec<AccountAddress>,
    pub personas: Vec<IdentityAddress>,
    pub hidden_personas: Vec<IdentityAddress>,
}

impl AddressesOfEntitiesInBadState {
    pub fn new(
        accounts: impl IntoIterator<Item = AccountAddress>,
        hidden_accounts: impl IntoIterator<Item = AccountAddress>,
        personas: impl IntoIterator<Item = IdentityAddress>,
        hidden_personas: impl IntoIterator<Item = IdentityAddress>,
    ) -> Self {
        Self {
            accounts: Vec::from_iter(accounts),
            hidden_accounts: Vec::from_iter(hidden_accounts),
            personas: Vec::from_iter(personas),
            hidden_personas: Vec::from_iter(hidden_personas),
        }
    }

    pub fn empty() -> Self {
        Self::new([], [], [], [])
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
            accounts: Vec::<_>::sample(),
            hidden_accounts: Vec::new(),
            personas: Vec::<_>::sample(),
            hidden_personas: Vec::new(),
        }
    }

    fn sample_other() -> Self {
        Self {
            accounts: Vec::new(),
            hidden_accounts: Vec::sample_other(),
            personas: Vec::new(),
            hidden_personas: Vec::sample_other(),
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

        let sut = SUT::new([], [], [], Vec::sample());
        assert!(sut.is_empty());
    }
}
