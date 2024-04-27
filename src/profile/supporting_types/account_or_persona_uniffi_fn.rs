use crate::prelude::*;

#[uniffi::export]
pub fn new_account_or_persona_sample() -> AccountOrPersona {
    AccountOrPersona::sample()
}

#[uniffi::export]
pub fn new_account_or_persona_sample_other() -> AccountOrPersona {
    AccountOrPersona::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccountOrPersona;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_account_or_persona_sample(),
                new_account_or_persona_sample_other(),
                // duplicates should get removed
                new_account_or_persona_sample(),
                new_account_or_persona_sample_other(),
            ])
            .len(),
            2
        );
    }
}
