use crate::prelude::*;

impl Profile {
    /// Returns the non-hidden personas on the current network, empty if no personas
    /// on the network
    pub fn personas_on_current_network(&self) -> Result<Personas> {
        self.current_network().map(|n| n.personas.non_hidden())
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
            Personas::just(Persona::sample_stokenet_leia_skywalker())
        );
    }
}
