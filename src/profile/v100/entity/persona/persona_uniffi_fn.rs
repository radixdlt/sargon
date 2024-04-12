use crate::prelude::*;

#[uniffi::export]
pub fn new_persona_sample() -> Persona {
    Persona::sample()
}

#[uniffi::export]
pub fn new_persona_sample_other() -> Persona {
    Persona::sample_other()
}

#[uniffi::export]
pub fn new_persona_sample_mainnet_batman() -> Persona {
    Persona::sample_mainnet_batman()
}

#[uniffi::export]
pub fn new_persona_sample_mainnet_satoshi() -> Persona {
    Persona::sample_mainnet_satoshi()
}

#[uniffi::export]
pub fn new_persona_sample_stokenet_leia_skywalker() -> Persona {
    Persona::sample_stokenet_leia_skywalker()
}

#[uniffi::export]
pub fn new_persona_sample_stokenet_hermione() -> Persona {
    Persona::sample_stokenet_hermione()
}

#[cfg(test)]
mod uniffi_tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Persona;

    #[test]
    fn samples() {
        assert_eq!(new_persona_sample(), SUT::sample());

        assert_eq!(new_persona_sample_other(), SUT::sample_other());

        assert_eq!(
            new_persona_sample_mainnet_batman(),
            SUT::sample_mainnet_batman()
        );

        assert_eq!(
            new_persona_sample_mainnet_satoshi(),
            SUT::sample_mainnet_satoshi()
        );

        assert_eq!(
            new_persona_sample_stokenet_leia_skywalker(),
            SUT::sample_stokenet_leia_skywalker()
        );

        assert_eq!(
            new_persona_sample_stokenet_hermione(),
            SUT::sample_stokenet_hermione()
        );
    }
}
