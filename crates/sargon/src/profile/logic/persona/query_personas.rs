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
}

#[cfg(test)]
mod tests {
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
