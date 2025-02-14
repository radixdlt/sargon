use crate::prelude::*;

pub trait GetEntityAddressByAccessControllerAddress {
    fn get_securified_entity_by_access_controller_address(
        &self,
        address: AccessControllerAddress,
    ) -> Result<AnySecurifiedEntity>;
}

impl GetEntityAddressByAccessControllerAddress for Profile {
    fn get_securified_entity_by_access_controller_address(
        &self,
        address: AccessControllerAddress,
    ) -> Result<AnySecurifiedEntity> {
        let network_id = address.network_id();
        self.securified_personas_on_network(network_id)
            .iter()
            .find(|p| {
                p.securified_entity_control.access_controller_address()
                    == address
            })
            .map(AnySecurifiedEntity::from)
            .or(self
                .securified_accounts_on_network(network_id)
                .iter()
                .find(|a| {
                    a.securified_entity_control.access_controller_address()
                        == address
                })
                .map(AnySecurifiedEntity::from))
            .ok_or_else(|| {
                CommonError::NoEntityFoundWithAccessControllerAddress {
                    bad_value: address.to_string(),
                }
            })
    }
}

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
    ) -> IdentifiedVecOf<AccountOrPersona>;

    fn get_unsecurified_entities_of_kind_on_network(
        &self,
        entity_kind: CAP26EntityKind,
        network_id: NetworkID,
    ) -> IdentifiedVecOf<AnyUnsecurifiedEntity> {
        self.get_entities_of_kind_on_network_in_key_space(
            entity_kind,
            network_id,
            // We don't support unhardened paths really. CAP26 dictates all path components are hardened.
            // And all out BIP44 LIKE paths from Olympia are (contrary to BIP44) in fact hardened
            KeySpace::Unsecurified { is_hardened: true },
        )
        .into_iter()
        .filter_map(|e: AccountOrPersona| AnyUnsecurifiedEntity::new(e).ok())
        .collect()
    }

    fn unsecurified_accounts_on_network(
        &self,
        network_id: NetworkID,
    ) -> IdentifiedVecOf<UnsecurifiedAccount> {
        self.get_unsecurified_entities_of_kind_on_network(
            CAP26EntityKind::Account,
            network_id,
        )
        .into_iter()
        .map(|x| UnsecurifiedAccount::try_from(x).unwrap())
        .collect()
    }

    fn get_securified_entities_of_kind_on_network<
        E: IsSecurifiedEntity
            + HasEntityKind
            + TryFrom<AccountOrPersona>
            + Identifiable
            + std::fmt::Debug,
    >(
        &self,
        network_id: NetworkID,
    ) -> IdentifiedVecOf<E> {
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
    ) -> IdentifiedVecOf<SecurifiedAccount> {
        self.get_securified_entities_of_kind_on_network(network_id)
    }

    fn unsecurified_personas_on_network(
        &self,
        network_id: NetworkID,
    ) -> IdentifiedVecOf<UnsecurifiedPersona> {
        self.get_unsecurified_entities_of_kind_on_network(
            CAP26EntityKind::Identity,
            network_id,
        )
        .into_iter()
        .map(|x| UnsecurifiedPersona::try_from(x).unwrap())
        .collect()
    }

    fn securified_personas_on_network(
        &self,
        network_id: NetworkID,
    ) -> IdentifiedVecOf<SecurifiedPersona> {
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
    ) -> IdentifiedVecOf<AccountOrPersona> {
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
