use crate::prelude::*;

#[uniffi::export]
pub fn new_account_or_address_of_sample() -> AccountOrAddressOf {
    AccountOrAddressOf::sample()
}

#[uniffi::export]
pub fn new_account_or_address_of_sample_other() -> AccountOrAddressOf {
    AccountOrAddressOf::sample_other()
}

#[uniffi::export]
pub fn account_or_address_of_account_address(
    recipient: &AccountOrAddressOf,
) -> AccountAddress {
    *recipient.account_address()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccountOrAddressOf;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_account_or_address_of_sample(),
                new_account_or_address_of_sample_other(),
                // duplicates should get removed
                new_account_or_address_of_sample(),
                new_account_or_address_of_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn test_account_address() {
        let sut = SUT::sample();
        assert_eq!(
            account_or_address_of_account_address(&sut),
            *sut.account_address()
        )
    }
}
