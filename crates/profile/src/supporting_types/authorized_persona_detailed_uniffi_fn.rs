use crate::prelude::*;

#[uniffi::export]
pub fn new_authorized_persona_detailed_sample() -> AuthorizedPersonaDetailed {
    AuthorizedPersonaDetailed::sample()
}

#[uniffi::export]
pub fn new_authorized_persona_detailed_sample_other(
) -> AuthorizedPersonaDetailed {
    AuthorizedPersonaDetailed::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AuthorizedPersonaDetailed;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_authorized_persona_detailed_sample(),
                new_authorized_persona_detailed_sample_other(),
                // duplicates should get removed
                new_authorized_persona_detailed_sample(),
                new_authorized_persona_detailed_sample_other(),
            ])
            .len(),
            2
        );
    }
}
