use crate::prelude::*;

impl IsKeySpaceAware for AccountOrPersona {
    fn key_space(&self) -> KeySpace {
        if self.security_state().is_securified() {
            KeySpace::Securified
        } else if self.is_unsecurified(IsHardened(true)) {
            KeySpace::Unsecurified { is_hardened: true }
        } else if self.is_unsecurified(IsHardened(false)) {
            KeySpace::Unsecurified { is_hardened: false }
        } else {
            unreachable!("should never happen")
        }
    }
}

impl AccountOrPersona {
    pub fn is_unsecurified(&self, is_hardened: IsHardened) -> bool {
        match self.security_state() {
            EntitySecurityState::Unsecured { value: uec } => {
                uec.transaction_signing
                    .derivation_path()
                    .index()
                    .is_hardened()
                    == is_hardened.0
            }
            _ => false,
        }
    }

    pub fn matches_key_space(&self, key_space: KeySpace) -> bool {
        match key_space {
            KeySpace::Securified => self.is_securified(),
            KeySpace::Unsecurified { is_hardened } => {
                self.is_unsecurified(IsHardened(is_hardened))
            }
        }
    }
}

impl ProfileNetwork {
    pub fn personas_non_hidden(&self) -> Personas {
        self.personas.non_hidden()
    }

    pub fn accounts_non_hidden(&self) -> Accounts {
        self.accounts.non_hidden()
    }

    pub fn get_entities_erased(
        &self,
        entity_kind: CAP26EntityKind,
    ) -> IndexSet<AccountOrPersona> {
        match entity_kind {
            CAP26EntityKind::Account => self
                .accounts
                .items()
                .into_iter()
                .map(AccountOrPersona::from)
                .collect::<IndexSet<_>>(),
            CAP26EntityKind::Identity => self
                .personas
                .items()
                .into_iter()
                .map(AccountOrPersona::from)
                .collect::<IndexSet<_>>(),
        }
    }

    pub fn get_entities<E: IsEntity>(&self) -> IndexSet<E> {
        self.get_entities_erased(E::entity_kind())
            .into_iter()
            .map(|e| E::try_from(e).ok().unwrap())
            .collect()
    }

    pub fn get_entities_of_kind_in_key_space(
        &self,
        entity_kind: CAP26EntityKind,
        key_space: KeySpace,
    ) -> IndexSet<AccountOrPersona> {
        self.get_entities_erased(entity_kind)
            .into_iter()
            .filter(|e| e.matches_key_space(key_space))
            .collect()
    }

    pub fn contains_entity_by_address<E: IsEntity>(
        &self,
        entity_address: &E::Address,
    ) -> bool {
        self.get_entities_erased(E::entity_kind())
            .into_iter()
            .any(|e| {
                e.address()
                    == Into::<AddressOfAccountOrPersona>::into(
                        entity_address.clone(),
                    )
            })
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
