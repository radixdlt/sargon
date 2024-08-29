use crate::prelude::*;

#[uniffi::export]
pub fn new_authorized_persona_simple_sample_mainnet() -> AuthorizedPersonaSimple
{
    AuthorizedPersonaSimple::sample_mainnet()
}

#[uniffi::export]
pub fn new_authorized_persona_simple_sample_mainnet_other(
) -> AuthorizedPersonaSimple {
    AuthorizedPersonaSimple::sample_mainnet_other()
}

#[uniffi::export]
pub fn new_authorized_persona_simple_sample_stokenet() -> AuthorizedPersonaSimple
{
    AuthorizedPersonaSimple::sample_stokenet()
}

#[uniffi::export]
pub fn new_authorized_persona_simple_sample_stokenet_other(
) -> AuthorizedPersonaSimple {
    AuthorizedPersonaSimple::sample_stokenet_other()
}

#[cfg(test)]
mod uniffi_tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AuthorizedPersonaSimple;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_authorized_persona_simple_sample_mainnet(),
                new_authorized_persona_simple_sample_mainnet_other(),
                new_authorized_persona_simple_sample_stokenet(),
                new_authorized_persona_simple_sample_stokenet_other(),
                // duplicates should get removed
                new_authorized_persona_simple_sample_mainnet(),
                new_authorized_persona_simple_sample_mainnet_other(),
                new_authorized_persona_simple_sample_stokenet(),
                new_authorized_persona_simple_sample_stokenet_other(),
            ])
            .len(),
            4
        );
    }
}
