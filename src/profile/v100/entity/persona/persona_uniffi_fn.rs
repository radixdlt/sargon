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
pub fn new_persona_sample_mainnet_ripley() -> Persona {
    Persona::sample_mainnet_ripley()
}

#[uniffi::export]
pub fn new_persona_sample_stokenet_leia_skywalker() -> Persona {
    Persona::sample_stokenet_leia_skywalker()
}

#[uniffi::export]
pub fn new_persona_sample_stokenet_hermione() -> Persona {
    Persona::sample_stokenet_hermione()
}

#[uniffi::export]
pub fn new_persona_sample_stokenet_connor() -> Persona {
    Persona::sample_stokenet_connor()
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

    #[test]
    fn hash_of_sample_values() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_persona_sample_mainnet_batman(),
                new_persona_sample_mainnet_satoshi(),
                new_persona_sample_mainnet_ripley(),
                new_persona_sample_stokenet_hermione(),
                new_persona_sample_stokenet_leia_skywalker(),
                new_persona_sample_stokenet_connor(),
                // duplicates should be removed
                new_persona_sample_mainnet_batman(),
                new_persona_sample_mainnet_satoshi(),
                new_persona_sample_mainnet_ripley(),
                new_persona_sample_stokenet_hermione(),
                new_persona_sample_stokenet_leia_skywalker(),
                new_persona_sample_stokenet_connor(),
            ])
            .len(),
            6
        )
    }
}
