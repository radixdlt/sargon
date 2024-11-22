use crate::prelude::*;

impl Personas {
    pub fn non_hidden(&self) -> Self {
        self.clone()
            .into_iter()
            .filter(|p| !p.is_hidden())
            .collect()
    }
}

impl Profile {
    pub fn unsecurified_personas_on_network(
        &self,
        network_id: NetworkID,
    ) -> IndexSet<UnsecurifiedEntity> {
        self.get_unsecurified_entities_of_kind_on_network(
            CAP26EntityKind::Identity,
            network_id,
        )
    }

    pub fn securified_personas_on_network(
        &self,
        network_id: NetworkID,
    ) -> IndexSet<SecurifiedPersona> {
        self.get_securified_entities_of_kind_on_network(network_id)
    }

    /// Returns the non-hidden personas on the current network, empty if no personas
    /// on the network
    pub fn personas_on_current_network(&self) -> Result<Personas> {
        self.current_network().map(|n| n.personas.non_hidden())
    }

    /// Returns **ALL** personas - including hidden/deleted ones, on **ALL** networks.
    pub fn personas_on_all_networks_including_hidden(&self) -> Personas {
        self.networks
            .iter()
            .flat_map(|n| n.personas.clone().into_iter())
            .collect::<Personas>()
    }

    /// Looks up the persona by identity address, returns Err if the persona is
    /// unknown, will return a hidden persona if queried for.
    pub fn persona_by_address(
        &self,
        address: IdentityAddress,
    ) -> Result<Persona> {
        for network in self.networks.iter() {
            if let Some(persona) = network.personas.get_id(address) {
                return Ok(persona.clone());
            }
        }
        Err(CommonError::UnknownPersona)
    }
}

#[cfg(test)]
mod personas_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Personas;

    #[test]
    fn test_get_non_hidden_none_hidden() {
        let sut = SUT::sample();
        assert_eq!(&sut.non_hidden(), &sut)
    }

    #[test]
    fn test_get_non_hidden_one_hidden() {
        let values =
            &[Persona::sample_mainnet(), Persona::sample_mainnet_turing()];
        let sut = SUT::from_iter(values.clone());

        assert_eq!(sut.non_hidden(), SUT::just(Persona::sample_mainnet()))
    }
}

#[cfg(test)]
mod profile_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Profile;

    #[test]
    fn test_personas_on_current_network() {
        let sut = SUT::sample();
        assert_eq!(
            sut.personas_on_current_network().unwrap(),
            Personas::sample_mainnet()
        );
    }

    #[test]
    fn test_personas_on_current_network_stokenet() {
        let sut = SUT::sample_other();
        assert_eq!(
            sut.personas_on_current_network().unwrap(),
            Personas::just(Persona::sample_stokenet_leia_skywalker()) // Hermione is hidden
        );
    }

    #[test]
    fn test_persona_by_address() {
        let sut = SUT::sample();
        assert_eq!(
            sut.persona_by_address(Persona::sample_mainnet().address),
            Ok(Persona::sample_mainnet())
        );
    }
}
