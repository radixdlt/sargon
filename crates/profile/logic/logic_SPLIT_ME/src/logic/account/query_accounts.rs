use crate::prelude::*;

pub trait ProfileAccountsOnAllNetworksIncludingHidden {
    fn accounts_on_all_networks_including_hidden(&self) -> Accounts;
}

impl ProfileAccountsOnAllNetworksIncludingHidden for Profile {
    /// Returns **ALL** accounts - including hidden/deleted ones, on **ALL** networks.
    fn accounts_on_all_networks_including_hidden(&self) -> Accounts {
        self.networks
            .iter()
            .flat_map(|n| n.accounts.clone().into_iter())
            .collect::<Accounts>()
    }
}

pub trait ProfileEntitiesOfKindOnNetworkInKeySpace {
    fn get_entities_of_kind_on_network_in_key_space(
        &self,
        entity_kind: CAP26EntityKind,
        network_id: NetworkID,
        key_space: KeySpace,
    ) -> IndexSet<AccountOrPersona>;

    fn get_unsecurified_entities_of_kind_on_network(
        &self,
        entity_kind: CAP26EntityKind,
        network_id: NetworkID,
    ) -> IndexSet<UnsecurifiedEntity> {
        self.get_entities_of_kind_on_network_in_key_space(
            entity_kind,
            network_id,
            // We don't support unhardened paths really. CAP26 dictates all path components are hardened.
            // And all out BIP44 LIKE paths from Olympia are (contrary to BIP44) in fact hardened
            KeySpace::Unsecurified { is_hardened: true },
        )
        .into_iter()
        .map(|e: AccountOrPersona| {
            let factor_instance = match e.security_state() {
                EntitySecurityState::Unsecured { value: uec } => {
                    uec.transaction_signing.clone()
                }
                _ => unreachable!(
                    "Should already have filtered out securified entities"
                ),
            };
            UnsecurifiedEntity::new(e.address(), factor_instance)
        })
        .collect()
    }

    fn unsecurified_accounts_on_network(
        &self,
        network_id: NetworkID,
    ) -> IndexSet<UnsecurifiedEntity> {
        self.get_unsecurified_entities_of_kind_on_network(
            CAP26EntityKind::Account,
            network_id,
        )
    }

    fn get_securified_entities_of_kind_on_network<
        E: IsSecurifiedEntity + HasEntityKind + TryFrom<AccountOrPersona>,
    >(
        &self,
        network_id: NetworkID,
    ) -> IndexSet<E> {
        self.get_entities_of_kind_on_network_in_key_space(
            E::entity_kind(),
            network_id,
            KeySpace::Securified,
        )
        .into_iter()
        .flat_map(|x| E::try_from(x).ok())
        .collect()
    }

    fn securified_accounts_on_network(
        &self,
        network_id: NetworkID,
    ) -> IndexSet<SecurifiedAccount> {
        self.get_securified_entities_of_kind_on_network(network_id)
    }

    fn unsecurified_personas_on_network(
        &self,
        network_id: NetworkID,
    ) -> IndexSet<UnsecurifiedEntity> {
        self.get_unsecurified_entities_of_kind_on_network(
            CAP26EntityKind::Identity,
            network_id,
        )
    }

    fn securified_personas_on_network(
        &self,
        network_id: NetworkID,
    ) -> IndexSet<SecurifiedPersona> {
        self.get_securified_entities_of_kind_on_network(network_id)
    }

    /// Returns the non-hidden personas on the current network, empty if no personas
    /// on the network
    fn personas_on_current_network(&self) -> Result<Personas>;

    /// Returns the hidden personas on the current network, empty if no hidden personas
    /// on the network
    fn hidden_personas_on_current_network(&self) -> Result<Personas>;

    /// Returns **ALL** personas - including hidden/deleted ones, on **ALL** networks.
    fn personas_on_all_networks_including_hidden(&self) -> Personas;
}

impl ProfileEntitiesOfKindOnNetworkInKeySpace for Profile {
    /// Returns the non-hidden personas on the current network, empty if no personas
    /// on the network
    fn personas_on_current_network(&self) -> Result<Personas> {
        self.current_network().map(|n| n.personas.non_hidden())
    }

    /// Returns the hidden personas on the current network, empty if no hidden personas
    /// on the network
    fn hidden_personas_on_current_network(&self) -> Result<Personas> {
        self.current_network().map(|n| n.personas.hidden())
    }

    /// Returns **ALL** personas - including hidden/deleted ones, on **ALL** networks.
    fn personas_on_all_networks_including_hidden(&self) -> Personas {
        self.networks
            .iter()
            .flat_map(|n| n.personas.clone().into_iter())
            .collect::<Personas>()
    }

    fn get_entities_of_kind_on_network_in_key_space(
        &self,
        entity_kind: CAP26EntityKind,
        network_id: NetworkID,
        key_space: KeySpace,
    ) -> IndexSet<AccountOrPersona> {
        self.networks
            .get_id(network_id)
            .map(|n| {
                n.get_entities_of_kind_in_key_space(entity_kind, key_space)
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Profile;

    #[test]
    fn test_accounts_on_current_network() {
        let sut = SUT::sample();
        assert_eq!(
            sut.accounts_on_current_network().unwrap(),
            Accounts::sample_mainnet()
        );
    }

    #[test]
    fn test_accounts_on_current_network_stokenet() {
        let sut = SUT::sample_other();
        assert_eq!(
            sut.accounts_on_current_network().unwrap(),
            Accounts::just(Account::sample_stokenet_nadia()) // olivia is hidden
        );
    }

    #[test]
    fn hidden_accounts_on_current_network() {
        let sut = SUT::sample_other();
        assert_eq!(
            sut.hidden_accounts_on_current_network().unwrap(),
            Accounts::just(Account::sample_stokenet_olivia()) // nadia is visible
        );
    }

    #[test]
    fn test_accounts_for_display_on_current_network() {
        let sut = SUT::sample();
        assert_eq!(
            sut.accounts_for_display_on_current_network().unwrap(),
            Accounts::sample_mainnet()
                .iter()
                .map(AccountForDisplay::from)
                .collect::<AccountsForDisplay>()
        );
    }

    #[test]
    fn test_account_by_address() {
        let sut = SUT::sample();
        assert_eq!(
            sut.account_by_address(Account::sample_mainnet().address),
            Ok(Account::sample_mainnet())
        );
    }
}
