use crate::prelude::*;

impl ProfileNetwork {
    pub fn personas_non_hidden(&self) -> Personas {
        self.personas.non_hidden()
    }

    pub fn accounts_non_hidden(&self) -> Accounts {
        self.accounts.visible()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ProfileNetwork;

    #[test]
    fn test_accounts_get_non_hidden_none_hidden() {
        let sut = SUT::sample();
        assert_eq!(&sut.accounts_non_hidden(), &sut.accounts)
    }

    #[test]
    fn test_accounts_get_non_hidden_one_hidden() {
        let values = &[
            Account::sample_mainnet_alice(),
            Account::sample_mainnet_diana(),
        ];
        let accounts = Accounts::from_iter(values.clone());
        let profile_network = SUT::new(
            NetworkID::Mainnet,
            accounts.clone().to_owned(),
            Personas::new(),
            AuthorizedDapps::new(),
            ResourcePreferences::new(),
        );
        assert_eq!(
            &profile_network.accounts_non_hidden().items(),
            &[Account::sample_mainnet_alice()]
        )
    }

    #[test]
    fn test_personas_get_non_hidden_none_hidden() {
        let sut = SUT::sample();
        assert_eq!(&sut.personas_non_hidden(), &sut.personas)
    }

    #[test]
    fn test_personas_get_non_hidden_one_hidden() {
        let values = &[
            Persona::sample_mainnet_batman(),
            Persona::sample_mainnet_turing(),
        ];
        let personas = Personas::from_iter(values.clone());
        let sut = SUT::new(
            NetworkID::Mainnet,
            Accounts::new(),
            personas,
            AuthorizedDapps::new(),
            ResourcePreferences::new(),
        );
        assert_eq!(
            &sut.personas_non_hidden().items(),
            &[Persona::sample_mainnet_batman()]
        )
    }
}
