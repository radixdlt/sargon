use crate::prelude::*;

#[uniffi::export]
pub fn new_mnemonic_sample() -> Mnemonic {
    Mnemonic::sample()
}

#[uniffi::export]
pub fn new_mnemonic_sample_other() -> Mnemonic {
    Mnemonic::sample_other()
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Mnemonic;

    #[test]
    fn hash_of_sample() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_mnemonic_sample(),
                new_mnemonic_sample_other(),
                // duplicates should be removed
                new_mnemonic_sample(),
                new_mnemonic_sample_other(),
            ])
            .len(),
            2
        );
    }
}
