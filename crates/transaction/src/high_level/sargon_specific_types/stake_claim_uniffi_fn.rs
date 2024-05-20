use crate::prelude::*;

#[uniffi::export]
pub fn new_stake_claim_sample() -> StakeClaim {
    StakeClaim::sample()
}

#[uniffi::export]
pub fn new_stake_claim_sample_other() -> StakeClaim {
    StakeClaim::sample_other()
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = StakeClaim;

    #[test]
    fn hash_of_sample_values() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_stake_claim_sample(),
                new_stake_claim_sample_other(),
                // duplicates should be removed
                new_stake_claim_sample(),
                new_stake_claim_sample_other(),
            ])
            .len(),
            2
        );
    }
}
