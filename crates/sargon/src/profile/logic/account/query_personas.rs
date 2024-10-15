use crate::prelude::*;

impl Profile {
    /// Returns the non-hidden personas on the current network, empty if no personas
    /// on the network
    pub fn personas_on_current_network(&self) -> Result<Personas> {
        self.current_network().map(|n| n.personas.non_hidden())
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
mod tests {
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
