use crate::prelude::*;

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

    #[test]
    fn hidden() {
        let values =
            &[Persona::sample_mainnet(), Persona::sample_mainnet_turing()];
        let sut = SUT::from_iter(values.clone());

        assert_eq!(sut.hidden(), SUT::just(Persona::sample_mainnet_turing()))
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
    fn hidden_personas_on_current_network() {
        let sut = SUT::sample_other();
        assert_eq!(
            sut.hidden_personas_on_current_network().unwrap(),
            Personas::just(Persona::sample_stokenet_hermione()) // Leia is visible
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
