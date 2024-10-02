use crate::prelude::*;

#[uniffi::export]
pub fn new_account_for_display_sample() -> AccountForDisplay {
    AccountForDisplay::sample()
}

#[uniffi::export]
pub fn new_account_for_display_sample_other() -> AccountForDisplay {
    AccountForDisplay::sample_other()
}

#[uniffi::export]
pub fn new_account_for_display_from_account(
    account: Account,
) -> AccountForDisplay {
    AccountForDisplay::from(account)
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

    #[test]
    fn test_new_account_for_display_from_account() {
        let sut = Account::sample();
        assert_eq!(
            new_account_for_display_from_account(sut.clone()),
            AccountForDisplay::from(sut)
        );
    }
}
