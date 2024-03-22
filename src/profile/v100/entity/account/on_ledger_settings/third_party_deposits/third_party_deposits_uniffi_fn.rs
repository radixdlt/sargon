use crate::prelude::*;

#[uniffi::export]
pub fn new_third_party_deposits_sample() -> ThirdPartyDeposits {
    ThirdPartyDeposits::sample()
}

#[uniffi::export]
pub fn new_third_party_deposits_sample_other() -> ThirdPartyDeposits {
    ThirdPartyDeposits::sample_other()
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ThirdPartyDeposits;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_third_party_deposits_sample(),
                new_third_party_deposits_sample_other(),
                // duplicates should get removed
                new_third_party_deposits_sample(),
                new_third_party_deposits_sample_other(),
            ])
            .len(),
            2
        );
    }
}
