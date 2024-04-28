use crate::prelude::*;

#[uniffi::export]
pub fn new_account_for_display_sample() -> AccountForDisplay {
    AccountForDisplay::sample()
}

#[uniffi::export]
pub fn new_account_for_display_sample_other() -> AccountForDisplay {
    AccountForDisplay::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccountForDisplay;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_account_for_display_sample(),
                new_account_for_display_sample_other(),
                // duplicates should get removed
                new_account_for_display_sample(),
                new_account_for_display_sample_other(),
            ])
            .len(),
            2
        );
    }
}
