use crate::prelude::*;

pub trait VisibleOrHidden {
    fn visible(&self) -> Self;
    fn hidden(&self) -> Self;
}

impl VisibleOrHidden for Accounts {
    fn visible(&self) -> Self {
        self.clone()
            .into_iter()
            .filter(|p| !p.is_hidden() && !p.is_tombstoned())
            .collect()
    }

    fn hidden(&self) -> Self {
        self.clone()
            .into_iter()
            .filter(|p| p.is_hidden() && !p.is_tombstoned())
            .collect()
    }
}

pub trait PersonasVisibility {
    fn non_hidden(&self) -> Self;
    fn hidden(&self) -> Self;
}

impl PersonasVisibility for Personas {
    fn non_hidden(&self) -> Self {
        self.clone()
            .into_iter()
            .filter(|p| !p.is_hidden())
            .collect()
    }

    fn hidden(&self) -> Self {
        self.clone().into_iter().filter(|p| p.is_hidden()).collect()
    }
}

pub trait ProfileNetworkEntitiesQuerying {
    fn accounts_non_hidden(&self) -> Accounts;
    fn accounts_hidden(&self) -> Accounts;
    fn personas_non_hidden(&self) -> Personas;
    fn personas_hidden(&self) -> Personas;
    fn get_entities_erased(
        &self,
        entity_kind: CAP26EntityKind,
    ) -> IdentifiedVecOf<AccountOrPersona>;

    fn get_entities_of_kind_in_key_space(
        &self,
        entity_kind: CAP26EntityKind,
        key_space: KeySpace,
    ) -> IdentifiedVecOf<AccountOrPersona> {
        self.get_entities_erased(entity_kind)
            .into_iter()
            .filter(|e| e.matches_key_space(key_space))
            .collect()
    }

    fn entity_by_address(
        &self,
        entity_address: &AddressOfAccountOrPersona,
    ) -> Option<AccountOrPersona> {
        let entities = self
            .get_entities_erased(entity_address.get_entity_kind())
            .into_iter()
            .filter(|e| e.address() == *entity_address)
            .collect_vec();
        assert!(entities.len() <= 1);
        entities.first().cloned()
    }

    fn contains_entity_by_address(
        &self,
        entity_address: &AddressOfAccountOrPersona,
    ) -> bool {
        self.entity_by_address(entity_address).is_some()
    }
}

impl ProfileNetworkEntitiesQuerying for ProfileNetwork {
    fn accounts_non_hidden(&self) -> Accounts {
        self.accounts.visible()
    }

    fn accounts_hidden(&self) -> Accounts {
        self.accounts.hidden()
    }

    fn personas_non_hidden(&self) -> Personas {
        self.personas.non_hidden()
    }

    fn personas_hidden(&self) -> Personas {
        self.personas.hidden()
    }

    fn get_entities_erased(
        &self,
        entity_kind: CAP26EntityKind,
    ) -> IdentifiedVecOf<AccountOrPersona> {
        match entity_kind {
            CAP26EntityKind::Account => self
                .accounts
                .items()
                .into_iter()
                .map(AccountOrPersona::from)
                .collect::<IdentifiedVecOf<_>>(),
            CAP26EntityKind::Identity => self
                .personas
                .items()
                .into_iter()
                .map(AccountOrPersona::from)
                .collect::<IdentifiedVecOf<_>>(),
        }
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
            MFAFactorInstances::new(),
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
            MFAFactorInstances::new(),
        );
        assert_eq!(
            &sut.personas_non_hidden().items(),
            &[Persona::sample_mainnet_batman()]
        )
    }
}
